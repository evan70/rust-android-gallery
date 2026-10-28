//! `std::fs` behind [`FileOps`], plus the default trash and vault folders.

use crate::housekeeping::domain::ports::FileOps;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The real filesystem.
pub struct FsOps;

impl FsOps {
    pub fn new() -> Self {
        FsOps
    }

    /// Where trashed files live: a dot-folder on the shared storage, a temp folder in the preview.
    pub fn default_trash_dir() -> PathBuf {
        if cfg!(target_os = "android") {
            PathBuf::from("/sdcard/.GalleryTrash")
        } else {
            std::env::temp_dir().join("gallery-trash")
        }
    }

    /// Where hidden files live: a dot-folder with a `.nomedia` marker.
    pub fn default_hidden_dir() -> PathBuf {
        if cfg!(target_os = "android") {
            PathBuf::from("/sdcard/.GalleryHidden")
        } else {
            std::env::temp_dir().join("gallery-hidden")
        }
    }
}

impl Default for FsOps {
    fn default() -> Self {
        Self::new()
    }
}

/// A copy is only accepted when every byte arrived; a short one is removed again.
fn copied_fully(from: &Path, to: &Path, copied: u64, len: u64) -> io::Result<()> {
    if copied != len {
        let _ = fs::remove_file(to);
        return Err(io::Error::other(format!("incomplete copy of {}", from.display())));
    }
    Ok(())
}

impl FileOps for FsOps {
    fn move_file(&self, from: &Path, to: &Path) -> io::Result<()> {
        if fs::rename(from, to).is_ok() {
            return Ok(());
        }
        // different filesystem: copy, then remove the source only if the copy is complete
        let copied = fs::copy(from, to)?;
        copied_fully(from, to, copied, fs::metadata(from)?.len())?;
        fs::remove_file(from)
    }

    fn remove(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    fn create_dir_all(&self, dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dir)
    }

    fn write_marker(&self, dir: &Path, name: &str) -> io::Result<()> {
        fs::write(dir.join(name), b"")
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

#[cfg(test)]
mod tests {
    //! Only the copy guard is tested here; the rest of the housekeeping contract lives in
    //! `apps/gallery/tests/housekeeping.rs`.

    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("gallery-fsops-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_short_copy_is_thrown_away_and_the_source_kept() {
        let dir = temp("shortcopy");
        let from = dir.join("from.jpg");
        let to = dir.join("to.jpg");
        fs::write(&from, b"0123456789").unwrap();
        fs::write(&to, b"0123").unwrap(); // what a failed copy leaves behind

        let err = copied_fully(&from, &to, 4, 10);
        assert!(err.is_err(), "a short copy is an error");
        assert!(!to.exists(), "the partial copy is removed again");
        assert_eq!(fs::read(&from).unwrap(), b"0123456789", "the source is untouched");
        let _ = fs::remove_dir_all(&dir);
    }
}
