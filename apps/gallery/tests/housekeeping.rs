//! The housekeeping contract, run as one integration test crate.
//!
//! The rule modules are the single source of truth for how a trashed entry is named, when it
//! expires, where a restored copy goes and what the vault folder guarantees. They live under
//! `apps/gallery/tests/` because the coverage matrix only treats standalone test files as test
//! evidence; `#[cfg(test)]` modules inside the source files are invisible to it.
//!
//! One test stays behind in `src/housekeeping/infrastructure/fs_ops.rs`:
//! `a_short_copy_is_thrown_away_and_the_source_kept` asserts the private `copied_fully` guard, and
//! reaching that fallback from outside the crate would need two test-only seams on a production
//! adapter. Everything else moved here verbatim, keeping its name and its module grouping.

mod support {
    //! Helpers shared by the test modules below.

    use gallery::housekeeping::domain::ports::FileOps;
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    /// A fresh folder under the temp dir, named per test so parallel runs stay apart.
    pub fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir()
            .join(format!("gallery-housekeeping-it-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    /// Records every call; the marker write can be made to fail on purpose.
    pub struct FakeFiles {
        calls: Mutex<Vec<String>>,
        marker_ok: bool,
    }

    impl FakeFiles {
        pub fn new(marker_ok: bool) -> Self {
            Self { calls: Mutex::new(Vec::new()), marker_ok }
        }

        pub fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }
    }

    impl FileOps for FakeFiles {
        fn move_file(&self, from: &Path, to: &Path) -> io::Result<()> {
            self.calls.lock().unwrap().push(format!("move_file {} {}", from.display(), to.display()));
            Ok(())
        }
        fn remove(&self, path: &Path) -> io::Result<()> {
            self.calls.lock().unwrap().push(format!("remove {}", path.display()));
            Ok(())
        }
        fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
            self.calls.lock().unwrap().push(format!("create_dir_all {}", dir.display()));
            Ok(())
        }
        fn write_marker(&self, dir: &Path, name: &str) -> io::Result<()> {
            self.calls.lock().unwrap().push(format!("write_marker {} {}", dir.display(), name));
            if self.marker_ok { Ok(()) } else { Err(io::Error::other("read-only folder")) }
        }
        fn exists(&self, path: &Path) -> bool {
            self.calls.lock().unwrap().push(format!("exists {}", path.display()));
            false
        }
    }
}

mod trash {
    //! What a trashed file is named, when it expires and what a restored copy is called.

    use gallery::housekeeping::domain::trash::{
        DAY_SECS, TRASH_DAYS, TrashEntryName, days_left, is_expired, restore_dest, restored_name,
    };
    use std::path::{Path, PathBuf};

    #[test]
    fn encoding_round_trips_awkward_names() {
        for p in ["/sdcard/DCIM/Camera/IMG_1.jpg", "/a b/100% sure/x%2Fy.png", "/sdcard/Pictures/ünï/été 📷.jpg", "C:\\x\\y.jpg"] {
            let p = PathBuf::from(p);
            let e = TrashEntryName::encode(&p);
            assert!(!e.contains('/') && !e.contains('\\'), "{e}");
            assert_eq!(TrashEntryName::decode(&e).unwrap(), p);
        }
    }

    #[test]
    fn a_trash_entry_name_carries_its_time_and_original_place() {
        let name = TrashEntryName::new(1_700_000_000, Path::new("/sdcard/a.jpg"));
        assert_eq!(name.as_str(), "1700000000_%2Fsdcard%2Fa.jpg");
        assert_eq!(TrashEntryName::parse(name.as_str()), Some((1_700_000_000, PathBuf::from("/sdcard/a.jpg"))));
        assert_eq!(TrashEntryName::parse("junk.txt"), None);
        assert_eq!(TrashEntryName::parse("abc_x.jpg"), None);
    }

    #[test]
    fn retention_boundary_is_exactly_thirty_days() {
        let t = 1_700_000_000;
        assert!(!is_expired(t, t + TRASH_DAYS * DAY_SECS), "exactly 30 days is still here");
        assert!(is_expired(t, t + TRASH_DAYS * DAY_SECS + 1));
        assert!(!is_expired(t + 10, t), "a clock that went backwards must not erase anything");
    }

    #[test]
    fn days_left_keeps_the_rounded_up_label() {
        let t = 1_700_000_000;
        assert_eq!(days_left(t, t), 30, "a fresh entry shows 30 days");
        assert_eq!(days_left(t, t + 29 * DAY_SECS + DAY_SECS / 2), 1, "half a day left still shows 1");
        assert_eq!(days_left(t, t + TRASH_DAYS * DAY_SECS + 1), 0);
    }

