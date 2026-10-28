## Original Request

Plan a follow-up change that moves housekeeping tests into apps/gallery/tests

## Roadmap Linkage

- Issues: none
- Milestone: none
- Roadmap item/slice: none
- Rationale: none

## Why

The housekeeping extraction (`trash-vault-domain-extraction`, archived 2026-10-28) left all 18 of its
tests in `#[cfg(test)]` modules inside `apps/gallery/src/housekeeping/**`. `openspec-coverage-matrix`
only recognises evidence paths that look like standalone test files, so the five `vault`
requirements stayed `partial` in `.ai-factory/qa/trash-vault-domain-extraction/coverage.json` even
though every one of them is tested. Done readiness only passed after `allowWarnOnDone.coverage` was
flipped to `true`.

That policy is a workaround, not a fix: any future change touching the vault or the trash pays the
same warning, and reviewers cannot point at one file that holds the housekeeping contract. An
integration test crate at `apps/gallery/tests/housekeeping.rs` gives the coverage matrix a real test
path, keeps the rule modules almost free of test scaffolding, and makes the contract runnable on its
own with `cargo test --test housekeeping`.

## What Changes

- `apps/gallery/src/lib.rs` exposes `housekeeping` as a public module (`mod housekeeping;` →
  `pub mod housekeeping;`) so a test crate outside the library can reach the domain rules and
  adapters. Nothing else in the crate's public surface changes.
- 17 of the 18 in-module housekeeping tests move to `apps/gallery/tests/housekeeping.rs`, grouped by
  the module they cover (`trash`, `vault`, `ports`, `fs_ops`, `flow`).
- One test stays where it is: `a_short_copy_is_thrown_away_and_the_source_kept` exercises the private
  `copied_fully` guard in `fs_ops.rs`. It cannot move without either exposing that guard or adding a
  copy seam that `move_file` would bypass on the same filesystem (`fs::rename` succeeds first), so it
  keeps a short comment explaining the exception.
- `AGENTS.md` and the module-structure notes point at the new test file.

Explicit non-goals: no behaviour change in the trash, the vault or the app; no new dependency; no
`Cargo.toml`, `android/**` or unrelated module edits; the in-module tests of `lib.rs`, `albumops.rs`,
`viewer.rs`, `thumbcache.rs`, `media.rs` and `gridlayout.rs` stay where they are because they exercise
private crate internals.

## Capabilities

### New Capabilities

None — this change moves test code and widens one module's visibility; no product or workflow
requirement changes.

### Modified Capabilities

None — `openspec/specs/{albums,photos,trash,vault}/spec.md` stay untouched, which is why the change
declares `skip_specs: true` in `.openspec.yaml`.

## Impact

- **Code:** `apps/gallery/src/lib.rs` (one line),
  `apps/gallery/src/housekeeping/{mod.rs,domain/*.rs,infrastructure/*.rs}` (test modules removed), new
  `apps/gallery/tests/housekeeping.rs`.
- **Public surface:** `gallery::housekeeping` becomes reachable for the test crate. The library is
  only consumed by `main.rs` and the Android `cdylib`, so nothing external depends on it staying
  private.
- **Tests:** the suite keeps its size — 44 tests before, 44 after: 27 in the library (26 existing plus
  the copy-guard test that stays) and 17 in the integration target.
- **Tooling:** `cargo test --test housekeeping` becomes a one-command check of the housekeeping
  contract; the coverage matrix can cite `apps/gallery/tests/housekeeping.rs` as test evidence.
- **Risks:** a moved test may rely on a private helper and fail to compile (fix by keeping that test
  in-file, not by widening the API); the integration crate can drift from the library's internal
  helpers; the test file becomes the only place where the housekeeping contract is expressed, so it
  must stay complete.
- **Assumptions:** the project keeps its current test conventions (no dev-dependencies, no
  `[[test]]` harness declaration — Cargo discovers `tests/*.rs` automatically); the desktop test
  target still builds on a PC where `eframe` has its platform features.

## SDD Profile Inputs

