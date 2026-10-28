//! Housekeeping: the trash and the hidden vault.
//!
//! `domain` decides what a trashed file is called, when it expires, and where a restored copy
//! goes; `infrastructure` is the only place that touches the disk or the phone's screen lock.

pub mod domain;
pub mod infrastructure;

#[cfg(test)]
mod flow_tests {
    //! The trash round trips that used to live in `trash.rs`, driven through the domain rules
    //! plus the real filesystem adapter.

    use super::domain::ports::FileOps;
    use super::domain::trash::{TrashEntryName, restore_dest};
    use super::domain::vault::Vault;
    use super::infrastructure::fs_ops::FsOps;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("gallery-housekeeping-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

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