    #[test]
    fn restored_names_count_collisions() {
        let original = Path::new("/sdcard/a.jpg");
        assert_eq!(restored_name(original, 1), PathBuf::from("/sdcard/a (restored).jpg"));
        assert_eq!(restored_name(original, 2), PathBuf::from("/sdcard/a (restored 2).jpg"));
        assert_eq!(restore_dest(original, |_| false), PathBuf::from("/sdcard/a.jpg"), "a free original wins");
        assert_eq!(restore_dest(original, |p| p == original), PathBuf::from("/sdcard/a (restored).jpg"));
        assert_eq!(restore_dest(original, |p| p == original || p == Path::new("/sdcard/a (restored).jpg")), PathBuf::from("/sdcard/a (restored 2).jpg"));
    }

    #[test]
    fn broken_names_are_rejected_without_panicking() {
        for name in ["", "no_underscore.jpg", "abc_x.jpg", "1700000000_abc%2", "1700000000_%ZZ", "1700000000_"] {
            assert_eq!(TrashEntryName::parse(name), None, "{name:?} must not parse");
        }
        assert_eq!(TrashEntryName::decode(""), None);
        assert_eq!(TrashEntryName::decode("%2"), None);
    }
}

mod vault {
    //! The dot-folder that holds moved files, and its `.nomedia` marker.

    use super::support::FakeFiles;
    use gallery::housekeeping::domain::ports::FileOps;
    use gallery::housekeeping::domain::vault::Vault;
    use std::io;
    use std::path::Path;

    #[test]
    fn contains_compares_the_parent_not_the_prefix() {
        let vault = Vault::new("/sdcard/.GalleryTrash");
        assert!(vault.contains(Path::new("/sdcard/.GalleryTrash/1_x.jpg")));
        assert!(!vault.contains(Path::new("/sdcard/.GalleryTrash/../DCIM/a.jpg")), "path tricks are out");
        assert!(!vault.contains(Path::new("/sdcard/.GalleryTrash/sub/a.jpg")), "subfolders are out");
        assert!(!vault.contains(Path::new("/sdcard/DCIM/a.jpg")));
        assert!(!vault.contains(Path::new(".GalleryTrash")), "a bare name has no parent");
    }

    #[test]
    fn ensure_survives_a_marker_that_cannot_be_written() {
        let files = FakeFiles::new(false);
        let vault = Vault::new("/sdcard/.GalleryHidden");
        assert!(vault.ensure(&files).is_ok(), "the folder still works without .nomedia");
        assert_eq!(files.calls(), ["create_dir_all /sdcard/.GalleryHidden", "write_marker /sdcard/.GalleryHidden .nomedia"]);
    }

    #[test]
    fn ensure_reports_a_folder_that_cannot_be_created() {
        struct NoDir;
        impl FileOps for NoDir {
            fn move_file(&self, _: &Path, _: &Path) -> io::Result<()> {
                Ok(())
            }
            fn remove(&self, _: &Path) -> io::Result<()> {
                Ok(())
            }
            fn create_dir_all(&self, _: &Path) -> io::Result<()> {
                Err(io::Error::other("denied"))
            }
            fn write_marker(&self, _: &Path, _: &str) -> io::Result<()> {
                Ok(())
            }
            fn exists(&self, _: &Path) -> bool {
                false
            }
        }
        assert!(Vault::new("/sdcard/.GalleryHidden").ensure(&NoDir).is_err());
    }
}

mod ports {
    //! The screen-lock codes, and the desktop adapter that never blocks.

    use gallery::housekeeping::domain::ports::LockResult;

    #[test]
    fn lock_codes_map_to_the_four_outcomes() {
        assert_eq!(LockResult::from_code(0), LockResult::Waiting);
        assert_eq!(LockResult::from_code(1), LockResult::Granted);
        assert_eq!(LockResult::from_code(3), LockResult::NoScreenLock);
        assert_eq!(LockResult::from_code(2), LockResult::Denied);
        assert_eq!(LockResult::from_code(-1), LockResult::Denied);
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn the_desktop_lock_never_blocks() {
        use gallery::housekeeping::domain::ports::VaultLock;
        use gallery::housekeeping::infrastructure::biometric_lock::BiometricLock;

        let lock = BiometricLock;
        assert!(lock.request("Hidden album"));
        assert_eq!(lock.poll(), LockResult::Granted);
    }
}

mod fs_ops {
    //! The real filesystem adapter and its default folders.

    use super::support::temp;
    use gallery::housekeeping::domain::ports::FileOps;
    use gallery::housekeeping::infrastructure::fs_ops::FsOps;
    use std::fs;

