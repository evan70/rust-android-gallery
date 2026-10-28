//! What the trash and the vault need from the outside world: the filesystem and the phone's
//! screen lock. Both are traits so the rules and the app can be checked without either.

use std::io;
use std::path::Path;

/// File operations the trash and the vault rely on.
pub trait FileOps: Send + Sync {
    /// Move a file, falling back to copy-then-remove when the two ends are on different filesystems.
    fn move_file(&self, from: &Path, to: &Path) -> io::Result<()>;
    /// Erase one file.
    fn remove(&self, path: &Path) -> io::Result<()>;
    fn create_dir_all(&self, dir: &Path) -> io::Result<()>;
    /// Write an empty marker file into `dir` (the `.nomedia` that keeps galleries out).
    fn write_marker(&self, dir: &Path, name: &str) -> io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
}

/// Answer from the phone's screen-lock prompt.
///
/// The desktop build only ever grants, so three of the four outcomes are constructed by the
/// Android adapter alone; that is not dead code, it is conditional code.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LockResult {
    /// The prompt is still up.
    Waiting,
    /// The user confirmed it.
    Granted,
    /// No PIN, pattern or fingerprint is set on the phone.
    NoScreenLock,
    /// The prompt was dismissed or failed.
    Denied,
}

impl LockResult {
    /// The platform's status code: 0 waiting, 1 granted, 3 no screen lock, anything else denied.
    #[cfg_attr(not(target_os = "android"), allow(dead_code))]
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => Self::Waiting,
            1 => Self::Granted,
            3 => Self::NoScreenLock,
            _ => Self::Denied,
        }
    }
}

/// The phone's confirmation prompt, behind a port so the app also runs without one.
pub trait VaultLock: Send + Sync {
    /// Ask the system to confirm the user. `true` means a prompt is up and [`VaultLock::poll`]
    /// will answer it; `false` means the answer will never come.
    fn request(&self, reason: &str) -> bool;
    /// 0 = still waiting, 1 = granted, 3 = no screen lock configured, other = denied.
    fn poll(&self) -> LockResult;
}
