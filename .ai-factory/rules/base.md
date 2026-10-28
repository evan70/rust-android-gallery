# Project Base Rules

> Auto-detected conventions from the codebase analysis. Edit as needed.

## Naming Conventions

- Files: single lowercase word per module, except compound names written without separators
  (`albumops.rs`, `gridlayout.rs`, `thumbcache.rs`); the crate root is `lib.rs`
- Variables: `snake_case`, short and concrete (`items`, `roots`, `thumbs`, `scan_rx`, `done_rx`)
- Functions: `snake_case` verbs or verb phrases (`scan`, `start_scan`, `spawn_workers`,
  `default_trash_dir`, `move_to_trash`, `format_duration`)
- Types: `PascalCase` (`Gallery`, `Item`, `Album`, `Shared`, `VideoThumb`, `RowKind`)
- Constants: `SCREAMING_SNAKE_CASE` (`THUMB_PX`, `FULL_PX`, `MAX_THUMB_TEXTURES`, `TRASH_DAYS`)
- Module headers: a `//!` doc block at the top of each file explaining what it does and any
  non-obvious platform detail; `///` comments on fields, constants and functions whose purpose is
  not obvious from the name
- Section separators inside large files: `// ---------- name ----------`, except in `viewer.rs`

## Module Structure

- `apps/gallery/src/lib.rs` — the `Gallery` app struct, library scan, background decoding, the
  main UI loop and shared helpers (~2.7k lines). Keep new state on `Gallery` and new screens in
  their own module.
- Pure, testable logic lives in leaf modules with no egui state: `gridlayout.rs`, `exif.rs`,
  `housekeeping/domain/`, `thumbcache.rs`, `media.rs`, `diag.rs`.
- UI features extend `Gallery` from their own module via `impl Gallery` blocks: `albumops.rs`,
  `settings.rs`, `space.rs`, `hidden.rs`, `intents.rs`, `viewer.rs`.
- Integration tests live in `apps/gallery/tests/housekeeping.rs` (one binary, an inner module per
  rule area). They reach the crate through `gallery::housekeeping::…`, so anything a test needs
  there has to stay `pub`.
- Platform-specific code is behind `#[cfg(target_os = "android")]` and must keep a desktop
  implementation (or stub) so `cargo test` and the `gallery-desktop` binary still build.
- Java classes in `apps/gallery/android/java/dev/heartnetapps/gallery/` are a thin platform
  bridge only; UI and behaviour belong in Rust.
- Keep cross-module items `pub` and everything else private — most items in `lib.rs` stay private
  because the modules are in the same crate.

## Error Handling

- Return `Option` for "not found / not available" (`pub fn read(path: &Path) -> Option<Exif>`) and
  `std::io::Result` for filesystem operations (`move_to_trash`, `copy`, `rename`).
- Recoverable failures are ignored explicitly with `let _ = ...` or logged, never `unwrap()`ed on
  data that comes from the device (unwraps are acceptable in tests and on infallible invariants).
- No custom error enums: failures collapse into `None` / `io::Error` at the module boundary.
- Panics are last-resort only: the diag hook writes the message to `diag.log` before the release
  build aborts.

## Control Flow

- Prefer flat, readable control flow over deeply nested conditionals. Use guard clauses, early
  `return`/`continue`, small named helper methods, or explicit classification logic when they make
  the code easier to follow. Handle edge cases and irrelevant branches early so the main path stays
  visible.
- `let ... else { return ... }` is the standard early-exit form in this codebase.
- Long UI functions are split by section comments rather than by nesting; extract a helper instead
  of adding another `if` level.

## Logging

- `log` macros (`log::error!`, `log::warn!`) for platform-level problems; `android_logger` is
  initialised once at startup at Info level.
- `diag::log(...)` for the diagnostic trail (starts, resumes, scans, panic reasons). Keep it terse
  and never log file names or personal data.
- No `println!` / `dbg!` in application code.