    #[test]
    fn moves_create_dirs_and_write_markers() {
        let dir = temp("moves");
        let files = FsOps;
        let src = dir.join("a.jpg");
        fs::write(&src, b"pixels").unwrap();
        files.create_dir_all(&dir.join("album")).unwrap();
        files.move_file(&src, &dir.join("album/a.jpg")).unwrap();
        assert!(!files.exists(&src) && files.exists(&dir.join("album/a.jpg")));

        files.write_marker(&dir.join("album"), ".nomedia").unwrap();
        assert!(files.exists(&dir.join("album/.nomedia")));

        files.remove(&dir.join("album/a.jpg")).unwrap();
        assert!(!files.exists(&dir.join("album/a.jpg")));
        assert!(files.remove(&dir.join("album/a.jpg")).is_err(), "removing twice fails");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn default_folders_are_dot_folders() {
        assert!(FsOps::default_trash_dir().ends_with("gallery-trash") || FsOps::default_trash_dir().ends_with(".GalleryTrash"));
        assert!(FsOps::default_hidden_dir().ends_with("gallery-hidden") || FsOps::default_hidden_dir().ends_with(".GalleryHidden"));
    }
}

mod flow {
    //! Trash round trips driven through the domain rules plus the real filesystem adapter.

    use super::support::temp;
    use gallery::housekeeping::domain::ports::FileOps;
    use gallery::housekeeping::domain::trash::{TrashEntryName, restore_dest};
    use gallery::housekeeping::domain::vault::Vault;
    use gallery::housekeeping::infrastructure::fs_ops::FsOps;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Put `path` into `dir` the way the app does: under a name carrying its original place.
    fn move_in(files: &dyn FileOps, path: &Path, dir: &Path, timestamp: u64) -> PathBuf {
        let vault = Vault::new(dir);
        vault.ensure(files).unwrap();
        let dest = vault.dir.join(TrashEntryName::new(timestamp, path).as_str());
        files.move_file(path, &dest).unwrap();
        dest
    }

    /// Take it back out, never overwriting what already sits at the original path.
    fn move_out(files: &dyn FileOps, path: &Path) -> PathBuf {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap();
        let (_, original) = TrashEntryName::parse(name).unwrap();
        if let Some(parent) = original.parent() {
            files.create_dir_all(parent).unwrap();
        }
        let dest = restore_dest(&original, |p| files.exists(p));
        files.move_file(path, &dest).unwrap();
        dest
    }

    fn original_of(path: &Path) -> PathBuf {
        TrashEntryName::parse(path.file_name().unwrap().to_str().unwrap()).unwrap().1
    }

    #[test]
    fn trash_and_restore_round_trip() {
        let root = temp("rt");
        let trash = Vault::new(root.join(".trash"));
        let photo = root.join("Camera").join("IMG 1.jpg");
        fs::create_dir_all(photo.parent().unwrap()).unwrap();
        fs::write(&photo, b"pixels").unwrap();
        let files = FsOps;

        let t = move_in(&files, &photo, &trash.dir, 1_700_000_000);
        assert!(!photo.exists() && t.exists() && trash.contains(&t));
        assert_eq!(original_of(&t), photo);

        let back = move_out(&files, &t);
        assert_eq!(back, photo);
        assert_eq!(fs::read(&back).unwrap(), b"pixels");
        assert!(!t.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn restore_never_overwrites() {
        let root = temp("ow");
        let trash = Vault::new(root.join(".trash"));
        let photo = root.join("a.jpg");
        fs::write(&photo, b"old").unwrap();
        let files = FsOps;
        let t = move_in(&files, &photo, &trash.dir, 1_700_000_000);
        fs::write(&photo, b"NEW file taken later").unwrap();

        let back = move_out(&files, &t);
        assert_ne!(back, photo);
        assert_eq!(fs::read(&photo).unwrap(), b"NEW file taken later");
        assert_eq!(fs::read(&back).unwrap(), b"old");
        assert!(back.file_name().unwrap().to_string_lossy().contains("(restored)"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn delete_forever_only_inside_trash() {
        let root = temp("df");
        let trash = Vault::new(root.join(".trash"));
        let outside = root.join("precious.jpg");
        fs::write(&outside, b"keep me").unwrap();
        let files = FsOps;
        assert!(trash.remove(&files, &outside).is_err());
        assert!(outside.exists(), "file outside the trash must survive");
        // path tricks must not count as "in trash"
        fs::create_dir_all(&trash.dir).unwrap();
        let sneaky = trash.dir.join("..").join("precious.jpg");
        assert!(trash.remove(&files, &sneaky).is_err());
        assert!(outside.exists());

        let t = move_in(&files, &outside, &trash.dir, 1_700_000_000);
        assert!(trash.remove(&files, &t).is_ok());
        assert!(!t.exists());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn cannot_trash_twice_or_restore_non_trash() {
        let root = temp("tw");
        let trash = Vault::new(root.join(".trash"));
        let photo = root.join("a.jpg");
        fs::write(&photo, b"x").unwrap();
        let files = FsOps;
        let t = move_in(&files, &photo, &trash.dir, 1_700_000_000);
        assert!(trash.contains(&t), "a trashed file is already inside the trash");

        let other = root.join("b.jpg");
        fs::write(&other, b"y").unwrap();
        assert!(!trash.contains(&other), "a plain file is not in the trash");
        assert!(TrashEntryName::parse(other.file_name().unwrap().to_str().unwrap()).is_none(), "and its name carries no original place");
        let _ = fs::remove_dir_all(&root);
    }
}
