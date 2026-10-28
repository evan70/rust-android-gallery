# Android Rust Gallery

## Overview

A fast, low-memory photo and video gallery for Android written in Rust. The entire UI is drawn with
egui/eframe (glow renderer) inside a single native activity; Java is only a thin platform bridge.
Everything stays on the phone: there is no account, no sync and no network code. Media is read
straight from the shared storage folders, and app state is kept in small plain-text files plus a
thumbnail cache in the app's data directory.

The repository is a Cargo workspace (edition 2024, `resolver = "3"`) with one member, `apps/gallery`,
which builds both the Android `.so` (loaded by `GalleryActivity`) and a desktop binary used to try
the UI on a PC.

## Core Features

- Photos and videos in a date-grouped grid, with fast scrolling and a scroll handle
- Albums mode: real folders plus pinned virtual albums (All, Videos, Favourites, Trash, Hidden)
- Hidden vault locked with fingerprint / screen lock, stored in a `.nomedia` dot-folder
- Trash with 30-day restore window, undo snackbar, share, delete and move
- In-app video player: playback speed, loop, mute, rotate, floating window (foreground service),
  double-tap seeking on the screen edges
- Full-screen viewer with pinch zoom, swipe and EXIF details
- Storage cleanup view ("Free up space")
- "Open with" receiver and content picker so other apps can hand photos/videos to the gallery

## Tech Stack

- **Programming language:** Rust (edition 2024), `apps/gallery/src` (~6.6k lines)
- **UI framework:** `eframe`/`egui` 0.37 with the `glow` (OpenGL ES 3.0) renderer, custom dark visuals
- **Android integration:** `android-activity` 0.6 (native activity), `jni` 0.21, `ndk` 0.9
  (api-level 30, nativewindow), `android_logger` for `log` output
- **Image and video decoding:** `image` 0.25 (jpeg, png, webp, gif, bmp); video thumbnails and
  playback go through Android's `MediaMetadataRetriever` / `MediaPlayer` via JNI
- **Java bridge:** `apps/gallery/android/java/dev/heartnetapps/gallery/*.java`
  (`GalleryActivity`, `AuthHelper`, `DisplayHelper`, `MediaHelper`, `ShareHelper`, `ShareProvider`,
  `FloatingPlayerService`)
- **Database / ORM:** none — no database; metadata is read from the filesystem on demand
- **Build:** `apps/gallery/build-apk.ps1` (PowerShell, Windows). `cargo ndk` for arm64-v8a, then
  `javac` / `d8` / `aapt2` / `zipalign` / `apksigner` from build-tools 34.0.0, minSdk 30,
  targetSdk 34
- **Tests:** Rust `cargo test` — 27 in-module `#[cfg(test)]` unit tests plus the housekeeping
  contract in `apps/gallery/tests/housekeeping.rs`

## Current Code Structure

- Single-struct UI state: `Gallery` in `src/lib.rs` holds items, albums, view indices, texture
  handles and open dialogs; feature files add `impl Gallery` blocks (`albumops.rs`, `settings.rs`,
  `space.rs`, `hidden.rs`, `intents.rs`, `viewer.rs`).
- Pure logic is split out into testable modules without egui: `gridlayout.rs` (row/date layout),
  `exif.rs` (minimal TIFF/EXIF parser), `housekeeping/` (trash and vault rules in `domain/`,
  filesystem and screen-lock adapters in `infrastructure/`),
  `thumbcache.rs` (cache keys and JPEG cache files), `media.rs` (extension lists, duration
  formatting, `cfg`-gated Android/stub backends), `diag.rs` (rolling diagnostic log).
- Background work: a small worker pool (`Mutex` + `Condvar` job queue) decodes thumbnails and
  reports back over `mpsc` channels; the library scan runs on its own thread and sends
  `Vec<Item>`. Visible-range atomics let workers prioritise what is on screen.
- Platform code is gated with `#[cfg(target_os = "android")]` and has a desktop stub (see
  `media.rs`, `perm.rs`), so the app keeps compiling and testing on a PC.
- State persists through plain-text files in the data directory: `favourites.txt`, `settings.txt`,
  `pins`, resume state, `thumbs/` (JPEG thumbnails keyed by path/mtime/size hash) and `diag.log`.
- Permissions are kept minimal but include `MANAGE_EXTERNAL_STORAGE`, `READ_MEDIA_IMAGES/VIDEO`,
  `USE_BIOMETRIC`, `POST_NOTIFICATIONS`, `SYSTEM_ALERT_WINDOW` and a media-playback foreground
  service.

## Non-Functional Requirements

- **Logging:** `log` crate with `android_logger` (Info level) on Android; `diag::log` writes a
  rotating `diag.log` (max 120 KB, previous run kept as `diag.old`) that deliberately never records
  file names
- **Error handling:** `Option` / `io::Result` with early returns; recoverable failures are ignored
  or logged rather than panicking (panics are caught by the diag hook before the abort)
- **Memory:** bounded texture cache (`MAX_THUMB_TEXTURES`), 320 px thumbnails, release profile with
  `lto = "thin"`, `codegen-units = 1`, `panic = "abort"`, `strip = true`
- **Privacy:** no network access, no analytics, no identifiers; see `PRIVACY.md`
- **Security:** APKs are signed with the maintainer's own key (`docs/SIGNING.md`), release builds
  remap source paths out of the binary

## Architecture

Detailed architecture guidelines live in `.ai-factory/ARCHITECTURE.md`.
Pattern: Explicit Architecture (Flat Vertical Slice) — bounded contexts (`library`, `media`,
`housekeeping`, `shell`) with a framework-free domain and ports, flat slices per entity or process,
and a single composition root in `lib.rs`. The domain model is DDD in scope: bounded contexts,
ubiquitous language, aggregates with explicit invariants (Trash, Vault, Album, Media item) and value
objects (thumbnail key, EXIF, day key). ORM repositories, a unit of work, domain events and CQRS are
deliberately omitted — there is no relational store, no transaction boundary and only one writer.
The document describes the target structure; the current flat module layout is treated as legacy and
migrated incrementally.

## Known Gaps

- `README.md` "Layout" still mentions `tools/build/` for the shared APK build script, but no such
  directory exists in the tree — the script lives at `apps/gallery/build-apk.ps1`.
- There is no CI configuration and no git metadata in this checkout.
