## Original Request

Vytvor prvý OpenSpec change pre migráciu trash a vault pravidiel do domain typov podľa ARCHITECTURE.md

## Roadmap Linkage

- Issues: none
- Milestone: none
- Roadmap item/slice: none
- Rationale: none

## Why

`apps/gallery/src/trash.rs` mixes pure rules (entry naming, retention, original-path decoding) with
filesystem I/O, and `apps/gallery/src/hidden.rs` keeps the vault invariants inside an
`impl Gallery` UI block: it creates the folder, writes `.nomedia`, and borrows `trash::move_to_trash`
and `trash::restore` for vault moves. The consequences are concrete:

- The vault invents its invariants at the call site (`hidden.rs:93-98`), so a second caller could
  move items into the vault without the `.nomedia` marker and no test would notice.
- The trash retention rule lives as an inline expression in `lib.rs:1030`
  (`now.saturating_sub(t) > TRASH_DAYS * 86_400`), not as a named, testable predicate.
- The entry-name codec (`trash::encode` / `decode`) is private, so restore safety can only be
  tested by touching the filesystem.
- `.ai-factory/ARCHITECTURE.md` defines a `housekeeping` bounded context with a framework-free
  domain and ports for exactly this kind of rule, and its "Legacy vs New Code Policy" allows
  extracting legacy code incrementally instead of rewriting it.

This is the first migration step named in the architecture document, and it is deliberately
behavior-preserving: the same files move, the same names are written, the same items expire.

## What Changes

- Add `apps/gallery/src/housekeeping/domain/` with framework-free rules and value objects:
  `trash.rs` (entry-name codec, retention predicate, restore-collision naming),
  `vault.rs` (vault folder invariants, hide/unhide origin handling), `ports.rs` (`FileOps`,
  `VaultLock` traits).
- Add `apps/gallery/src/housekeeping/infrastructure/` adapters: `fs_ops.rs` (move with rename-else-copy
  fallback, remove, create directory, write marker file) and `biometric_lock.rs` (Android
  `perm::authenticate` / `perm::take_auth` behind the `VaultLock` port, desktop auto-unlock).
- Construct the adapters once in the composition root — `FsOps` and the desktop lock in
  `Gallery::new` (`lib.rs:595`), the Android lock's `AndroidApp` handle in `Gallery::with_android`
  (`lib.rs:569-579`, because `new()` has `android: None`) — and pass them as `Arc<dyn FileOps>` /
  `Arc<dyn VaultLock>`; the existing `impl Gallery` screens keep their signatures and act as the
  legacy facade.
- Port the six existing `trash.rs` unit tests to the new modules and add pure tests for the codec,
  retention boundary and restore collision naming that previously required disk access.

Compatibility boundary: the crate is a single `cdylib`/`rlib`; `trash::*` items are crate-internal,
so their move is not a public API break. The Java bridge and the Android manifest are untouched.

## Capabilities

### New Capabilities

- `vault`: owns the hidden-album location and its `.nomedia` marker, requires an unlock before its
  contents are listed or shown, and preserves the original location across hide/unhide.

### Modified Capabilities

None. The trash requirements are already canonical in `openspec/specs/trash/spec.md` and this change
is behavior-preserving, so it carries no trash delta — the extraction must keep that spec true
rather than change it. No delta spec is needed for the code-move parts (domain split, ports,
adapters); the only behavior contract this change introduces is the vault delta above.

## Impact

Affected code:

- `apps/gallery/src/trash.rs` (removed; rules move to `housekeeping/domain/trash.rs`, I/O to
  `housekeeping/infrastructure/fs_ops.rs`)
- `apps/gallery/src/hidden.rs` (screen logic calls the vault domain and `VaultLock` port)
- `apps/gallery/src/lib.rs` (`scan`, expiry sweep, trash move/restore/delete call sites, `Gallery`
  fields for the adapters)
- `apps/gallery/src/albumops.rs` (`trash::move_file` → `FileOps::move_file`)
- `apps/gallery/src/viewer.rs` (`trash::trashed_at` → domain entry parse)
- `apps/gallery/src/settings.rs` (unchanged call site, verified by the build)

Dependencies: none added; still `std`, `eframe`, `jni`/`ndk` on Android only.

Systems: filesystem layout unchanged (`/sdcard/.GalleryTrash`, `/sdcard/.GalleryHidden` on Android);
Android biometric prompt reused through the existing `AuthHelper` path.

Assumptions: the current tests encode the intended behavior; the 30-day check is strictly "more than
30 days", not "30 days or more".

Risks and open questions:

- Move ordering during extraction can silently change expiry semantics (`>` vs `>=`) — pinned by a
  boundary test.
- The `.nomedia` write is currently best-effort (`let _ =`); the domain must keep it best-effort and
  not turn a marker failure into a failed hide.
- Open question: whether `VaultLock` should also own `update_secure` (screenshot blocking). Decision
  in `design.md`: keep it in the existing permission bridge for this change.

## SDD Profile Inputs

