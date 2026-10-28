# Project Roadmap

> A fast, low-memory photo and video gallery for Android, written in Rust, that keeps everything on
> the phone.

## Summary

The app covers the four capabilities the README promises: photos and videos in a date-grouped grid,
albums, a hidden album behind the device lock, and a 30-day trash. Each now has a canonical OpenSpec
spec, and the housekeeping extraction (archived 2026-10-28) moved the trash and vault rules out of the
UI modules behind the `FileOps` / `VaultLock` ports, so entry naming, retention, restore collisions
and the vault marker are stated once and testable without a device.

The open gap is test visibility, not behaviour. The housekeeping tests live in `#[cfg(test)]` modules
inside `apps/gallery/src/housekeeping/**`, which the coverage matrix does not treat as test evidence,
so the `vault` and `trash` requirements read as `partial` on verify and done runs.

## Evidence Sources

- **Lifecycle evidence:** `openspec/specs/vault/spec.md`, `openspec/specs/trash/spec.md`,
  `openspec/changes/archive/2026-10-28-trash-vault-domain-extraction/proposal.md`,
  `.ai-factory/qa/trash-vault-domain-extraction/done.md`,
  `.ai-factory/state/trash-vault-domain-extraction/final-summary.md`
- **GitHub evidence:** unavailable — this checkout has no remote, issues, or milestones, so lifecycle
  state is reconciled from local artifacts only.

<!-- aifhub:roadmap-change-lifecycle:start -->
## OpenSpec Change Lifecycle

| Change | Issues | Milestone | Roadmap item/slice | Local state | Evidence |
|---|---|---|---|---|---|
<!-- aifhub:roadmap-change-lifecycle:end -->

## Capabilities

The item names below are what a change writes into the `Roadmap item/slice` field of its
`## Roadmap Linkage` section. A change that names one gets a managed lifecycle row.

No change has claimed an item yet. `trash-vault-domain-extraction` predates this file and its
proposal records `- Roadmap item/slice: none`, so it deliberately has no lifecycle row — its
finalization evidence stays in Milestones and Completed.

| Item | Status | Local evidence |
|---|---|---|
| **Vault** | done | `openspec/specs/vault/spec.md`, `apps/gallery/src/housekeeping/domain/vault.rs`, `apps/gallery/src/hidden.rs` |
| **Trash** | done | `openspec/specs/trash/spec.md`, `apps/gallery/src/housekeeping/domain/trash.rs` |

## Milestones

- [x] **Vault** — hidden album in a `.nomedia` folder, unlocked by the device screen lock, with the
      marker written on first hide, origin-preserving unhide, and cached pixels dropped on lock.
- [x] **Trash** — 30-day retention under `<unix-seconds>_<percent-encoded path>` names, restore that
      never overwrites, and permanent delete confined to the trash folder.
- [ ] **Vault test seam** — move the housekeeping rule tests into `apps/gallery/tests/` so the
      coverage matrix sees them and the `allowWarnOnDone.coverage` workaround can be dropped.

## Completed

| Milestone | Date |
|-----------|------|
| Trash | 2026-10-28 |
| Vault | 2026-10-28 |

## Next Steps

- [ ] Implement `housekeeping-test-seam`, then set `allowWarnOnDone.coverage` back to `false`.
- [ ] Name `Vault` (or `Trash`) in the `Roadmap Linkage` of the next change touching either, so the
      lifecycle block gains its first `planned` row instead of reporting `skipped`.

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Housekeeping tests stay invisible to the coverage matrix | Medium — every future vault or trash change pays a coverage warning | `housekeeping-test-seam` |
| Roadmap items drift from the names proposals actually write | Low — linkage stops matching anything, so changes skip the lifecycle block again | Keep the spelling identical to this table (`Vault`, `Trash`) in `Roadmap item/slice` |

---

*Last updated: 2026-10-28*
