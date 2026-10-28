# AGENTS.md

> A structural map of this repository for AI agents. Keep it factual and update it when the layout
> changes.

## Project Overview

A fast, low-memory photo and video gallery for Android written in Rust. The whole UI is egui/eframe
drawn into a single native activity; Java is only a thin platform bridge. No network, no database —
media is read from the filesystem and app state lives in plain-text files plus a thumbnail cache.
See `.ai-factory/DESCRIPTION.md` for the full specification.

## Tech Stack

- **Programming language:** Rust (edition 2024), Cargo workspace with one member
- **Framework:** eframe/egui 0.37 (glow renderer), android-activity + jni + ndk on Android
- **Database:** none
- **ORM:** none

## Project Structure

```
Cargo.toml                     workspace manifest (members = apps/gallery)
apps/gallery/
  Cargo.toml                   gallery crate: cdylib+rlib for Android, `gallery-desktop` bin
  build-apk.ps1                Windows PowerShell APK build: cargo ndk → javac/d8 → aapt2 → sign
  src/
    lib.rs                     `Gallery` app struct, library scan, worker pool, main UI, helpers
    main.rs                    desktop runner (`gallery-desktop <folder>`)
    albumops.rs                albums: create/rename/delete, move/copy picker, pins
    viewer.rs                  full-screen viewer: pinch zoom, swipe, video controls
    media.rs                   video thumbnails and playback (JNI on Android, stub on desktop)
    gridlayout.rs              date-grouped grid layout (pure logic)
    thumbcache.rs              thumbnail cache keys and JPEG cache files
    housekeeping/              trash and hidden vault: domain rules + filesystem/lock adapters
      domain/trash.rs          trashed-name encoding, 30-day retention, restore naming
      domain/vault.rs          dot-folder + .nomedia rules, containment check
      domain/ports.rs          FileOps and VaultLock traits
      infrastructure/fs_ops.rs std::fs adapter, default trash/vault folders
      infrastructure/biometric_lock.rs  screen-lock prompt (desktop: always granted)
    hidden.rs                  hidden vault UI and unlock flow
    exif.rs                    minimal EXIF/TIFF reader (pure logic)
    space.rs                   "Free up space" view
    settings.rs                settings screen
    intents.rs                 "Open with" / content picker intents
    perm.rs                    Android runtime permissions (Android only)
    diag.rs                    rolling diagnostic log (`diag.log`)
  android/
    AndroidManifest.xml        package dev.heartnetapps.gallery, minSdk 30, targetSdk 34
    java/dev/heartnetapps/gallery/*.java   thin Java bridge (activity, biometric, share, player)
    res/                       launcher icons and colours
docs/                          INSTALL.md, SIGNING.md
```

## Key Entry Points

| File | Purpose |
|------|---------|
| `apps/gallery/src/lib.rs` | `Gallery` struct and `eframe::App` implementation — start here for UI work |
| `apps/gallery/src/main.rs` | Desktop binary entry point for trying the UI on a PC |
| `apps/gallery/android/java/dev/heartnetapps/gallery/GalleryActivity.java` | Android entry point that loads `libgallery.so` |
| `apps/gallery/android/AndroidManifest.xml` | Permissions, activity, provider and foreground service declarations |
| `apps/gallery/build-apk.ps1` | The only supported APK build path (Windows + PowerShell) |
| `Cargo.toml` | Workspace definition and release profile |

## Documentation

| Document | Path | Description |
|----------|------|-------------|
| README | README.md | What the app is, features, install and build steps |
| Install guide | docs/INSTALL.md | Installing the released APK |
| Signing guide | docs/SIGNING.md | Signing APKs with your own key (`HEARTNET_KEYSTORE`) |
| Contributing | CONTRIBUTING.md | Contribution notes |
| Privacy | PRIVACY.md | What the app does and does not collect |
| Security | SECURITY.md | How to report a vulnerability |

## AI Context Files

| File | Purpose |
|------|---------|
| AGENTS.md | This map of the repository for AI agents |
| .ai-factory/DESCRIPTION.md | Project specification: stack, features, architecture notes |
| .ai-factory/ARCHITECTURE.md | Target architecture: bounded contexts, flat slices, ports and dependency rules (current flat modules are legacy, migrated incrementally) |
| .ai-factory/rules/base.md | Detected coding conventions (naming, modules, errors, logging) |
| .ai-factory/config.yaml | AI Factory settings (UI language `sk`, artifacts English, git disabled, OpenSpec-native mode via `aifhub.tools.openspec: true`) |
| openspec/specs/ | Canonical OpenSpec requirements: `photos`, `albums`, `trash` (project `android-rust-gallery` in `openspec/config.yaml`) |
| openspec/changes/ | OpenSpec change proposals — new plans are written here, not to `.ai-factory/plans/` |
| .ai-factory/rules/generated/ | Rules compiled from the active OpenSpec change (`openspec-base.md`, `index.json`) |
| .agents/skills/ | Installed skills, including `rust-skills` and `rust-engineer` |
| skills-lock.json | Sources and hashes of the installed external skills |

## Agent Rules

- Run shell commands one at a time, not chained — a failure then points at the exact step.
  - Incorrect: `cd apps/gallery && cargo test && cargo build`
  - Correct: first `cd apps/gallery`, then `cargo test`, then `cargo build`
- This checkout has no git repository, so do not plan branch, diff or push steps.
- `cargo test` (from `apps/gallery` or the workspace root) is the verification command; the APK
  build needs Windows, the Android SDK/NDK and a signing key, so it is not part of normal checks.
- Keep platform code behind `#[cfg(target_os = "android")]` with a desktop stub so tests keep
  building on a PC.
- `rust-skills` assumes `thiserror`/`tokio`; this project uses plain `Option`/`io::Result` and no
  async runtime, so apply its guidance only where it fits the existing conventions.
- Artifact protocol is OpenSpec: requirements live in `openspec/specs/`, work is proposed as a
  change under `openspec/changes/<id>/` (`proposal.md`, `tasks.md`, `specs/**/spec.md`) and archived
  into specs when done. Check mode with `ai-factory aifhub-mode status`, never by hand-editing
  `aifhub.tools.openspec`.
