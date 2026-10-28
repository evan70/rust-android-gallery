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
