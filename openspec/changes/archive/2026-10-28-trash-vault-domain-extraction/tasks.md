# Tasks

## 1. Planning and artifacts

- [x] 1.1 Create `proposal.md`, `design.md`, `tasks.md` and the `vault` delta spec (the `trash` requirements are canonical in `openspec/specs/trash/spec.md` and stay unchanged); verification: `openspec validate trash-vault-domain-extraction --type change --strict --json --no-interactive --no-color` exits 0
- [x] 1.2 Keep the canonical specs honest: after implementation, no requirement is edited to match the code; verification: `openspec validate photos --type spec --strict` and `openspec validate trash --type spec --strict` both exit 0
- [x] 1.3 Run `ai-factory aifhub-mode sync --change trash-vault-domain-extraction` and confirm generated rules compile without warnings other than the expected "no active change" ones; verification: command exits 0 and `.ai-factory/rules/generated/index.json` lists the change

## 2. Domain rules (no I/O, no platform)

- [x] 2.1 Create `apps/gallery/src/housekeeping/domain/trash.rs` with only the entry-name codec as associated functions: `TrashEntryName::encode(&Path) -> String`, `TrashEntryName::decode(&str) -> Option<PathBuf>` and `TrashEntryName::new(timestamp, original) -> Self`; verification: an in-module test round-trips `/a b/100% sure/x%2Fy.png`, `C:\\x\\y.jpg` and `ünï/été 📷.jpg`, and asserts the produced name contains no `/` or `\`
- [x] 2.2 Add `TrashEntryName::parse(&str) -> Option<(u64, PathBuf)>` that splits on the first `_`, parses the seconds and decodes the rest; verification: test parses `1700000000_%2Fsdcard%2Fa.jpg` into `(1700000000, /sdcard/a.jpg)` and returns `None` for `junk.txt` and `abc_x.jpg`
- [x] 2.3 Add the retention and restore-naming rules to the same module: `TRASH_DAYS = 30`, `DAY_SECS = 86_400`, `is_expired(trashed_at, now)` and `restored_name(original, attempt)`; verification: test asserts `is_expired(t, t + 30 * DAY_SECS) == false` and `is_expired(t, t + 30 * DAY_SECS + 1) == true`, and that `restored_name` yields `a (restored).jpg` then `a (restored 2).jpg`
- [x] 2.4 Create `apps/gallery/src/housekeeping/domain/vault.rs` with `Vault { dir }`, `Vault::ensure(&dyn FileOps)` (create dir, best-effort `.nomedia`) and `Vault::contains`; verification: `cargo test` builds the module and `Vault::contains` has a unit test for the `..` path trick
- [x] 2.5 Create `apps/gallery/src/housekeeping/domain/ports.rs` with `FileOps` and `VaultLock` traits and the `LockResult` enum (0 waiting, 1 granted, 3 no screen lock, other denied); verification: `cargo build` reports no `#[cfg]` inside the trait definitions, and the doc comment on `poll` names the four outcomes
- [x] 2.6 Create `apps/gallery/src/housekeeping/mod.rs`, `housekeeping/domain/mod.rs` and `housekeeping/infrastructure/mod.rs`, and register `mod housekeeping;` in `apps/gallery/src/lib.rs`; verification: `cargo build` succeeds and `hidden.rs`, `albumops.rs` and `viewer.rs` resolve `crate::housekeeping::…` paths
- [x] 2.7 Add `days_left(trashed_at, now)` to `housekeeping/domain/trash.rs`, keeping the existing `div_ceil` rounding (0..=30 days) so the viewer label does not shift; verification: unit test asserts 30 for a fresh entry, 1 for 29 days 12 h and 0 for 30 days + 1 s

## 3. Ports and adapters

