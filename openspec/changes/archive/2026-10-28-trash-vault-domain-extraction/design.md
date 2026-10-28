# Design: Extract trash and vault rules into the housekeeping context

## Technical Approach

Split the current `trash.rs` along its natural seam: everything that can be decided from strings,
times and paths becomes domain code; everything that touches the filesystem or the Android lock
becomes an adapter behind a port.

```
apps/gallery/src/housekeeping/
├── domain/
│   ├── mod.rs
│   ├── trash.rs       TrashEntryName (format/parse), retention predicate, restore-collision naming
│   ├── vault.rs       vault folder rules: directory + .nomedia marker, hide/unhide paths
│   └── ports.rs       FileOps, VaultLock traits
└── infrastructure/
    ├── mod.rs
    ├── fs_ops.rs      std::fs adapter: move (rename → copy+verify+remove), remove, create_dir,
    │                  write_marker, move_file
    └── biometric_lock.rs  Android perm::authenticate / perm::take_auth; desktop auto-unlock
```

Domain surface (behavior-preserving, signatures chosen to be testable without I/O):

```rust
// housekeeping/domain/trash.rs
pub const TRASH_DAYS: u64 = 30;
pub const DAY_SECS: u64 = 86_400;

pub struct TrashEntryName(String);          // "<unix-seconds>_<percent-encoded original path>"
impl TrashEntryName {
    pub fn new(timestamp: u64, original: &Path) -> Self;
    pub fn parse(file_name: &str) -> Option<(u64, PathBuf)>;   // replaces trashed_at + original_of
    pub fn encode(path: &Path) -> String;                      // was private encode()
    pub fn decode(encoded: &str) -> Option<PathBuf>;           // was private decode()
}

pub fn is_expired(trashed_at: u64, now: u64) -> bool;          // now - trashed_at > TRASH_DAYS * DAY_SECS
pub fn days_left(trashed_at: u64, now: u64) -> u64;            // 0..=30, keeps the viewer's div_ceil rounding
pub fn restored_name(original: &Path, attempt: u32) -> PathBuf; // "a.jpg" → "a (restored).jpg"
```

```rust
// housekeeping/domain/vault.rs
pub struct Vault { pub dir: PathBuf }
impl Vault {
    pub fn ensure(&self, files: &dyn FileOps) -> std::io::Result<()>; // create dir + .nomedia (best effort)
    pub fn contains(&self, path: &Path) -> bool;                      // parent == dir
}
```

```rust
// housekeeping/domain/ports.rs
pub trait FileOps: Send + Sync {
    fn move_file(&self, from: &Path, to: &Path) -> std::io::Result<()>;   // rename, else copy+verify+remove
    fn remove(&self, path: &Path) -> std::io::Result<()>;
    fn create_dir_all(&self, dir: &Path) -> std::io::Result<()>;
    fn write_marker(&self, dir: &Path, name: &str) -> std::io::Result<()>;
    fn exists(&self, path: &Path) -> bool;
}

pub trait VaultLock: Send + Sync {
    /// Ask the system for confirmation; the answer arrives later via `poll`.
    fn request(&self, reason: &str) -> bool;
    /// 0 = still waiting, 1 = granted, 3 = no screen lock configured, other = denied.
    fn poll(&self) -> LockResult;
}
```

Wiring stays in one place, but **not in one function**: `Gallery::new` (`lib.rs:595`) builds
`Arc<dyn FileOps>` (`FsOps`) and the desktop `Arc<dyn VaultLock>` (always granted), and moves the
`trash::default_trash_dir()` / `trash::default_hidden_dir()` calls at `lib.rs:612-613` into the
adapter. The Android `AndroidApp` handle does **not** exist yet at that point — `new()` stores
`android: None` (`lib.rs:724`) and the handle arrives later in `Gallery::with_android`
(`lib.rs:569-579`). The Android lock is therefore one `Arc<dyn VaultLock>` created in `new()` that
holds `Mutex<Option<AndroidApp>>`; `with_android` fills that slot before the first frame, and
`request`/`poll` return "denied" until the handle is present. `request`/`poll` take `&self` so the
interior mutability lives in the adapter, not in `Gallery`. The UI blocks in `hidden.rs` / `lib.rs` keep working through those handles; in particular
`request_unlock` and `poll_auth` call the port instead of `perm::authenticate` / `perm::take_auth`,
while `update_secure` keeps calling `perm::set_secure` directly (screenshot blocking is not a lock
prompt).

The `housekeeping` module tree (`housekeeping/mod.rs`, `domain/mod.rs`, `infrastructure/mod.rs`, plus
`mod housekeeping;` in `lib.rs`) has to exist before any of these files compile.

`#[cfg(target_os = "android")]` moves out of the domain entirely: the desktop/Android choice happens
inside `biometric_lock.rs` (and in `FileOps` only for `default_*_dir()`), so `domain/` compiles the
same everywhere.