```json
{
  "planning_mode": "full",
  "behavior_change": false,
  "modules": 1,
  "repositories": 1,
  "public_api": false,
  "data_migration": false,
  "reversible": true,
  "security_sensitive": true,
  "architecture_novelty": false,
  "requirements_clear": true,
  "expected_files": 11
}
```

## Non-goals

- No behavior change: same trash/vault paths, same entry-name scheme, same 30-day window, same
  " (restored)" suffix rules, same confirmations and messages.
- No UI changes and no move of the `impl Gallery` trash/hidden screens into slices (deferred to a
  later change).
- No new permissions, no media-database changes, no encryption of the vault.
- No change to the thumbnail cache format or to `albumops::move_file` semantics beyond relocating
  the helper behind `FileOps`.

## Acceptance Examples

| Given | When | Then |
|---|---|---|
| the crate at its current revision | the extraction is complete | `cargo test` in `apps/gallery` passes with at least the current 32 tests, none removed or ignored |
| a trashed entry exactly 30 days old | the expiry sweep runs | the entry stays in the trash |
| a trashed entry 30 days and one second old | the expiry sweep runs | the entry is erased |
| a vault folder that does not exist | the first item is hidden | the folder exists and contains an empty `.nomedia` |
| the vault is locked | the user opens the Hidden album | no hidden item is listed and no thumbnail is decoded for it |
| `housekeeping/domain/` after extraction | an import audit runs | no `std::fs`, `eframe`, `jni`, `ndk` or `perm::` reference is present |
| a file outside the trash folder | permanent deletion is requested for it | the call errors and the file survives |

## Allowed Change Surface

- `apps/gallery/src/housekeeping/mod.rs`
- `apps/gallery/src/housekeeping/domain/**`
- `apps/gallery/src/housekeeping/infrastructure/**`
- `apps/gallery/src/trash.rs` (deleted)
- `apps/gallery/src/hidden.rs`
- `apps/gallery/src/lib.rs`
- `apps/gallery/src/albumops.rs`
- `apps/gallery/src/viewer.rs`
- `AGENTS.md` (project tree, stale `trash.rs` entry only)
- `.ai-factory/DESCRIPTION.md` and `.ai-factory/rules/base.md` (stale-structure notes only)

## Forbidden Change Surface

- `apps/gallery/Cargo.toml` and `Cargo.toml` — no new dependencies, no edition/profile changes
- `apps/gallery/android/**` — manifest, Java bridge and resources stay as they are
- `apps/gallery/src/perm.rs`, `media.rs`, `exif.rs`, `gridlayout.rs`, `thumbcache.rs`, `space.rs`,
  `settings.rs`, `intents.rs`
- `openspec/specs/**` — the delta specs are archived by `$aif-done`, not by this change

## Verification Plan

- `cargo test` from `apps/gallery` (baseline: 32 tests) — must pass with no test removed or ignored
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` from `apps/gallery`
- `grep -n "std::fs\|eframe\|jni\|ndk\|perm::" apps/gallery/src/housekeeping/domain/*.rs` — no output
- `grep -rn "trash::" apps/gallery/src` — no remaining call into the removed module
- `openspec validate trash-vault-domain-extraction --type change --strict` — passes
- Desktop smoke test: `cargo run --bin gallery-desktop -- <tmp-folder>`, trash a file, restore it,
  hide it, unlock on desktop, unhide it

## Requirements Reconciliation

Declared authority: `.ai-factory/ARCHITECTURE.md` (target structure and legacy-vs-new policy) >
`.ai-factory/rules/base.md` (detected conventions) > current code (evidence of existing behavior, not
a requirement source).

Cited decisions:

- `ARCHITECTURE.md` "Dependency Rules": `domain/` must not import `std::fs`, `jni`, `ndk`, `egui`;
  service/screen code calls adapters through ports.
- `ARCHITECTURE.md` "Legacy vs New Code Policy": extract legacy code only where new code consumes
  it; do not refactor unrelated files.
- `ARCHITECTURE.md` "Domain Model (DDD)" → Trash and Vault aggregates with the invariants quoted in
  the delta specs.
- `rules/base.md`: `Option`/`io::Result` at module boundaries, no custom error enums, guard clauses,
  in-module `#[cfg(test)] mod tests`.

Behavior dimensions covered by the extracted model:

| Dimension | Supported values | Rule under test |
|---|---|---|
| Entry state | in trash / not in trash | move refuses a second trash; delete refuses outside paths |
| Entry age | `elapsed <= 30d` / `elapsed > 30d` | only strictly older entries are erased by the sweep |
| Restore target | free / occupied | occupied target gets " (restored)" / " (restored n)" |
| Entry name | valid scheme / foreign name | foreign names are not restorable and are not guessed |
| Vault folder | missing / present without `.nomedia` / present with `.nomedia` | marker ensured before the first item is moved |
| Vault lock | locked / unlocked | no listing or thumbnails while locked |

Verification evidence: the existing `trash.rs` tests (round-trip, restore-never-overwrites,
delete-only-inside-trash, cannot-trash-twice) are the authority for current behavior; they are ported
unchanged to the extracted modules, and `cargo test` in `apps/gallery` must pass before and after.

## Research Context

Omitted — no research artifact exists for this change.