- [x] 3.1 Create `apps/gallery/src/housekeeping/infrastructure/fs_ops.rs` implementing `FileOps` over `std::fs`: `move_file` with `rename` then `copy` + length check + source removal, `remove`, `create_dir_all`, `write_marker`, `exists`, plus `default_trash_dir` / `default_hidden_dir` carrying the existing Android/desktop paths; verification: cross-device fallback is covered by a unit test that fails the rename and asserts a short copy leaves both files untouched
- [x] 3.2 Create `apps/gallery/src/housekeeping/infrastructure/biometric_lock.rs` implementing `VaultLock`: `#[cfg(target_os = "android")]` calls `perm::authenticate` / `perm::take_auth`, desktop returns granted immediately; verification: `cargo test` on the desktop target compiles the desktop branch, and `grep -rn "perm::" apps/gallery/src/housekeeping/domain/` returns nothing
- [x] 3.3 Wire both adapters as `Arc<dyn FileOps>` / `Arc<dyn VaultLock>` fields: `FsOps` and the desktop lock in `Gallery::new` (`lib.rs:595`), and the Android lock as a handle-holder whose `Mutex<Option<AndroidApp>>` slot is filled in `Gallery::with_android` (`lib.rs:569-579`) — the app handle does not exist in `new()` (`android: None` at `lib.rs:724`); also move the `trash::default_trash_dir()` / `trash::default_hidden_dir()` calls at `lib.rs:612-613` into the adapter; verification: `cargo build` succeeds on the desktop target, `grep -rn "FsOps::new\|BiometricLock::new" apps/gallery/src` shows only the composition root, `grep -n "trash::default_" apps/gallery/src/lib.rs` returns nothing, and `with_android` assigns the handle into the lock
- [x] 3.4 Replace the direct `perm::authenticate` / `perm::take_auth` calls in `hidden.rs::request_unlock` and `hidden.rs::poll_auth` with `VaultLock::request` / `VaultLock::poll`, preserving the `auth_pending` gate and the 0/1/3 outcome mapping; leave `update_secure` (`perm::set_secure`) untouched; verification: `grep -n "perm::authenticate\|perm::take_auth" apps/gallery/src/hidden.rs` is empty and the desktop preview unlocks without a prompt

## 4. Call-site migration (behavior preserved)

- [x] 4.1 Migrate the move-to-trash call sites — `trash_items` (batch, `lib.rs:1248`, also used by album delete) and `trash_item` (single item from the viewer, `lib.rs:1350`) — to `TrashEntryName::new` + `FileOps`; verification: `cargo test` green and neither function calls `trash::` (`grep -n "trash::" apps/gallery/src/lib.rs` shows no hit inside `fn trash_items` or `fn trash_item`)
- [x] 4.2 Migrate the restore call sites — `restore_items` (batch, `lib.rs:1276`) and `restore_item` (single item from the viewer, `lib.rs:1371`) — to `TrashEntryName::parse` + `restored_name` + `FileOps::exists`/`move_file`; verification: the ported restore-never-overwrites test passes and a foreign-named entry still fails restore without panicking
- [x] 4.3 Migrate the permanent-delete call sites (`lib.rs:1300`, `1399`, `1418`) and the `vault_of` directory choice to the domain containment check + `FileOps::remove`; verification: the ported delete-only-inside-trash test passes, and deleting a hidden item still targets the vault folder rather than the trash folder
- [x] 4.4 Replace the expiry step inside `apply_scan` (predicate at `lib.rs:1030`, delete at `1031`) with `is_expired`; verification: boundary test from 2.3 stays green and an entry older than 30 days is deleted and marked `gone` by `apply_scan`
- [x] 4.5 Replace the `scan` trash/vault branch (`lib.rs:105-131`, `trash::original_of` at `111`) with `TrashEntryName::parse`; verification: the existing `trashed_media_shows_up_only_as_trash_and_decodes_without_an_extension` test still passes and a scan of a temp folder with one valid and one foreign trash file yields one item with `trashed = true` and one item with no `orig`
- [x] 4.6 Replace `hidden.rs` hide/unhide internals with `Vault::ensure`, `TrashEntryName` and `FileOps`, keeping the thumbnail-cache purge and `notify_media` order; verification: existing hide/unhide behavior unchanged — hiding still removes the cached thumbnail and sets `orig`, unhiding restores the original folder
- [x] 4.7 Replace `albumops.rs:40` `trash::move_file` with `FileOps::move_file`, and `viewer.rs:319` `trash::trashed_at` plus the retention arithmetic at `viewer.rs:321` with `TrashEntryName::parse` and `days_left`; delete the duplicate `const TRASH_DAYS` in `lib.rs`; verification: `cargo test` green, `grep -rn "trash::" apps/gallery/src` is empty, and `grep -c "86_400" apps/gallery/src/lib.rs` returns 0

## 5. Tests

