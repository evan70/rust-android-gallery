# Architecture: Explicit Architecture (Flat Vertical Slice)

## Overview

The gallery is organised as a set of bounded contexts (library, media, housekeeping, shell), each
split into a small, framework-free **domain** (entities, value objects, and the ports/ traits that
describe what the context needs from the outside world) and **slices** — one flat folder per entity
or cross-entity process, holding the screen, the use-case service and the adapter side by side.
Dependencies point inward: presentation and infrastructure may depend on the domain, never the other
way round.

This pattern was chosen because the app has real domain rules that must stay testable on a PC
(trash retention and name encoding, vault paths, thumbnail cache keys, date grouping, EXIF parsing)
while the platform underneath (Android JNI/NDK, filesystem layout, GL) is noisy and only available
on a device. Ports let the domain and the use-case services be exercised with in-memory fakes
instead of a phone, which is exactly what the existing `#[cfg(test)]` modules already try to do.
Flat slices keep each feature readable in one place: for a single-crate app with no ORM and no HTTP
layer, nested `Application/Infrastructure/Presentation` folders would add depth without adding
information.

The domain model follows Domain-Driven Design where this app actually has rules: bounded contexts
with their own language, a few aggregates whose invariants are enforced in code, and value objects
for derived identities. DDD's tactical ceremony that exists to manage a relational store — ORM
repositories, unit of work, domain events, CQRS — is deliberately left out; see
[Domain Model (DDD)](#domain-model-ddd).

## Decision Rationale

- **Project type:** native Android photo/video gallery with a desktop preview binary; single Cargo
  workspace member, no server, no database.
- **Tech stack:** Rust 2024, eframe/egui 0.37 (glow), android-activity + jni + ndk, `image`.
- **Key factor:** hostile platform dependencies (JNI, NDK, `std::fs`, egui) must not leak into the
  rules that decide what the gallery shows and what gets deleted. Ports keep those rules pure and
  unit-testable; flat vertical slices keep the feature code local.
- **Team size:** small (1-5 developers), so the pattern is kept as light as possible: no DI
  container, no CQRS, no event bus — wiring happens once in the composition root.

## Domain Model (DDD)

### Ubiquitous language

Use these words in code, commit messages and docs. A synonym is a bug in the model.

| Term | Meaning in this project | Current code |
|---|---|---|
| Media item | One photo or video file the gallery can show | `Item` (`lib.rs`) |
| Album | A folder of media items shown as one screen | `Album`, `albumops.rs` |
| Built-in album | Pseudo-album (All, Videos, Favourites, Trash, Hidden, External) keyed by a control character so it can never equal a folder path | `ALL_KEY`, `VIDEOS_KEY`, `FAVS_KEY`, `TRASH_KEY`, `HIDDEN_KEY`, `EXT_KEY` |
| Pin | A user-chosen album kept at the top of the Albums tab | `pins`, `pins_loaded` |
| Trash | Folder that holds "deleted" items for the restore window | `TRASH_KEY`, `trash_dir`, `housekeeping/domain/trash.rs` |
| Trash entry | A file named `<unix-seconds>_<percent-encoded original path>` so it can be restored | `TrashEntryName::encode` / `decode` / `parse` |
| Vault / hidden album | Private dot-folder with a `.nomedia` marker, listed only after unlock | `hidden_dir`, `hidden.rs` |
| Unlock | Biometric / screen-lock confirmation before the vault is listed | `AuthHelper`, `hidden_unlocked` |
| Thumbnail key | Identity of a cached thumbnail: FNV-1a over path + size + mtime (seconds) | `thumbcache::key` |
| Scan | Walking the roots and classifying what is a media item | `scan`, `start_scan` |
| Free up space | Review of large items with delete/trash actions | `space.rs` |

### Aggregates and invariants

Each aggregate owns its invariants; services orchestrate, they do not re-implement these rules.

- **Trash** (`housekeeping/domain/trash.rs`)
  - Every entry name is `<unix-seconds>_<percent-encoded original path>`; `/`, `%`, `\`, `:` and
    non-printable bytes are percent-encoded.
  - Restore is only possible when the name decodes back to a path (`TrashEntryName::parse`); a file whose
    name does not follow the scheme is not restorable and must be reported, not guessed.
  - Retention is `TRASH_DAYS = 30`; expiry is derived from `trashed_at`, never stored separately.
  - Moving to the trash returns the new location, which is what makes the undo snackbar possible.
- **Vault** (`housekeeping/domain/vault.rs`)
  - Lives in a dot-folder containing `.nomedia`, so no other gallery or media picker lists it.
  - Listing requires an unlock; locking again must not erase the items or their `orig` path.
  - Hide/unhide preserves the original location so the item can go back where it came from.
- **Album** (`library/domain/album.rs`)
  - A real album is a folder path; a built-in album is a key starting with `\u{1}`. The two
    namespaces must never collide.
  - Pins reference album keys, so a renamed folder may drop its pin — that is acceptable, a pin to
    a non-existent key is not.
- **Media item identity** (`library/domain/item.rs`)
  - Identity is the file path within a scan; `mtime` and `size` describe the version.
  - A permanently deleted item stays in the session with `gone = true` so view indices remain
    stable — indices are positions, not identities.
- **Value objects**
  - `ThumbnailKey` — derived from path, size and mtime, so a replaced file gets a new entry.
  - `Exif` — parsed once, with its display rules (`camera()`, `settings_line()`) inside the type.
  - `DayKey` — `yyyymmdd` in local time, used for date headings and grouping.

### Not domain

- `gridlayout` is presentation geometry (row heights, cell sizes, month labels). It is pure and
testable, but it belongs to `library/slices/photo/`, not to `domain/`.
- Paths and permission strings (`perm.rs`, `intents.rs`) are platform facts, not domain rules.

### Deliberately omitted tactical patterns

| Pattern you may know from PHP DDD | Why it is not here |
|---|---|
| Repository per aggregate over an ORM | The store is the filesystem. A "repository" is a thin port over `read_dir` / `rename`; there is nothing to abstract beyond that. |
| Unit of Work, identity map, lazy loading | No transactions and no relational identity. A scan is a snapshot taken on a background thread. |
| Domain events and a dispatcher | The only candidate (undo after a move) is a 7-second UI affordance, not a business reaction. |
| CQRS and read models | Every screen reads the same in-memory `Vec<Item>` plus a view index; a separate read model would duplicate it. |
| Aggregate factories / builders for every entity | Constructors plus `Default` are enough at this size. |
| DI container | One composition root (`Gallery::new`, `main.rs`) wires ports to adapters. |

Revisit this table only with evidence: a second persistence backend, background sync, or more than
one writer to the library would justify domain events or a read model.

## Folder Structure

Target structure for the `gallery` crate. Existing files move into it incrementally (see
[Legacy vs New Code Policy](#legacy-vs-new-code-policy)).

```
apps/gallery/src/
├── lib.rs                          # COMPOSITION ROOT: wires ports to adapters, owns the app loop
├── main.rs                         # desktop binary: builds the same root with desktop adapters
│
├── library/                        # ── CONTEXT: what media exists ──
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── item.rs                 # Item, MediaKind, day key, sort rules
│   │   ├── album.rs                # Album, built-in album keys, pins
│   │   └── ports.rs                # ItemSource, AlbumStore (traits only)
│   ├── slices/
│   │   ├── photo/                  # entity slice: the date-grouped grid
│   │   │   ├── photo_screen.rs     # egui presentation (grid, selection, headings)
│   │   │   ├── photo_service.rs    # use cases: load page, select, favourite
│   │   │   └── photo_repository.rs # adapters used by this slice (scan + favourites file)
│   │   ├── album/                  # entity slice: albums, rename/delete, move/copy
│   │   └── scan/                   # process slice: walk roots, classify, publish results
│   └── infrastructure/
│       ├── fs_source.rs            # ItemSource over the filesystem
│       └── favourites_file.rs      # favourites.txt adapter
│
├── media/                          # ── CONTEXT: pixels and playback ──
│   ├── domain/
│   │   ├── thumbnail.rs            # Thumbnail, ThumbnailKey (hash of path+mtime+size)
│   │   ├── playback.rs             # PlaybackState, PlayerCommand
│   │   └── ports.rs                # Thumbnailer, Player, ThumbnailCache
│   ├── slices/
│   │   ├── thumbnail/              # thumbnail_service (queueing, budget) + screen widget
│   │   └── video/                  # video player screen + service
│   └── infrastructure/
│       ├── android/                # JNI adapters: MediaMetadataRetriever, MediaPlayer, SurfaceTexture
│       ├── desktop/                # desktop stubs
│       └── cache_fs.rs             # thumbs/ JPEG cache adapter
│
├── housekeeping/                   # ── CONTEXT: destructive and private operations ──
│   ├── domain/
│   │   ├── trash.rs                # retention window, trashed-name encoding/decoding
│   │   ├── vault.rs                # hidden-vault rules (.nomedia dot-folder)
│   │   └── ports.rs                # FileOps, VaultLock
│   ├── slices/
│   │   ├── trash/                  # trash screen + service (restore, empty, undo)
│   │   ├── vault/                  # hidden screen + unlock flow
│   │   └── space/                  # "Free up space" screen + service
│   └── infrastructure/
│       ├── fs_ops.rs               # move/copy/delete adapters
│       └── biometric_lock.rs       # AuthHelper bridge
│
├── shell/                          # ── CONTEXT: the frame around the screens ──
│   ├── domain/
│   │   ├── navigation.rs           # Tab, Screen, resume state
│   │   ├── settings.rs             # typed settings + parsing rules
│   │   └── ports.rs                # SettingsStore, Diagnostics
│   ├── slices/
│   │   ├── navigation/             # app shell: top bar, tabs, dialogs
│   │   └── settings/               # settings screen
│   └── infrastructure/
│       ├── settings_file.rs        # settings.txt adapter
│       └── diag_log.rs             # diag.log adapter
│
└── platform/                       # ── APPLICATION-WIDE ADAPTERS (never imported by domain) ──
    ├── android/
    │   ├── perm.rs                 # runtime permissions
    │   ├── intents.rs              # "Open with" / content picker
    │   └── jni_env.rs              # JNIEnv helpers
    └── desktop/
        └── stubs.rs                # no-op implementations for PC builds and tests

apps/gallery/android/               # Java platform bridge; called only through platform/android/
apps/gallery/build-apk.ps1          # build, unchanged by this architecture
```

Directory rules:

- Every context has exactly one `domain/`, one `slices/` and one optional `infrastructure/`.
- Every slice is **flat**: `*_screen.rs` (presentation), `*_service.rs` (use cases),
  `*_repository.rs` (adapters used only by that slice) live side by side; there is no nested
  `application/`, `infrastructure/` or `presentation/` folder inside a slice.
- Slice folders are named after the entity (`photo`, `album`) or the process (`scan`).
- `platform/` is the only place allowed to touch `jni`, `ndk`, `android_activity` or
  `android_logger` directly.

## Dependency Rules

```
lib.rs (composition root)
   ├── shell/slices ──┐
   ├── library/slices ─┤
   ├── media/slices ───┼──> <context>/domain  (entities, rules, ports)
   ├── housekeeping/slices ─┘
   └── <context>/infrastructure ──> implements <context>/domain ports
                                    and may use platform/
```

- ✅ `slices/*` may depend on their own context's `domain/` and on `platform/` (through an adapter
  in `infrastructure/`, not directly).
- ✅ `infrastructure/*` implements traits declared in `domain/ports.rs` and may use `std::fs`,
  `jni`, `ndk`, `image`, `android_logger`.
- ✅ `lib.rs` wires the graph once and passes services to screens.
- ❌ `domain/` must never import `eframe`, `egui`, `jni`, `ndk`, `android_*`, `std::fs`,
  `std::process`, or any `infrastructure/`, `slices/` or `platform/` module.
- ❌ A slice must not import another slice in the same context or in another context. Cross-slice
  work goes through a process slice (`scan`) or through a domain type.
- ❌ Screens must not call `std::fs` or JNI directly; they call a service.
- ❌ `platform/` must not import any context. It is a leaf.
- ❌ No `#[cfg(target_os = "android")]` in `domain/` — platform selection belongs in the adapter
  layer and in the composition root.

## Layer/Module Communication

- **Screens → services:** a screen owns its widget state and calls one service method per user
  action; the service returns domain values, never egui types.
- **Services → ports:** a service receives its dependencies as `Arc<dyn Port>` at construction
  and never looks them up globally.
- **Infrastructure → domain:** adapters convert their own types (JNI objects, `std::fs::Metadata`,
  `io::Error`) into domain types at the boundary.
- **Background work:** a process slice (`scan`) owns its thread and channel; it reports
  domain-typed results to the composition root, which forwards them to the interested services.
  Long jobs never hold an egui context inside domain or service code.
- **Cross-context:** through domain values and, where a side effect is needed, through a service
  the composition root injects. No context reaches into another context's slice.

## Key Principles

1. **Domain purity.** Trash retention, album keys, thumbnail keys, date grouping and EXIF parsing
   are plain Rust with no framework types, so `cargo test` covers them on a PC.
2. **Ports before platforms.** Any new Android capability (new JNI call, permission, service) is
   introduced as a trait in `domain/ports.rs` with an Android adapter and a desktop stub, never as
   a `#[cfg]` branch inside a service.
3. **Flat slices, one feature one place.** Adding a screen means adding `*_screen.rs` next to its
   service and repository, not editing a shared god struct.
4. **One composition root.** All wiring lives in `lib.rs` (`Gallery::new` on Android,
   `main.rs` on desktop). Nothing else constructs adapters.
5. **Domain types cross boundaries.** Services and screens exchange `Item`, `Album`, `Thumbnail`,
   `PlaybackState`, `Settings` — not raw paths, JNI handles or egui textures.
6. **Destructive operations are explicit use cases.** Delete, move-to-trash, empty-trash and
   hide/unhide are service methods with their own tests; no screen performs filesystem work.

## Legacy vs New Code Policy

- **New Features:** All new code MUST strictly follow the architecture defined in this document.
  New files go into the folder this document defines, with a domain port for every external
  dependency.
- **Legacy Code Modification:** The current flat layout (`lib.rs` `impl Gallery` blocks,
  top-level `viewer.rs`, `media.rs`, `hidden.rs`, …) is legacy. Do NOT refactor unrelated legacy
  code to fit this architecture. Touch it only for bug fixes, when explicitly asked to refactor,
  or when it must be consumed by new code.
- **Interoperability:** When new code must call legacy code, isolate the interaction behind an
  adapter in the relevant `infrastructure/` folder, so legacy patterns do not leak into the new
  slices. Existing `impl Gallery` feature methods are treated as a facade during migration.

## Code Examples

### A domain port and its adapter

```rust
// library/domain/ports.rs — no framework, no I/O types
use std::path::{Path, PathBuf};

pub struct PhotoRecord {
    pub path: PathBuf,
    pub mtime: u64,
    pub size: u64,
    pub video: bool,
}

pub trait ItemSource: Send + Sync {
    fn list(&self, roots: &[PathBuf]) -> Vec<PhotoRecord>;
    fn remove(&self, path: &Path) -> std::io::Result<()>;
}
```

```rust
// library/infrastructure/fs_source.rs — the only place that walks the filesystem
use std::path::{Path, PathBuf};
use crate::library::domain::ports::{ItemSource, PhotoRecord};

pub struct FsItemSource {
    pub video_ext: &'static [&'static str],
}

impl ItemSource for FsItemSource {
    fn list(&self, roots: &[PathBuf]) -> Vec<PhotoRecord> {
        // std::fs::read_dir here; a failed entry is skipped, not fatal
        Vec::new()
    }

    fn remove(&self, path: &Path) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }
}
```

### A use-case service tested with a fake port

```rust
// housekeeping/domain/ports.rs — the minimal contract the service needs
use std::path::{Path, PathBuf};

pub trait FileOps: Send + Sync {
    fn move_to(&self, from: &Path, dir: &Path) -> std::io::Result<PathBuf>;
    fn remove(&self, path: &Path) -> std::io::Result<()>;
}
```

```rust
// housekeeping/slices/trash/trash_service.rs
use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::housekeeping::domain::ports::FileOps;

pub struct TrashService {
    files: Arc<dyn FileOps>,
}

impl TrashService {
    pub fn new(files: Arc<dyn FileOps>) -> Self {
        Self { files }
    }

    /// Returns the new location, so the caller can offer an undo.
    pub fn move_to_trash(&self, path: &Path, trash_dir: &Path) -> Option<PathBuf> {
        self.files.move_to(path, trash_dir).ok()
    }

    pub fn empty(&self, trash_dir: &Path) -> usize {
        // delegates listing to the adapter, returns how many items were erased
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// In-memory fake: records calls, never touches the disk.
    struct FakeFileOps;
    impl FileOps for FakeFileOps {
        fn move_to(&self, from: &Path, dir: &Path) -> std::io::Result<PathBuf> {
            Ok(dir.join(from.file_name().unwrap_or_default()))
        }
        fn remove(&self, _path: &Path) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn move_reports_the_new_location() {
        let service = TrashService::new(Arc::new(FakeFileOps));
        let moved = service.move_to_trash(Path::new("a.jpg"), Path::new("trash"));
        assert!(moved.is_some_and(|p| p.ends_with("a.jpg")));
    }
}
```

### The composition root wires ports once

```rust
// lib.rs
use std::sync::Arc;
use library::domain::ports::ItemSource;
use library::infrastructure::fs_source::FsItemSource;
use library::slices::photo::photo_service::PhotoService;

pub struct Gallery {
    photos: PhotoService,
    // ... other services, no adapters
}

impl Gallery {
    pub fn new(cc: &eframe::CreationContext<'_>, roots: Vec<PathBuf>) -> Self {
        let source: Arc<dyn ItemSource> = Arc::new(FsItemSource { video_ext: VIDEO_EXT });
        Self { photos: PhotoService::new(source), /* ... */ }
    }
}
```

## Anti-Patterns

- ❌ **`impl Gallery` as a god object** — adding another `fn album_menu(&mut self)` to `lib.rs`
  instead of a slice service plus screen.
- ❌ **Platform leakage** — `#[cfg(target_os = "android")]`, `jni::JNIEnv`, `ndk::…` or `std::fs`
  inside `domain/` or inside a service.
- ❌ **Screens doing I/O** — a screen calling `std::fs::rename` or a Java helper directly instead
  of a service.
- ❌ **Port-shaped hole** — an `infrastructure/` module that imports a slice or a screen.
- ❌ **Anemic domain** — putting trash retention, key derivation or grouping rules in a service
  while the domain type stays a bare data bag.
- ❌ **Shared mutable globals** — a `static Mutex<…>` adapter (as `diag.rs` uses today) reachable
  from domain code; inject a `Diagnostics` port instead.
- ❌ **Cross-context imports** — `media/slices/video/video_screen.rs` importing
  `housekeeping/slices/trash/trash_service.rs`.
