//! Trash rules: what a trashed file is named, when it expires, and what a restored copy is called.
//!
//! A trashed file is named `<unix-seconds>_<percent-encoded original path>`, so its original
//! place survives the move without a separate index. Nothing here touches a disk: every rule is
//! decided from strings, paths and times.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Trashed items are erased for good after this many days.
pub const TRASH_DAYS: u64 = 30;
/// Seconds in a day, used with [`TRASH_DAYS`].
pub const DAY_SECS: u64 = 86_400;

/// The file name of one trashed entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrashEntryName(String);

impl TrashEntryName {
    /// The name a file gets when it goes into the trash at `timestamp`.
    pub fn new(timestamp: u64, original: &Path) -> Self {
        Self(format!("{timestamp}_{}", Self::encode(original)))
    }

    /// The name as written on disk.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// When a file went into the trash (unix seconds) and where it came from, if its name follows
    /// our scheme. Foreign names in the folder are ignored.
    pub fn parse(file_name: &str) -> Option<(u64, PathBuf)> {
        let (seconds, encoded) = file_name.split_once('_')?;
        let timestamp: u64 = seconds.parse().ok()?;
        Self::decode(encoded).map(|original| (timestamp, original))
    }

    /// Percent-encode a path into something that can be one file name on any filesystem.
    pub fn encode(path: &Path) -> String {
        let mut out = String::new();
        for b in path.to_string_lossy().bytes() {
            match b {
                b'/' | b'%' | b'\\' | b':' => out.push_str(&format!("%{b:02X}")),
                0x20..=0x7E => out.push(b as char),
                _ => out.push_str(&format!("%{b:02X}")),
            }
        }
        out
    }

    /// Undo [`TrashEntryName::encode`]; `None` when the escapes are broken or empty.
    pub fn decode(encoded: &str) -> Option<PathBuf> {
        let bytes = encoded.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' {
                let hex = encoded.get(i + 1..i + 3)?;
                out.push(u8::from_str_radix(hex, 16).ok()?);
                i += 3;
            } else {
                out.push(bytes[i]);
                i += 1;
            }
        }
        if out.is_empty() {
            return None;
        }
        String::from_utf8(out).ok().map(PathBuf::from)
    }
}

/// Whether an entry put in the trash at `trashed_at` is older than [`TRASH_DAYS`] at `now`.
/// Exactly 30 days is not expired yet; a second more is.
pub fn is_expired(trashed_at: u64, now: u64) -> bool {
    now.saturating_sub(trashed_at) > TRASH_DAYS * DAY_SECS
}

/// Whole days an entry has left before it is erased (`0..=TRASH_DAYS`), rounded up like the
/// viewer's label always did.
pub fn days_left(trashed_at: u64, now: u64) -> u64 {
    (TRASH_DAYS * DAY_SECS).saturating_sub(now.saturating_sub(trashed_at)).div_ceil(DAY_SECS)
}

/// The name of the `attempt`-th restored copy: `a.jpg`, `a (restored).jpg`, `a (restored 2).jpg`.
pub fn restored_name(original: &Path, attempt: u32) -> PathBuf {
    let stem = original.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let ext = original.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let tag = if attempt <= 1 { " (restored)".to_string() } else { format!(" (restored {attempt})") };
    original.with_file_name(format!("{stem}{tag}{ext}"))
}

/// Where a restored file goes: its original path, or a `(restored)` copy when something is there.
/// `taken` answers whether a path is already occupied, so the rule stays testable without a disk.
pub fn restore_dest(original: &Path, mut taken: impl FnMut(&Path) -> bool) -> PathBuf {
    let mut dest = original.to_path_buf();
    let mut attempt = 0;
    while taken(&dest) {
        attempt += 1;
        dest = restored_name(original, attempt);
    }
    dest
}

/// The current time in unix seconds; the clock is the only thing this module reads.
pub fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

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