- [x] 5.1 Port the six existing `trash.rs` tests unchanged (encoding round-trip, trash time from name, trash/restore round-trip, restore never overwrites, delete forever only inside trash, cannot trash twice) to the new modules; verification: all six pass under `cargo test`
- [x] 5.2 Add the domain edge-case tests not already written in 2.1–2.3: a name with no `_`, an empty string, a truncated percent escape (`abc%2`), a `%` followed by non-hex (`%ZZ`) and a decode that yields an empty path; verification: every case returns `None` and none panics, and the test module imports no `std::fs`
- [x] 5.3 Add a `FakeFileOps` test double in the domain tests and assert `Vault::ensure` still reports success when the marker write fails; verification: the fake records `create_dir_all` and a failing `write_marker`, and the test passes
- [x] 5.4 Verify the vault "locking purges cached pixels" requirement manually in the desktop preview: hide two photos, unlock, then lock again; verification: the two hidden thumbnails and durations are dropped and re-decoded on the next unlock, while a visible photo keeps its cached thumbnail
- [x] 5.5 Migrate the three `lib.rs` in-module tests that call `trash::move_to_trash` / `trash::restore` (`lib.rs:2567-2568` in `trashed_media_shows_up_only_as_trash_and_decodes_without_an_extension`, `2592` in `hidden_files_are_found_only_as_hidden_and_in_no_other_view`, `2684-2687` in `batch_trash_and_restore_move_every_item_once`) to the domain API plus a temp-dir `FsOps`; verification: those three tests pass unchanged in intent and `grep -rn "trash::" apps/gallery/src` is empty
- [x] 5.6 Delete `apps/gallery/src/trash.rs` and remove its `mod trash;` declaration — run only after 5.1–5.5; verification: `cargo test` from `apps/gallery` compiles without the module, no `use crate::trash` remains, and the ported trash tests still pass

## 6. Documentation and verification

- [x] 6.1 Run `cargo clippy --all-targets` and `cargo fmt --check` from `apps/gallery` against the pre-change baseline; the crate-wide `-D warnings` gate is out of scope because the baseline already carries 31 warnings and formatting diffs in files this change may not touch (see Forbidden Change Surface); verification: clippy reports no warning in `housekeeping/**`, `hidden.rs` or the migrated call sites of `lib.rs`, `albumops.rs` and `viewer.rs`, and `cargo fmt --check` shows only pre-existing diffs (the project deliberately writes dense lines, e.g. `albumops.rs:12`), both recorded in `.ai-factory/qa/trash-vault-domain-extraction/verify.md`
- [x] 6.2 Run the full `cargo test` from `apps/gallery` and record the before/after count (baseline 32 tests) in the change summary; verification: all tests pass and no test is ignored or removed
- [x] 6.3 Update the stale structure notes so they point at `housekeeping/` instead of the deleted `trash.rs`: `.ai-factory/DESCRIPTION.md` "Current Code Structure", the `AGENTS.md` project tree, and the module list in `.ai-factory/rules/base.md`; verification: `grep -rn "trash.rs" AGENTS.md .ai-factory/` returns nothing

## Commit Plan

> This checkout has no git repository (`git.enabled: false`, no `.git`), so these are **verification
> checkpoints**, not commits: they mark the points where the tree must be green before the next
> group starts. If the repository is initialized before implementation, each checkpoint is one
> commit with the message shown.

| Checkpoint | After | Suggested commit message | Gate to pass before continuing |
|---|---|---|---|
| A | 1.x | `docs(openspec): plan trash and vault domain extraction` | `openspec validate … --strict` and `aifhub-mode sync` exit 0 |
| B | 2.x | `refactor(housekeeping): add trash and vault domain rules and ports` | `cargo build` compiles the new module tree; domain tests from 2.1–2.3 toward a green run |
| C | 3.x | `refactor(housekeeping): add filesystem and biometric adapters` | `cargo test` green; adapters constructed only in `Gallery::new` |
| D | 4.x | `refactor(trash): route trash and vault call sites through ports` | `cargo test` green; `grep -rn "trash::" apps/gallery/src` is empty |
| E | 5.x | `test(housekeeping): port trash tests and delete the old module` | all ported tests green; `apps/gallery/src/trash.rs` deleted; manual lock-purge check done |
| F | 6.x | `docs: point structure notes at housekeeping` | no new clippy warning in the changed files, full `cargo test`, doc greps clean |

Stop and fold back to planning if a checkpoint gate fails twice: a red gate after the second fix
attempt means the split is wrong, not the code.
