# Design: Move the housekeeping tests into an integration test crate

## Technical Approach

Two small mechanical changes unlock one file move:

1. **Visibility.** `apps/gallery/src/lib.rs` declares `mod housekeeping;`. Cargo integration tests are
   a separate crate that links the library, so they can only reach `pub` items. The declaration
   becomes `pub mod housekeeping;`. Every type the tests need is already `pub` inside the tree
   (`TrashEntryName`, `TRASH_DAYS`, `DAY_SECS`, `is_expired`, `days_left`, `restored_name`,
   `restore_dest`, `now_secs`, `Vault`, `FileOps`, `VaultLock`, `LockResult`, `FsOps`,
   `BiometricLock`), so no further visibility edits are required. The library is only consumed by
   `main.rs` and the Android `cdylib`, so widening it costs nothing at runtime.

2. **Test location.** All `#[cfg(test)] mod tests { … }` blocks under `apps/gallery/src/housekeeping/`
   are deleted and their bodies are re-created once in `apps/gallery/tests/housekeeping.rs`.

```
apps/gallery/tests/
└── housekeeping.rs          one integration binary, 17 tests
    ├── mod support          temp(name), FakeFiles (records calls, marker can fail)
    ├── mod trash            6 tests: codec, parse, retention, days_left, restore naming, broken names
    ├── mod vault            3 tests: containment, marker failure, folder failure (NoDir)
    ├── mod ports            1 test:  0/1/3/other → Waiting/Granted/NoScreenLock/Denied
    ├── mod fs_ops           2 tests:  moves+markers+remove, default folders
    └── mod flow             4 tests:  trash/restore round trip, never overwrite,
                                       delete only inside the folder, cannot trash twice
```

`support` is a plain inner module in the same file (Cargo only treats top-level files in `tests/` as
test crates, so a helper module must not be a sibling file unless it is declared with `mod`).

The one test that stays behind is `a_short_copy_is_thrown_away_and_the_source_kept` in
`infrastructure/fs_ops.rs`. It asserts the private `copied_fully(from, to, copied, len)` guard, which
sits inside `move_file`'s cross-filesystem fallback. Reaching that fallback from outside the crate
would need an injectable copier **and** an injectable rename (`fs::rename` succeeds first for two
paths on the same filesystem), i.e. two test-only seams on a production adapter. Keeping one
four-line test next to the guard is cheaper and leaves the adapter honest. Its module header gains a
comment naming the reason.

## Data / Artifact Model

- No persisted data, schema, cache format or on-disk layout changes.
- Test artifacts:
  - `apps/gallery/tests/housekeeping.rs` — new, the single home of the housekeeping contract.
  - `apps/gallery/src/housekeeping/**` — no `#[cfg(test)]` except the documented guard test.
  - Coverage evidence for later changes can cite the relative path
    `apps/gallery/tests/housekeeping.rs`; `openspec-coverage-matrix` classifies it as test evidence
    because it lives under `tests/`.
- Naming inside the file mirrors the source module (`trash`, `vault`, `ports`, `fs_ops`, `flow`) so a
  failing test name still tells the reader which rule broke.

## Integration Points

| Test (current location) | New location |
|---|---|
| `domain/trash.rs`: `encoding_round_trips_awkward_names`, `a_trash_entry_name_carries_its_time_and_original_place`, `retention_boundary_is_exactly_thirty_days`, `days_left_keeps_the_rounded_up_label`, `restored_names_count_collisions`, `broken_names_are_rejected_without_panicking` | `tests/housekeeping.rs::trash` |
| `domain/vault.rs`: `contains_compares_the_parent_not_the_prefix`, `ensure_survives_a_marker_that_cannot_be_written`, `ensure_reports_a_folder_that_cannot_be_created` | `tests/housekeeping.rs::vault` (+ `support::FakeFiles`) |
| `domain/ports.rs`: `lock_codes_map_to_the_four_outcomes` | `tests/housekeeping.rs::ports` |
| `infrastructure/fs_ops.rs`: `moves_create_dirs_and_write_markers`, `default_folders_are_dot_folders` | `tests/housekeeping.rs::fs_ops` |
| `infrastructure/fs_ops.rs`: `a_short_copy_is_thrown_away_and_the_source_kept` | **stays** (private guard) |
| `infrastructure/biometric_lock.rs`: `the_desktop_lock_never_blocks` | `tests/housekeeping.rs::ports` (keep `#[cfg(not(target_os = "android"))]`) |
| `housekeeping/mod.rs`: `trash_and_restore_round_trip`, `restore_never_overwrites`, `delete_forever_only_inside_trash`, `cannot_trash_twice_or_restore_non_trash` | `tests/housekeeping.rs::flow` (+ `support::temp`, `flow::{move_in, move_out, original_of}`) |

`lib.rs` keeps `into_folder` / `out_of_folder` and the three scan/trash tests: they exercise `scan`,
`HIDDEN_KEY` and `EXT_KEY`, which are crate-private and unrelated to this contract.

## Alternatives Considered

1. **Only relax the coverage policy (already done).** Zero code, but the matrix would keep warning for
   every future housekeeping change and nothing would point a reviewer at the contract. Rejected as
   the whole solution; the flag stays as a safety net for other in-file tests.
2. **Expose `copied_fully` as `pub` so all 18 tests move.** Smallest file count, but it publishes an
   internal invariant (`copied == len`) as API and invites callers to depend on the short-copy guard.
   Rejected.
3. **Inject a copier into `FsOps` (`FsOps::with_copy`).** Testable and no new public function, but
   `move_file` attempts `fs::rename` first, so on one filesystem the seam is never reached; making the
   test meaningful would require injecting the rename too. Two test-only seams on the production
   adapter are worse than one local test. Rejected.
4. **One test file per module (`tests/trash.rs`, `tests/vault.rs`, …).** Each file becomes its own
   test binary and its own link step, and `FakeFiles`/`temp` would be duplicated or need a shared
   `mod support` in every file. Rejected in favour of one binary with inner modules.
5. **Leave the tests in place and add a thin `tests/housekeeping.rs` façade.** Would satisfy the
   coverage path while keeping duplicate suites. Rejected: two sources of truth for the same rules.

## Risks

- **Private helper dependency.** If a moved test touches a `pub(crate)` item, the integration crate
  will not compile. Mitigation: move tests module by module and keep the offending test in-file (as
  with the copy guard) instead of widening the API.
- **Assertion drift while copying.** A hand-copied test can silently lose an assertion. Mitigation:
  move each test verbatim, keep its name, and rely on `cargo test` counts (44 total: 27 + 17) plus a
  per-file review of the moved bodies.
- **Public surface creep.** Making the module public also silences dead-code warnings and lets future
  code reach internals. Mitigation: `pub mod housekeeping;` only; no re-exports, no `pub use` in
  `lib.rs`, and the struct fields stay as they are.
- **Desktop build on Linux.** `eframe` is declared without `x11`/`wayland`, so the test target does
  not build on a bare Linux host without them. Same limitation as the parent change; verified in a
  temporary copy, not by editing `Cargo.toml` (forbidden surface).
- **Coverage matrix expectations.** The matrix matches test paths found in task or verification text.
  Unless the change that uses the new file names its path, evidence will still not attach — this
  change can only make the path *available*, not force future matches.