```json
{
  "planning_mode": "full",
  "behavior_change": false,
  "modules": 2,
  "repositories": 1,
  "public_api": true,
  "data_migration": false,
  "reversible": true,
  "security_sensitive": false,
  "architecture_novelty": false,
  "requirements_clear": true,
  "expected_files": 9
}
```

## Non-goals

- No refactoring of the trash or vault rules; their signatures and behaviour stay exactly as they are.
- No new public API beyond the module visibility: the private `copied_fully` guard stays private.
- No move of the `lib.rs` / `albumops.rs` / `viewer.rs` / `thumbcache.rs` / `media.rs` /
  `gridlayout.rs` tests; they touch private internals of the crate root and are not part of the
  housekeeping contract.
- No new test framework, dev-dependency, `[[test]]` target declaration or CI job.
- No change to the trash/vault retention, naming, containment or unlock rules.
- No edits under `openspec/specs/**` and no revival of the archived `trash-vault-domain-extraction`
  change.

## Acceptance Examples

- **Moved suite runs standalone.** Given a clean checkout, when `cargo test --test housekeeping` runs,
  then 17 tests execute and all pass, and none of them is annotated `#[ignore]`.
- **Rule modules carry (almost) no tests.** Given `apps/gallery/src/housekeeping/**`, when the tree is
  searched for `#[cfg(test)]`, then exactly one match remains — in `fs_ops.rs`, next to the private
  copy guard and its explaining comment.
- **Library suite keeps its size.** Given `cargo test` at the workspace root, when it finishes, then
  the total is still 44 passing tests, with 27 in the library target and 17 in the integration target.
- **Moved tests keep their assertions.** Given the six trash tests, when they run from the
  integration crate, then the encoding round-trip, name parsing, 30-day boundary, `days_left` labels,
  `(restored)` naming and the broken-name cases assert the same values they did in-module.
- **Vault contract is reachable from outside.** Given `gallery::housekeeping::domain::vault::Vault`,
  when the integration crate builds one over a temp folder, then `ensure`, `contains` and `remove`
  behave as they did inside the crate.

## Allowed Change Surface

- `apps/gallery/src/lib.rs` — the `mod housekeeping;` line only
- `apps/gallery/src/housekeeping/**` — test modules removed (one documented exception in `fs_ops.rs`)
- `apps/gallery/tests/housekeeping.rs` — new integration test crate
- `AGENTS.md`, `.ai-factory/DESCRIPTION.md`, `.ai-factory/rules/base.md` — notes naming where the
  housekeeping tests live
- `openspec/changes/housekeeping-test-seam/**` — this change

## Forbidden Change Surface

- `Cargo.toml` and `apps/gallery/Cargo.toml` — no dev-dependencies, no `[[test]]`, no edition/profile
  changes
- `apps/gallery/android/**`, `apps/gallery/src/perm.rs`, `media.rs`, `exif.rs`, `gridlayout.rs`,
  `thumbcache.rs`, `space.rs`, `settings.rs`, `intents.rs`, `hidden.rs`, `albumops.rs`, `viewer.rs`
- `openspec/specs/**`
- `.ai-factory/config.yaml` — the `allowWarnOnDone.coverage` flag stays as it is; this change removes
  the reason to need it, not the policy itself

## Verification Plan

1. `cargo test --test housekeeping` → 17 passed, 0 failed.
2. `cargo test` from `apps/gallery` → 44 passed total (27 library + 17 integration).
3. `grep -rn "#\[cfg(test)\]" apps/gallery/src/housekeeping` → exactly one match, in `fs_ops.rs`.
4. `grep -n "pub mod housekeeping" apps/gallery/src/lib.rs` → one match.
5. `openspec validate housekeeping-test-seam --type change --strict` → valid.
6. `ai-factory aifhub-mode sync --change housekeeping-test-seam` → exit 0 (no delta specs, so only a
   report is written).
7. Review note: on a Linux host whose `eframe` lacks `x11`/`wayland`, the test run needs the same
   temporary feature workaround as the parent change; on Windows and in `cargo ndk` builds the
   manifest works as is.
