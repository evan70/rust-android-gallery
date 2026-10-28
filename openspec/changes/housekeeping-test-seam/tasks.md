# Tasks

## 1. Planning and artifacts

- [ ] 1.1 Write `proposal.md`, `design.md`, `tasks.md` and `.openspec.yaml` (`schema: spec-driven`, `skip_specs: true` because no requirement changes); verification: `openspec validate housekeeping-test-seam --type change --strict --json --no-interactive --no-color` exits 0
- [ ] 1.2 Confirm the accepted specs stay untouched and the change needs no delta spec; verification: `openspec validate --all --strict` still reports 4 passing specs and `git status --short openspec/specs` (or a directory listing when git is unavailable) shows no modification under `openspec/specs/**`

## 2. Visibility

- [ ] 2.1 Change `mod housekeeping;` to `pub mod housekeeping;` in `apps/gallery/src/lib.rs`, without re-exports or other visibility edits; verification: `grep -n "pub mod housekeeping" apps/gallery/src/lib.rs` prints exactly one line, `grep -c "^pub mod\|^pub use" apps/gallery/src/lib.rs` counts only that line, and the crate still builds

## 3. Move the tests

- [ ] 3.1 Create `apps/gallery/tests/housekeeping.rs` with a `support` module (`temp(name)`, `FakeFiles` recording calls and failing the marker write on demand) and the `trash` module holding the six codec/retention tests moved verbatim from `domain/trash.rs`; verification: `cargo test --test housekeeping` runs 6 tests, all pass
- [ ] 3.2 Add the `vault` module with the three tests moved from `domain/vault.rs`, including the local `NoDir` adapter; verification: `cargo test --test housekeeping` runs 9 tests, all pass, and `Vault::ensure` still reports success when `write_marker` fails
- [ ] 3.3 Add the `ports` module with `lock_codes_map_to_the_four_outcomes` and `the_desktop_lock_never_blocks` (keeping `#[cfg(not(target_os = "android"))]`), importing `gallery::housekeeping::domain::ports::…`; verification: `cargo test --test housekeeping` runs 11 tests, all pass
- [ ] 3.4 Add the `fs_ops` module with `moves_create_dirs_and_write_markers` and `default_folders_are_dot_folders`, using `gallery::housekeeping::infrastructure::fs_ops::FsOps`; verification: `cargo test --test housekeeping` runs 13 tests, all pass
- [ ] 3.5 Add the `flow` module with `trash_and_restore_round_trip`, `restore_never_overwrites`, `delete_forever_only_inside_trash` and `cannot_trash_twice_or_restore_non_trash`, plus the `move_in`/`move_out`/`original_of` helpers built on `Vault`, `TrashEntryName` and `FsOps`; verification: `cargo test --test housekeeping` runs 17 tests, all pass
- [ ] 3.6 Delete the moved in-module tests: the `#[cfg(test)] mod tests` blocks in `domain/trash.rs`, `domain/vault.rs`, `domain/ports.rs`, `infrastructure/biometric_lock.rs`, `housekeeping/mod.rs` (`flow_tests`), and the two moved tests in `infrastructure/fs_ops.rs`; keep `a_short_copy_is_thrown_away_and_the_source_kept` next to `copied_fully` with a comment naming the exception; verification: `grep -rc "#\[cfg(test)\]" apps/gallery/src/housekeeping` reports one match in `fs_ops.rs` and zero elsewhere, and `cargo test` is still 44 passing

## 4. Verification

- [ ] 4.1 Record the split: `cargo test --lib` (27 passed) and `cargo test --test housekeeping` (17 passed) and `cargo test` (44 passed, 0 failed, 0 ignored); verification: the three commands finish with those counts and no test name is marked `#[ignore]`
- [ ] 4.2 Lint the new file: `cargo clippy --all-targets` must report no warning inside `apps/gallery/tests/housekeeping.rs`, and `grep -n "use crate::" apps/gallery/tests/housekeeping.rs` must be empty because an integration crate sees `gallery::…` only; verification: the clippy output names no line of the new test file and the grep prints nothing
- [ ] 4.3 Confirm no manifest change was needed: `git diff --stat -- Cargo.toml apps/gallery/Cargo.toml` (or the same `grep` check on a checkout without git) is empty and Cargo still auto-discovers the target; verification: `cargo test --test housekeeping -- --list` lists the 17 names without any `[[test]]` entry in the manifest

## 5. Documentation and final checks

- [ ] 5.1 Point the structure notes at the new test home: `AGENTS.md` project tree (a `tests/housekeeping.rs` entry), the module notes in `.ai-factory/DESCRIPTION.md` and `.ai-factory/rules/base.md`; verification: `grep -rn "tests/housekeeping.rs" AGENTS.md .ai-factory/DESCRIPTION.md .ai-factory/rules/base.md` prints one hit per file
- [ ] 5.2 Run `openspec validate housekeeping-test-seam --type change --strict` and `ai-factory aifhub-mode sync --change housekeeping-test-seam`; verification: both exit 0, and the generated `openspec-change-housekeeping-test-seam.md` says "No OpenSpec change requirements found." because the change carries no delta specs

## Commit Plan

| Checkpoint | After | Suggested commit message | Gate before continuing |
|---|---|---|---|
| A | 1.x | `docs(openspec): plan the housekeeping test move` | `openspec validate … --strict` exits 0 |
| B | 2.x | `refactor(housekeeping): expose the module for integration tests` | crate still builds, one `pub mod` line only |
| C | 3.1–3.5 | `test(housekeeping): add the integration test crate` | `cargo test --test housekeeping` → 17 passed |
| D | 3.6–4.x | `test(housekeeping): drop the in-module tests` | `cargo test` → 44 passed, one `#[cfg(test)]` left |
| E | 5.x | `docs: point structure notes at the housekeeping tests` | validate + sync exit 0, doc greps clean |

This checkout has no committed git history (`git.enabled: false`, everything untracked), so the
checkpoints are verification gates; each becomes one commit with the message above if the repository
is initialized first.
