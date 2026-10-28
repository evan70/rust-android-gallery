//! Vault rules: the dot-folder that holds moved files (the trash and the hidden album), and the
//! `.nomedia` marker that keeps every other gallery out of it.

use super::ports::FileOps;
use std::io;
use std::path::{Path, PathBuf};

/// The marker Android's media scanner honours: a folder holding it is not indexed.
pub const NO_MEDIA: &str = ".nomedia";

/// A folder that holds files moved out of the library.
pub struct Vault {
    pub dir: PathBuf,
}

impl Vault {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Create the folder and its marker. A failing marker write is not an error: the folder still
    /// works, it is only noisier in other galleries.
    pub fn ensure(&self, files: &dyn FileOps) -> io::Result<()> {
        files.create_dir_all(&self.dir)?;
        let _ = files.write_marker(&self.dir, NO_MEDIA);
        Ok(())
    }

    /// Whether `path` sits directly in this folder. The parent is compared, so `vault/../file`
    /// and `vault/sub/file` are both outside.
    pub fn contains(&self, path: &Path) -> bool {
        path.parent() == Some(self.dir.as_path())
    }

    /// Erase `path`, but only when it sits directly in this folder.
    pub fn remove(&self, files: &dyn FileOps, path: &Path) -> io::Result<()> {
        if !self.contains(path) {
            return Err(io::Error::other("refusing to delete a file that is not in this folder"));
        }
        files.remove(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Records every call; the marker write can be made to fail on purpose.
    struct FakeFiles {
        calls: Mutex<Vec<String>>,
        marker_ok: bool,
    }

    impl FakeFiles {
        fn new(marker_ok: bool) -> Self {
            Self { calls: Mutex::new(Vec::new()), marker_ok }
        }
        fn calls(&self) -> Vec<String> {
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
