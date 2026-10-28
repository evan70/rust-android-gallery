//! The phone's fingerprint / screen-lock prompt behind [`VaultLock`].
//!
//! The desktop preview has no screen lock to ask for, so its lock grants access right away. On
//! Android the activity handle only exists once `Gallery::with_android` has it, so the adapter
//! holds it in a slot that starts empty and answers "denied" until then.

use crate::housekeeping::domain::ports::{LockResult, VaultLock};

#[cfg(target_os = "android")]
use crate::perm;
#[cfg(target_os = "android")]
use android_activity::AndroidApp;
#[cfg(target_os = "android")]
use std::sync::Mutex;

/// Android: asks `perm` for the system confirmation, through the activity handle it was given.
#[cfg(target_os = "android")]
pub struct BiometricLock {
    app: Mutex<Option<AndroidApp>>,
}

#[cfg(target_os = "android")]
impl BiometricLock {
    pub fn new() -> Self {
        Self { app: Mutex::new(None) }
    }

    /// Hand over the activity handle; called from `Gallery::with_android` before the first frame.
    pub fn attach(&self, app: AndroidApp) {
        *self.app.lock().unwrap() = Some(app);
    }
}

#[cfg(target_os = "android")]
impl Default for BiometricLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "android")]
impl VaultLock for BiometricLock {
    fn request(&self, reason: &str) -> bool {
        match self.app.lock().unwrap().as_ref() {
            Some(app) => {
                perm::authenticate(app, reason);
                true
            }
            None => false,
        }
    }

    fn poll(&self) -> LockResult {
        match self.app.lock().unwrap().as_ref() {
            Some(app) => LockResult::from_code(perm::take_auth(app)),
            None => LockResult::Denied,
        }
    }
}

/// Desktop: nothing to confirm, so the "prompt" is granted as soon as it is asked for.
#[cfg(not(target_os = "android"))]
pub struct BiometricLock;

#[cfg(not(target_os = "android"))]
impl BiometricLock {
    pub fn new() -> Self {
        BiometricLock
    }
}

#[cfg(not(target_os = "android"))]
impl Default for BiometricLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_os = "android"))]
impl VaultLock for BiometricLock {
    fn request(&self, _reason: &str) -> bool {
        true
    }

    fn poll(&self) -> LockResult {
        LockResult::Granted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "android"))]
    #[test]
    fn the_desktop_lock_never_blocks() {
        let lock = BiometricLock;
        assert!(lock.request("Hidden album"));
        assert_eq!(lock.poll(), LockResult::Granted);
    }
}