## Data / Artifact Model

- Trashed entry file name: `<unix-seconds>_<percent-encoded original path>`; percent-encoding covers
  `/`, `%`, `\`, `:`, any byte outside `0x20..=0x7E`, and every non-UTF-8 byte. Unchanged.
- Vault folder: a dot-folder (`/sdcard/.GalleryHidden` on Android, temp dir on desktop) containing an
  empty `.nomedia`. Created before the first item is moved; the marker write stays best-effort.
- No new persisted state, no schema change, no cache format change. `thumbs/` keys remain
  `mtime`/`size`/`path` based; hiding an item still purges its cached thumbnail.
- Restore naming: first collision `stem + " (restored)" + ext`, later ones
  `stem + " (restored n)" + ext`; the counter restarts per restore attempt, exactly as today.

## Integration Points

| Call site | Current | After |
|---|---|---|
| `lib.rs:109-125` `scan` trash/vault branch | `trash::original_of` (line 111) | `TrashEntryName::parse(...)` |
| `lib.rs:1030-1031` expiry step in `apply_scan` | inline `now - t > TRASH_DAYS * 86_400`, then `delete_forever` + `gone = true` | `domain::trash::is_expired(t, now)` |
| `lib.rs:1248` `trash_items`, `lib.rs:1350` `trash_item` | `trash::move_to_trash` | `TrashEntryName::new` + `FileOps::move_file` |
| `lib.rs:1276` `restore_items`, `lib.rs:1371` `restore_item` | `trash::restore` | `TrashEntryName::parse` + `restored_name` + `FileOps::exists`/`move_file` |
| `lib.rs:1300` `delete_forever_items`, `1399` `delete_forever_item`, `1418` `empty_trash` | `trash::delete_forever` | domain containment check + `FileOps::remove` |
| `hidden.rs:86-157` `hide_items` / `unhide_items` | inline folder + `.nomedia` (93-98), `trash::move_to_trash` (107), `trash::restore` (142) | `Vault::ensure`, `TrashEntryName`, `FileOps` |
| `albumops.rs:40` generic move | `trash::move_file` | `FileOps::move_file` |
| `viewer.rs:319` trash age | `trash::trashed_at` | `TrashEntryName::parse` |
| `viewer.rs:321` days-left label | inline `(TRASH_DAYS * 86_400).saturating_sub(…).div_ceil(86_400)` | `days_left(trashed_at, now)` |
| `lib.rs:46` constant | `const TRASH_DAYS: u64 = 30` | removed; `housekeeping::domain::trash::TRASH_DAYS` |
| `hidden.rs` unlock | `perm::authenticate` / `perm::take_auth` on Android | `VaultLock::request` / `poll` |

## Alternatives Considered

1. **Keep free functions, only extract the codec.** Smallest diff, but retention and restore
   collision rules stay untestable without a disk and the vault invariants stay in the UI — rejected
   because it does not remove the risk this change targets.
2. **Full slice extraction (`slices/trash/*_screen.rs`).** Would move the `impl Gallery` blocks too;
   ~700 lines of UI churn in `lib.rs`/`hidden.rs` for no behavioral gain. Deferred to a later change
   per the legacy-vs-new policy.
3. **Generic ports (`impl FileOps` monomorphised) instead of `dyn`.** Faster, but `Gallery` would
   become generic over two traits across every `impl Gallery` block. `Arc<dyn _>` matches the
   existing single composition root and costs one vtable hop per file operation.

## Risks

- **Retention boundary drift**: the current check is `>` (more than 30 days). `>=` would erase a
  day earlier. Pinned by a test at exactly 30 days and at 30 days + 1 second.
- **Path-trick safety**: `delete_forever` currently compares `path.parent()` with the trash dir, which
  rejects `trash/../file`. The extracted check must keep parent comparison (not `starts_with`).
- **Cross-filesystem move**: the copy fallback verifies the copied length before removing the source;
  a naive `fs::rename(...).or_else(copy)` rewrite in the adapter could delete a source after a short
  copy. Preserved verbatim and covered by the existing test.
- **Marker write failure**: `let _ = fs::write(.nomedia)` is intentional. The port method signature
  returns `io::Result`, and `Vault::ensure` ignores that result.
- **Locked-vault leakage**: `lock_hidden` must still purge thumbnails/durations for vault items; the
  refactor must not move that purge behind the port or it would run while locked.
- **Android handle timing**: the `AndroidApp` is only available from `Gallery::with_android`, not
  from `Gallery::new`. Constructing the lock with the app in `new()` would not compile; the adapter
  must accept the handle through a `Mutex<Option<AndroidApp>>` slot filled by `with_android`, and
  must behave as "locked" until then.
- **Platform leakage**: any `jni`/`ndk` reference appearing in `housekeeping/domain/` fails review
  even if it compiles, because it breaks the desktop test build.
