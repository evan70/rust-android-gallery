---
name: aif-plan
description: Plan a feature or task in fast, full, or ultra mode. Ultra creates an indexed multi-file bundle with deeply specified phases for execution by a smaller model. Use for "plan", "new feature", "start feature", "create tasks", or exhaustive implementation planning.
argument-hint: "[fast | full | ultra] [--parallel | --list | --cleanup <branch>] <description>"
allowed-tools: Read Write Glob Grep Bash(git *) Bash(cd *) Bash(cp *) Bash(mkdir *) Bash(basename *) Bash(shasum -a 256 *) Bash(sha256sum *) TaskCreate TaskUpdate TaskList AskUserQuestion Questions Task mcp__handoff__handoff_sync_status mcp__handoff__handoff_push_plan mcp__handoff__handoff_get_task mcp__handoff__handoff_list_tasks mcp__handoff__handoff_update_task
disable-model-invocation: false
version: 1.0.0
---

<!-- aif-ext:aifhub-extension:aif-plan:prepend:start -->
## AIFHub OpenSpec-Native Planning Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-plan` body. When any rule below conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Use the built-in `/aif-plan` skill as the canonical planning entrypoint for this extension workflow.

### Canonical Workflow

- Do not redirect users to deprecated planning aliases.
- The canonical public flow is `/aif-analyze -> /aif-explore -> /aif-plan -> /aif-improve -> /aif-implement -> /aif-verify`.
- Legacy planning references should be interpreted as `/aif-plan full`.

### Planning profile and AI Factory version gate

Resolve the artifact profile and parse the leading planning mode before creating a plan directory or writing any planning artifact.

- Recognize only a command-position mode token: `fast`, `full`, or `ultra`. Remove that one leading token from the request body; preserve every later occurrence of those words byte-for-byte, including casing, punctuation, internal whitespace, and line breaks.
- Absence of an explicit `ultra` token keeps the existing default, `fast`, or `full` behavior. Do not enable ultra from extension baseline metadata.
- For explicit `ultra`, use the dependency-free shared resolver in `scripts/ai-factory-version-resolver.mjs` before any write. Resolution precedence is: injected test toolchain/version, then project-managed `.ai-factory.json.version`, then bounded CLI evidence only when the executable provenance is proven to match the same project installation.
- A global or PATH-only `ai-factory --version` is not authoritative. When stable project metadata exists, unverified global/PATH-only CLI evidence is ignored with a bounded `ai-factory-cli-provenance-unverified` warning and does not override the project version.
- Missing, malformed, prerelease, unsupported `<2.18.0`, or provenance-matched CLI/project mismatch results fail closed and create no plan, change, bundle, companion, runtime-state, or research artifact.
- On that failure emit only a bounded diagnostic with `mode`, `profile`, `version`, `source`, and stable `code`, followed by the actionable handoff `/aif-plan full <request>`. Never include the request body, research body, provider output, credentials, raw stdout, or raw stderr in the diagnostic.

For stable AI Factory `>=2.18.0`, route explicit `ultra` by the resolved artifact profile:

- In OpenSpec-native mode, `ultra` is a depth profile over the canonical OpenSpec change. Apply the upstream Ultra Detail Gate to exact files and symbols, ordered edits, contracts and data flow, failure handling, bounded logging, tests, acceptance criteria, rollback, and verification detail in `design.md` and `tasks.md`.
- OpenSpec-native `ultra` still writes only `proposal.md`, `design.md`, `tasks.md`, and applicable delta specs. It MUST NOT create `index.md`, `phase-NN-*.md`, companion files, or an active standalone `<!-- aif:plan-mode:ultra -->` anywhere under the canonical change.
- In legacy AI Factory-only mode, explicit `ultra` remains upstream-owned. Hand control to the upstream `aif-plan` ultra workflow before AIFHub classic companion normalization; do not create or synchronize a sibling classic plan or companion files.

### Research and interface choices

After mode/version checks permit planning, read [research delegation and design alternatives](../../skills/shared/RESEARCH-DESIGN.md) when independent fact-finding or a material interface choice needs separate attention. Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/RESEARCH-DESIGN.md`. Apply this in OpenSpec-native and legacy planning: the coordinating parent accepts source-bound findings and compares distinct interfaces before writing the affected design. Keep routine changes direct, preserve the existing artifact owner, and use the same read-only guidance as a local fallback when subagents are unavailable. This does not replace upstream ultra gates or create another planning workflow.

### Executable task design

After mode and target checks permit local planning/refinement, follow [executable task design](../../skills/shared/TASK-DESIGN.md) for new task breakdowns, wide migrations or unclear prerequisites. Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/TASK-DESIGN.md`. Preserve current task identities and the existing bounded inline-verification migration; this guidance does not authorize restructuring an unrelated plan, tracker writes or an implementation runtime.

### MCP work-item-derived plan identity

Resolve work-item identity after preserving the explicit request and before deriving a new OpenSpec `change-id` or running the upstream sequential next-number scan. This contract is provider-neutral and covers GitHub Issues, Linear, Jira, YouGile, and other issue or task systems exposed through MCP.

- Accept a work-item identity only from the explicit planning input or the structured MCP record selected for that input. Prefer provider fields such as `identifier`, `key`, `number`, or display ID; never extract an ID from the title, description, labels, milestone text, branch name, or search-result ordering.
- Set `source_provider` to a canonical lowercase provider or MCP server identifier. Set `primary_source` to the provider's canonical HTTPS work-item URL when available; otherwise use a stable MCP resource URI returned by the provider. If MCP returns only a stable server name, resource kind, and record ID, construct `mcp://<server>/<resource-kind>/<stable-record-id>`. Never persist credentials, fragments, secret query parameters, raw provider payloads, or a transient search URL.
- Set `external_id` to the provider's short human-readable work-item key, for example `156`, `ENG-431`, or `PROJ-77`. If the provider exposes only an opaque stable ID, create `<provider>-<first-eight-stable-alphanumeric-characters>`, for example `yougile-a1b2c3d4`, while retaining the full opaque identity only in `primary_source`.
- A bare number, a pull-request URL, a branch name, title, label, milestone, roadmap text, or unrelated item discovered during exploration is not identity evidence. A GitHub shorthand such as `issue #156` is accepted only when the current repository is directly known and it resolves unambiguously to one canonical issue URL.
- Exactly one distinct work item selects `source_provider`, `primary_source`, and `external_id`. If several items are linked, use source-derived identity only when one is explicitly primary; preserve every canonical work-item reference under `## Roadmap Linkage`. Without an explicit primary item, keep the ordinary mode-specific ID policy and never choose the first, lowest, or highest item implicitly.
- Derive a bounded request slug from the requested work without repeating the external ID, then call `deriveSourceBoundChangeId(external_id, request_slug)`. The resulting semantic ID is `<normalized-external-id>-<request-slug>`, for example `156-fix-login-timeout`, `eng-431-fix-login-timeout`, or `proj-77-refresh-token`.

When the work-item identity resolves, persist it separately from the many-valued roadmap linkage:

```markdown
## AIFHub Source Binding

- Provider: <source_provider>
- Primary source: <primary_source>
- External ID: <external_id>
- Branch: <exact current git branch|none>
```

- Capture `Branch` from the exact attached git branch used for plan creation. Write the literal `none` only when no attached branch can be resolved; never derive it from the request slug or work-item title.
- In OpenSpec-native mode, write the exact section once in `proposal.md`. In legacy classic mode, write it once in the parent `.md` entrypoint and synchronize the same values to `status.yaml` as exact double-quoted scalars under `source_binding.provider`, `source_binding.primary_source`, `source_binding.external_id`, and `source_binding.branch`. For an upstream-owned marked ultra bundle, persist the Markdown section once in `index.md` and do not create a companion `status.yaml`.
- `Primary source` is the single canonical collision identity. `Provider` and `External ID` make it readable, but neither may authorize reuse by itself. The `Issues` list in `## Roadmap Linkage` remains lifecycle linkage and MUST NOT establish or replace the primary binding, even when it contains the same external ID from another provider, tenant, or repository.
- Treat `Provider`, `Primary source`, and `External ID` as immutable. Preserve `Branch` during ordinary refinement; change it only for an explicit branch-rebind request that keeps the same primary source and passes the same collision checks.
- Use `parseWorkItemSourceBinding()`, `parseLegacyWorkItemSourceBinding()`, `parseSynchronizedWorkItemSourceBinding()`, `matchesPrimarySourceBinding()`, and `deriveSourceBoundChangeId()` from `scripts/active-change-resolver.mjs` when creating or inspecting artifacts. Missing, duplicate, malformed, mismatched, unsynchronized, or differently bound metadata fails closed for creation, refinement, and direct validation.

Apply the resolved work-item identity by artifact mode:

- **OpenSpec-native mode:** set the new canonical `change-id` to `deriveSourceBoundChangeId(external_id, request_slug).changeId`. This source-binding override is independent of `workflow.plan_id_format`; validate it with `normalizeChangeId()` before any write.
- **Legacy AI Factory-only mode with active `workflow.plan_id_format: slug`:** use `<normalized-external-id>-<request-slug>` as the plan identifier.
- **Legacy AI Factory-only mode with active `workflow.plan_id_format: sequential`:** keep the required four-digit compatibility prefix. When `external_id` is a decimal value in `1..9999`, use that value as the four-digit prefix and the request slug as the stem (`156` -> `0156_fix-login-timeout`). For an alphanumeric or out-of-range external ID, allocate the upstream four-digit ordinal normally and begin the semantic stem with `<normalized-external-id>-<request-slug>`, for example `0042_PROJ-77-refresh-token`. The source binding, not the compatibility ordinal, remains the identity.
- `HANDOFF_BRANCH_PREPARED=1` retains upstream precedence and disables the legacy sequential prefix; the unprefixed semantic stem still begins with `<normalized-external-id>-`. Fast and fix plans keep their upstream artifact shapes but use the same semantic stem whenever their workflow accepts a plan identifier.

Collision handling is fail-closed and source-aware:

- Before choosing a path, scan active source-bound artifacts. If exactly one valid binding has `Primary source` equal to `primary_source`, reuse its existing identifier and route to refinement even if the current request would derive a different slug. If more than one active artifact claims the same primary source, stop with `ERROR [aif-plan] ambiguous-primary-source-binding provider=<provider> external-id=<bounded-id>`.
- In OpenSpec-native mode, if `openspec/changes/<derived-change-id>/` exists, reuse it only when its proposal has exactly one valid binding whose `Primary source` equals `primary_source`. Mere membership in `## Roadmap Linkage`, or equality of `Provider` / `External ID`, is insufficient. Otherwise stop with `ERROR [aif-plan] source-plan-id-collision external-id=<bounded-id> path=openspec/changes/<derived-change-id>`.
- In legacy mode, reuse an occupied plan identifier only when its Markdown entrypoint and, for classic plans, companion `status.yaml` contain valid synchronized bindings with the exact same `Primary source`. Any unbound, differently bound, unsynchronized, or differently shaped occupant stops with `ERROR [aif-plan] source-plan-id-collision external-id=<bounded-id> identifier=<bounded-plan-id>`.
- Never overwrite, allocate a suffix, or silently drop the external-ID prefix after a source-bound collision. The request slug is the readable disambiguator; the persisted full primary source protects the remaining cross-provider and cross-tenant collision case. Diagnostics may include only bounded provider, external ID, project-relative path or identifier, mode, and stable code.

After a source-bound OpenSpec change has been created or successfully refined, call `writeCurrentChangePointer(<resolved-change-id>)` from `scripts/active-change-resolver.mjs`. Do not advance the pointer before all canonical artifacts and the exact source binding pass local shape checks, and do not advance it on any collision or failed write. One exact persisted branch binding remains higher-precedence than ordinary slug branch matching. When several active source-bound changes intentionally share the same creation branch, the pointer written by the latest successful `/aif-plan` disambiguates them; without a pointer to one of those exact candidates, resolution fails with `ambiguous-branch-binding`. The pointer is also the deterministic fallback when `Branch` is `none` or no branch mapping applies.

On success, emit `INFO [aif-plan] source-bound plan identity: provider=<provider> external-id=<bounded-id> artifact=<project-relative-path>`.

### OpenSpec-native mode

When `.ai-factory/config.yaml` has `aifhub.tools.openspec: true`, OpenSpec-native instructions override legacy plan-folder instructions.

#### SDD planning depth and SessionBrief

For a newly planned canonical change, evaluate the reviewed task facts using the mapping in [SDD profiles](../../docs/sdd-profiles.md) and [ADR 0004](../../docs/adr/0004-sdd-profiles-and-session-brief.md). Keep the public `planning_mode` (`fast | full | ultra`) separate from the internal `profile`: `direct | quick | standard | expanded | ultra | tracer | research`. There is no internal profile named `full`.

- `direct` recommends the upstream `fast` path for one-file non-behavioral work. Before creating a canonical change, return that owner handoff when the actual mode is `fast`; do not manufacture an OpenSpec change solely to store a decision. Explicit `full` retains compact canonical planning as `quick`.
- `quick` uses compact `full` planning for a bounded change in one module and at most five expected files. `standard` uses `full` for broader scope and includes design. Public API, security/privacy, migrations, irreversibility, or multiple repositories select `expanded` with design. Explicit `ultra` retains the existing version-gated depth contract. Never silently change the user's command mode; return a mode-mismatch owner handoff when needed.
- Missing facts, conflicting requirements, or architecture uncertainty select `research` and return `/aif-explore` before implementation. `tracer` runs through `ai-factory aifhub-tracer run` / `aifhub-tracer promote`; it never satisfies `/aif-done`, and promotion requires updating canonical proposal/design/tasks/specs, running `/aif-mode sync --change <change-id>`, and compiling a fresh production SessionBrief before full implementation. Task text length and model confidence do not establish risk or completeness.
- Preserve project-required tests, security, review, human approval, migration/rollback, `/aif-verify`, and `/aif-done` rules independently of planning depth. The optional `.ai-factory/sdd-policy.json` adds a profile floor/gates/context references; it grants no permissions, tools, or network. Do not generate or overwrite project policy automatically.
- Record actual mode and reviewed facts once under `## SDD Profile Inputs` as the closed JSON shape documented in the guide. Unknown values remain omitted or `null`. This conditional AIFHub section supplements the source template; preserve `## Original Request` byte-for-byte and all exact source-template headings.
- Include meaningful `## Non-goals`, `## Acceptance Examples` (Given/When/Then), `## Allowed Change Surface`, and, where needed, `## Forbidden Change Surface`, `## Constraints`, `## Assumptions`, `## Open Questions`, and `## Verification Plan`. Behavioral changes require non-goals, acceptance examples, and delta specs. Every executable brief needs an explicit allowed surface. Do not add empty boilerplate or invent product architecture for docs/tooling-only work. Acceptance examples supplement formal requirements/scenarios. Unresolved Open Questions block implementation.
- Quick planning may shorten prose/rounds, but must still create design or any other artifact required by the selected OpenSpec schema or project policy. Validate canonical shape and run `/aif-mode sync --change <change-id>` through the existing owner workflow before compiling.
- Run `ai-factory aifhub-session-brief compile --change <change-id> --json` after canonical source/sync revisions settle. The helper owns only `.ai-factory/state/<change-id>/sdd/profile-decision.json` and `.ai-factory/state/<change-id>/context/session-brief.md` / `session-brief.json`; it never writes canonical content, QA, or generated rules. Keep credentials, provider transcripts, and hidden reasoning out of selected source sections. A successful compile proves context binding only; it does not satisfy a quality gate.
- Return the current brief digest in the implementation handoff. Missing/stale/blocked context returns to the planning owner, or explore for research. Recompile after later canonical edits. Do not silently retrofit SDD to existing unopted plans; they retain canonical filesystem fallback. If helpers are unavailable for a new or already opted-in SDD change, report the missing capability and return an owner handoff without using an unvalidated brief.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Use canonical OpenSpec artifacts under `openspec/changes/<change-id>/`:

- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/<capability>/spec.md` when the change affects product or workflow behavior

Do not create legacy `.ai-factory/plans` plan files or companion folders in this mode. Do not write runtime-only files into `openspec/changes/<change-id>/`.

If the task is docs/tooling-only and does not change product or workflow behavior, a delta spec may be omitted only when the plan explicitly explains why no delta spec is needed. When the selected OpenSpec CLI is `>=1.7.0`, also declare native OpenSpec `skip_specs: true` in `openspec/changes/<change-id>/.openspec.yaml`; preserve the selected `schema` and other existing metadata. For an older supported CLI, preserve the explicit proposal reason and compatibility finalizer path instead of writing metadata that the selected CLI does not understand.

When a behavior change removes the final requirement of a capability and the selected OpenSpec CLI is `>=1.8.0`, add native `retire_capabilities: true` to `openspec/changes/<change-id>/.openspec.yaml`, preserving the selected `schema` and other existing metadata. Do this only when the user's request explicitly authorizes capability retirement. Planning must not infer retirement merely because a `REMOVED` delta would leave the base capability empty. For an older supported CLI, record the required OpenSpec upgrade as a blocking archive prerequisite instead of writing unsupported retirement metadata.

#### Task Intake Normalization

Before normalizing task content, preserve the explicit planning request as canonical raw source when one was supplied:

- Remove only recognized invocation tokens that occur in command positions: the leading `/aif-plan` or runtime-equivalent skill token, one leading `fast`, `full`, or `ultra` mode token, and recognized control flags parsed before the request body.
- Do not remove words such as `full`, `fast`, `ultra`, `--list`, or `--parallel` when they occur inside the actual request text.
- Trim only parser-introduced outer whitespace. Preserve the remaining request wording, casing, punctuation, internal whitespace, and line breaks exactly; do not translate, summarize, normalize, or regenerate it.
- Write the fixed `## Original Request` heading and preserved body to `proposal.md` before the source-template `## Why` heading.
- If planning starts only from the resolved research artifact and no explicit request exists, omit `## Original Request`.
- If both an explicit request and relevant research influence the plan, keep both `## Original Request` and `## Research Context`; they have different source and mutability contracts.

Reconcile requirements before exploring an implementation:

- Treat canonical specs and the committed `## Research Context` snapshot as authoritative requirement sources; generated rules are derived guidance subordinate to canonical specs, `## Original Request` is the raw intent anchor, and roadmap items and task summaries are scope indicators, not detailed behavioral contracts, unless the project explicitly declares otherwise.
- Follow a source-priority hierarchy only when the user or project context declares one. When authoritative sources overlap, record whether they agree, one refines the other, or they conflict; when no declared hierarchy exists and an unresolved conflict would change the plan, ask the user and stop instead of silently choosing a source.
- Do not edit source requirements while planning. Report stale or contradictory requirement artifacts and route them to their owning command or the user.
- When behavior depends on two or more independent selectors, states, modes, or input shapes, enumerate the supported combinations before writing tasks; cover only combinations the requirements allow, not a speculative Cartesian product. For each material combination specify accepted input and validation, persisted state, returned output or next transition, side effects and invariants, and one verification scenario — a test only when testing is enabled, otherwise a runnable/manual verification step.
- When repository data, configuration, fixtures, schemas, or content define the real contract, plan at least one representative existing artifact through the primary verification path instead of relying only on synthetic fixtures.

Add an exact `## Requirements Reconciliation` section to `proposal.md` when multiple authoritative sources materially constrain the work, a conflict was resolved by an explicit user or project decision, independent behavior dimensions exist, or a representative real artifact is required. Keep it compact: declared authority, cited decisions with source paths, the applicable combination table, and verification evidence. Omit the complete heading and body for simple work where none of these signals applies; never emit a placeholder section. The heading is a fixed English compatibility token parsed by downstream workflow checks: do not translate or rename it, and preserve an existing section verbatim during replanning.

After preserving the raw request, normalize the task into these fields:

- task type
- goal
- non-goals
- constraints
- assumptions
- impacted capabilities
- C4 impact
- ADR candidates
- dependency graph
- acceptance criteria
- open questions
- suggested next command

Write the normalized task content into canonical OpenSpec artifacts:

- `proposal.md` for the exact OpenSpec source-template headings `## Why`, `## What Changes`, `## Capabilities`, and `## Impact`, plus the AIFHub raw-source and roadmap sections when applicable
- `design.md` for technical approach, C4 impact, ADR candidates, dependency graph, integration points, alternatives, and risks
- `tasks.md` for an executable implementation checklist
- `specs/**/spec.md` for behavior-changing requirements and scenarios

Do not create a separate task-preparation command or artifact in OpenSpec-native mode. `/aif-task-prepare`, `.ai-factory/specs/<task-id>.md`, `task-prepare.md`, and legacy companion files under `openspec/changes/<change-id>/` must not be created.

Raw input trace, normalization confidence, and temporary notes are runtime state only. They may be persisted only under `.ai-factory/state/<change-id>/` when needed and must never be written under `openspec/changes/<change-id>/`.

#### Revision-bound Research Context

When research Active Summary materially affects scope, tasks, constraints, or tradeoffs, resolve its exact source before writing `proposal.md`:

- Use the shared `resolveUltraResearchSource()` result from `scripts/ultra-research-resolver.mjs` for ultra research candidates. Consume its structured `source`, `revision`, `content`, and `diagnostic`; do not reimplement marker, path, status, Artifact Index, Active Summary, or digest checks in the prompt.
- Selection precedence is a safe explicit project-relative `RESEARCH.md` path, then an exact bundle slug, then exactly one caller-reviewed materially relevant active candidate. More than one relevant candidate stops with `ultra-research-ambiguous`; recency, fuzzy matching, and a newer `Updated` value never break the tie.
- Missing, unmarked, inactive, unsafe, symlinked, or invalid explicit sources stop Research Context creation with the resolver's bounded code. Invalid implicit candidates are ignored with bounded warnings and are never silently imported.
- For regular research, use the resolved `paths.research` Active Summary under the same normalization contract. For ultra research, use only `content.activeSummary` from the selected exact `source.path`; sibling C4, ADR, and dependency artifacts are rationale and cannot expand scope unless their conclusion is reflected in that Active Summary.

Commit only the selected relevant summary to `proposal.md` under `## Research Context`:

```text
Source: <exact selected RESEARCH.md source> (Active Summary, Updated: <timestamp>, SHA256: <digest>)
<copied Active Summary>
```

- Treat the embedded section as the committed requirements snapshot; do not copy the full `## Sessions` history into the proposal.
- Compute `SHA256` from the copied Active Summary after excluding markdown comments and the `Source` line, normalizing line endings to LF, removing trailing spaces from each line, preserving line order, and ending the digest input with exactly one newline.
- Once written, the snapshot and source revision remain immutable unless the user explicitly requests a research rebase through refinement.
- If the live research revision changes while planning is in progress, emit `WARN [research-drift] change-id=<change-id> source=<path> expected=<embedded revision> current=<live revision>` and keep the embedded snapshot authoritative. Do not log or duplicate the full research body, provider output, credentials, or other sensitive content in the warning.
- Live `## Sessions` may be consulted for rationale, but newer requirements must not silently expand committed scope.

#### Roadmap Linkage

Always include `## Roadmap Linkage` in every canonical OpenSpec proposal so downstream lifecycle commands can consume one stable shape. Use exactly these fields and keep them in this order:

```markdown
## Roadmap Linkage

- Issues: <comma-separated canonical HTTPS work-item URL(s) or stable MCP resource URI(s)|none>
- Milestone: <exact title|none>
- Roadmap item/slice: <exact item or slice|none>
- Rationale: <one bounded explanation|none>
```

- Capture only linkage explicitly supplied by the user or already committed in the selected planning source. Normalize every reference to a canonical provider HTTPS work-item URL or stable `mcp://` resource URI. GitHub issue references normalize to `https://github.com/<owner>/<repo>/issues/<number>` when the repository identity is directly available.
- For every missing value, write `none`. When the user explicitly supplies `none`, preserve an explicit `none` value verbatim. Do not omit a field whose value is `none`.
- Planning MUST NOT infer an issue or milestone from an issue title, branch name, repository labels, unrelated roadmap text, or another uncommitted contextual hint.
- If at least one of `Issues`, `Milestone`, or `Roadmap item/slice` is non-`none` and the configured roadmap should register the new change as `planned`, return the exact owner handoff `/aif-roadmap check`. Planning must not edit the roadmap itself.
- When all four fields are `none`, preserve the standardized section but do not return `/aif-roadmap check` solely for an all-`none` linkage.
- In the normal planning response, report only the captured linkage fields and the exact owner handoff when applicable. Do not print or summarize unrelated roadmap content.

#### Enabled optional tool use

- Before using any optional provider, call the installed wrapper when available: `ai-factory aifhub-memory-tools select --from-project --command aif-plan --json`.
- Use only entries returned in `selected_tools`. For each selected entry, follow its `tool_id`, `permission`, `execution`, `forbidden_operations`, `protected_artifacts`, read scope, purge path, and privacy caveat.
- Do not use tools that are absent from `selected_tools`, including tools listed in `not_selected_tools`, tools missing from config, tools forbidden for `/aif-plan`, or tools whose execution guidance is unavailable.
- If no optional provider is selected, continue with the rg baseline and direct repository/OpenSpec evidence.

#### Optional Graphify context

Graphify is optional supporting context for integration discovery before creating or refining a canonical OpenSpec change.

- This provider-specific boundary applies only when `selected_tools` includes Graphify, or when reading already reviewed Graphify output that exists in an allowed context path.
- `/aif-plan full` may read existing reviewed Graphify outputs such as `graphify-out/GRAPH_REPORT.md`, `graphify-out/graph.json`, `.ai-factory/references/graphify/GRAPH_REPORT.md`, and `.ai-factory/state/<change-id>/graphify/GRAPH_REPORT.md`.
- Missing Graphify or missing reports are degraded context, not planning failure.
- Do not install `graphifyy`, run `graphify`, add Graphify dependencies or manifest entries, or start/register Graphify MCP automatically.
- Treat extracted, inferred, ambiguous, or confidence-labeled Graphify relationships as hypotheses for direct repository inspection.
- Use Graphify output to identify possible integration points or impact areas only; final `proposal.md`, `design.md`, `tasks.md`, and delta specs must cite direct repository evidence and canonical OpenSpec/source context.
- Reviewed project-wide Graphify copies belong under `.ai-factory/references/graphify/`; reviewed change-scoped copies belong under `.ai-factory/state/<change-id>/graphify/`.
- Do not import raw Graphify output or generated files such as `GRAPH_REPORT.md`, `graph.json`, or `graph.html` into `openspec/changes/<change-id>/`, `openspec/specs/`, `.ai-factory/rules/generated/`, or `.ai-factory/qa/<change-id>/`.
- Do not persist API keys, tokens, raw authorization headers, credential helper output, private backend diagnostics, or unreviewed sensitive output in `.ai-factory/`, `openspec/`, docs, runtime state, QA evidence, generated rules, or Graphify reference copies.

#### Optional Context7 documentation context

Context7 is optional supporting documentation context for current library/API docs before creating or refining a canonical OpenSpec change.

- This provider-specific boundary applies only when `selected_tools` includes Context7, or when reading already reviewed Context7 notes that exist in an allowed context path.
- `/aif-plan full` may recommend that the user run Context7 manually outside AIFHub command ownership only when the selection output allows Context7, with commands such as `npx ctx7 library <name> <query>` and `npx ctx7 docs <libraryId> <query>`, or user-installed equivalents `ctx7 library <name> <query>` and `ctx7 docs <libraryId> <query>`.
- `/aif-plan full` may read reviewed Context7 notes under `.ai-factory/references/context7/` and `.ai-factory/state/<change-id>/context7/`.
- Missing Context7, missing Node.js runtime support, missing provider access, or missing reviewed notes is degraded context, not planning failure.
- If the user already configured Context7 MCP, available tools may include `resolve-library-id` plus a docs retrieval tool named `get-library-docs` or `query-docs`; use them only as optional read-only documentation context.
- Do not install `ctx7` or `@upstash/context7-mcp`, run `ctx7`, run `ctx7 setup`, add Context7 dependencies or manifest entries, add Context7 MCP templates to `extension.json`, mutate `.mcp.json`, `.cursor/mcp.json`, `.opencode.json`, agent rules, or agent skills, or start/register Context7 MCP automatically.
- Treat Context7 output as supporting context only; final `proposal.md`, `design.md`, `tasks.md`, and delta specs must remain source-grounded in direct repository evidence and canonical OpenSpec/source context.
- Reviewed project-wide Context7 notes belong under `.ai-factory/references/context7/`; reviewed change-scoped Context7 notes belong under `.ai-factory/state/<change-id>/context7/`.
- Do not import raw Context7 output, MCP transcripts, API responses, setup output, or generated provider configuration into `openspec/changes/<change-id>/`, `openspec/specs/`, `.ai-factory/rules/generated/`, or `.ai-factory/qa/<change-id>/`.
- Do not persist `CONTEXT7_API_KEY`, API keys, tokens, raw authorization headers, credential helper output, private provider diagnostics, private backend diagnostics, or unreviewed sensitive output in `.ai-factory/`, `openspec/`, docs, runtime state, QA evidence, generated rules, or Context7 reference copies.

#### Change ID policy

- When a primary MCP work-item identity resolved under the top-level source-bound contract, use `deriveSourceBoundChangeId(external_id, request_slug).changeId`.
- Otherwise derive a safe `<change-id>` slug from the request for new plans.
- Prefer lowercase kebab-case.
- Allow only safe relative IDs.
- Reject IDs containing `/`, `\`, `..`, absolute paths, path traversal, or unsafe characters.
- Use or reference `normalizeChangeId()` from `scripts/active-change-resolver.mjs` when useful.
- If `openspec/changes/<change-id>` already exists, do not overwrite silently. Apply the source-aware collision rules above for source-bound IDs; for ordinary IDs, ask for a new ID or create a deterministic suffix only in autonomous mode when asking is unavailable.

#### Ground planning in the target repository

Before selecting or changing a capability, inventory accepted specs under `openspec/specs/**`, separately from active changes. Reuse each existing capability's exact path, including nested directories. Read relevant accepted specs in full, including scenarios, before deciding what is covered. If using the optional CLI, `openspec list --specs` inventories specs; plain `openspec list` inventories changes and cannot substitute for it. Filesystem discovery remains sufficient when the CLI is unavailable; this does not add Store selection or initialization authority.

Before drafting `proposal.md`, `design.md`, `tasks.md`, or delta specs, read the relevant implementation, nearby tests, configuration, and project docs outside `openspec/`, alongside accepted specs and project context/rules. Keep this discovery read-only and proportional to the requested change; do it now rather than adding generic future tasks to explore the codebase.

Resolve the actual target repository when it differs from `planningHome.root`; ask for the target only when available context cannot identify it. For greenfield work, inspect the existing structure and setup docs. If source is unavailable, disclose the material limitation instead of inventing implementation facts. Distinguish observed behavior, assumptions, and proposed behavior, cite direct file evidence, and surface conflicts between accepted specs and current code for resolution before making a dependent design decision.

#### Required artifact shape

`proposal.md` should use:

```markdown
## Original Request

<verbatim explicit request; omit for research-only planning>

## Roadmap Linkage

- Issues: <comma-separated canonical HTTPS work-item URL(s) or stable MCP resource URI(s)|none>
- Milestone: <exact title|none>
- Roadmap item/slice: <exact item or slice|none>
- Rationale: <one bounded explanation|none>

## Why

Why this change is needed now.

## What Changes

- In-scope behavior or workflow changes
- Explicit non-goals and compatibility boundaries

## Capabilities

### New Capabilities

- `<capability-path>`: <new behavior contract; omit when none>

### Modified Capabilities

- `<existing-capability-path>`: <changed requirement; omit when none>

## Impact

Affected code, APIs, dependencies, systems, assumptions, risks, and open questions.

## Research Context

Source: <exact selected RESEARCH.md source> (Active Summary, Updated: <timestamp>, SHA256: <digest>)

<committed relevant Active Summary; omit when research did not shape the plan>

## Requirements Reconciliation

<declared authority, cited decisions, applicable combination table, and verification evidence; omit the complete section when no reconciliation signal applies>
```

For source-bound proposals only, insert this exact block immediately after `## Original Request` and before `## Roadmap Linkage`. For every ordinary plan, omit the complete heading and body; never emit placeholders, `none`, or an example binding.

```markdown
## AIFHub Source Binding

- Provider: <canonical lowercase provider or MCP server identifier>
- Primary source: <canonical HTTPS work-item URL or stable MCP resource URI>
- External ID: <provider's human-readable work-item key>
- Branch: <exact current git branch|none>
```

The source-template proposal headings are exact and case-sensitive: `## Why`, `## What Changes`, `## Capabilities`, `### New Capabilities`, `### Modified Capabilities`, and `## Impact`. Do not translate, rename, or replace them with older `Intent`, `Scope`, or `Approach` headings. AIFHub-owned `## Original Request`, conditional `## AIFHub Source Binding`, `## Roadmap Linkage`, and conditional `## Research Context` are additions; they do not replace the source headings. Conditional `## Requirements Reconciliation` is likewise an addition carrying the upstream reconciliation contract; its heading is a fixed compatibility token.

`design.md` should use:

```markdown
# Design: <Title>

## Technical Approach

## Data / Artifact Model

## Integration Points

## Alternatives Considered

## Risks
```

`tasks.md` must be a checkbox checklist:

- Every task checkbox MUST state how completion will be verified: a test, command, observable behavior, or delivered artifact.
- Keep that verification in the task description. Use a separate verification task only for broader integration or system behavior that spans multiple implementation tasks.

```markdown
# Tasks

## 1. Planning and artifacts

- [ ] 1.1 Create/update the OpenSpec proposal, design, tasks, and delta specs; verify the configured strict validation command accepts the complete artifact set

## 2. Implementation

- [ ] 2.1 Implement the scoped behavior; verify the focused regression passes
```

Delta specs must use OpenSpec requirement sections:

For a new capability, include a meaningful `## Purpose` in its delta before the first sync/archive; do not leave an opening `TBD`/`TODO` or the archive-generated placeholder. An existing main spec's Purpose is not replaced by a delta's Purpose. If strict validation reports that debt, record a bounded direct accepted-spec remediation in the change and preserve archived evidence; do not disguise it as a requirement delta.

```markdown
# Delta for <Capability>

## ADDED Requirements

### Requirement: <Requirement name>

The system MUST/SHALL ...

#### Scenario: <Scenario name>

- GIVEN ...
- WHEN ...
- THEN ...

## MODIFIED Requirements

### Requirement: <Existing requirement name>

...

## REMOVED Requirements

### Requirement: <Removed requirement name>
```

Every requirement should include at least one scenario when applicable.

#### Runtime state

Planning may create or update runtime state only under `.ai-factory/state/<change-id>/`.

Use or reference `ensureRuntimeLayout(changeId)` from `scripts/active-change-resolver.mjs` when runtime directories are needed.

Allowed runtime examples:

- `.ai-factory/state/<change-id>/plan-summary.md`
- `.ai-factory/state/<change-id>/validation.json`

Runtime QA output belongs under `.ai-factory/qa/<change-id>/`.

#### OpenSpec validation

Before validation, resolve the effective OpenSpec policy through:

```bash
ai-factory aifhub-mode status --json
```

Read `effectivePolicy` from the JSON output. When a compatible OpenSpec CLI is available, validate through `scripts/openspec-runner.mjs` using `validateOpenSpecChange(changeId)` or equivalent runner behavior.

The runner command corresponds to:

```bash
openspec validate <change-id> --type change --strict --json --no-interactive --no-color
```

- If validation passes, report success.
- If validation fails, repair generated artifacts when possible or clearly report the failing file, requirement, or section.
- Missing or unsupported OpenSpec CLI is degraded validation, not planning failure, unless the effective policy has `requireCliForPlan: true`; when required, treat the missing CLI as a planning blocker and do not claim OpenSpec validation passed.
- Do not install OpenSpec skills or slash commands.

Report generated OpenSpec artifact paths, effective policy summary, and validation status in the normal planning response. Persist validation evidence only under `.ai-factory/state/<change-id>/` when a runtime file is needed.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve the existing legacy companion plan-folder contract.

This mode is legacy AI Factory-only mode. The companion rules below apply only to classic `/aif-plan full` (and existing non-ultra default behavior). When `/aif-plan full` creates `.ai-factory/plans/<plan-id>.md`, it must also create and keep synchronized the companion folder:

- `.ai-factory/plans/<plan-id>/task.md`
- `.ai-factory/plans/<plan-id>/context.md`
- `.ai-factory/plans/<plan-id>/rules.md`
- `.ai-factory/plans/<plan-id>/verify.md`
- `.ai-factory/plans/<plan-id>/status.yaml`
- `.ai-factory/plans/<plan-id>/explore.md` when active research exists

Treat the plan file as the parent-compatible summary artifact and the folder as the structured execution/state artifact set.
The companion plan file may remain plain upstream markdown; the shared YAML frontmatter contract applies to the plan-folder markdown artifacts, not to the parent-compatible plan file.

For a source-bound classic plan, persist the exact Markdown source-binding section in `.ai-factory/plans/<plan-id>.md` and keep this exact mapping synchronized in `.ai-factory/plans/<plan-id>/status.yaml`. Create or explicitly rebind the Markdown and YAML values as one logical update, then require `parseSynchronizedWorkItemSourceBinding(markdown, status)` to pass before reporting success:

```yaml
source_binding:
  provider: "linear"
  primary_source: "mcp://linear/issue/<stable-record-id>"
  external_id: "ENG-431"
  branch: "<exact current git branch|none>"
```

For explicit `/aif-plan ultra`, first require the shared stable AI Factory `>=2.18.0` version gate, then leave creation and orchestration to the upstream skill. A valid upstream bundle is `<paths.plans>/<plan-id>/index.md` plus direct `phase-NN-<slug>.md` files with the exact standalone ultra marker, and `index.md` is its sole progress ledger. When its ID is source-bound, require the exact Markdown source-binding section in `index.md` without changing this ownership boundary. Do not create a sibling `<plan-id>.md`; do not create or synchronize `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, or `explore.md`; and do not add AIFHub checkboxes to phase files.

### Research Normalization

- In OpenSpec-native mode, do not import research into `openspec/changes/<change-id>/` as runtime-only notes.
- In legacy AI Factory-only mode, if `.ai-factory/RESEARCH.md` exists, normalize the active summary into plan-local `explore.md`.
- Keep `.ai-factory/RESEARCH.md` read-only.
- In legacy AI Factory-only mode, record the imported source and timestamp in `status.yaml.history`.

### Plan Resolution and Migration

- In OpenSpec-native mode, use OpenSpec change IDs and the shared active-change vocabulary.
- In legacy AI Factory-only mode, when the active branch slug matches an existing plan folder without a companion `.md` plan file, generate the missing plan file before continuing.
- In legacy AI Factory-only mode, record legacy upgrades in `status.yaml.history` with the source folder path and the generated companion plan file path.
- User-facing guidance must present the mode-appropriate canonical artifacts, not a mixed OpenSpec/legacy shape.

### Handoff Rules

- After planning, route the next step to `/aif-improve`.
- Do not mention deprecated orchestration or finalize aliases as active workflow steps.

### Codex Runtime

When running in Codex app/CLI:

- The planning stage (`/aif-plan full`, `/aif-improve`) should run in Codex Plan mode when structured clarifying questions are needed.
- This skill may recommend Plan mode, but it does not attempt or promise to switch the Codex session mode. The user controls the mode.
- In Codex Plan mode, use `request_user_input` only for 1-3 short questions.
- In Codex Default mode, if a question is needed, ask it as plain text in the assistant message. Do not use `question(...)`, `questionnaire(...)`, or `request_user_input`.
- If another CLI or IDE runtime exposes a planning mode, use that available planning-mode mechanism for structured planning questions; do not fabricate unavailable tools or client actions.
- In autonomous or subagent mode, do not ask interactive questions. Record assumptions and return blockers/open questions to the parent.
- See `skills/shared/QUESTION-TOOL.md` for the full runtime question format mapping.
<!-- aif-ext:aifhub-extension:aif-plan:prepend:end -->

# Plan - Implementation Planning

Create an implementation plan for a feature or task. Three modes:

- **Fast** – quick plan, no git branch, saves to the configured fast plan path (default: `.ai-factory/PLAN.md`)
- **Full** — richer plan, asks preferences, saves to the configured full-plan directory, and optionally creates a git branch/worktree when git is enabled and branch creation is allowed
- **Ultra** — exhaustive multi-file plan bundle under the configured full-plan directory: `index.md` is the manifest/progress ledger and every implementation phase is a separate, deeply specified markdown file. Use it when planning with a stronger model for later execution by a smaller model.

For ultra layout, detail requirements, integrity checks, and consumer behavior,
read `references/ULTRA-FORMAT.md` before creating or modifying the bundle.

## Workflow

### Step 0 (pre): Detect Handoff Mode

Determine Handoff mode, task ID, and branch contract. Resolve each value independently so legacy callers that pass only `HANDOFF_MODE` and `HANDOFF_TASK_ID` still enter Handoff mode correctly:

- `HANDOFF_MODE`: explicit prompt value if present; otherwise environment value; otherwise empty string.
- `HANDOFF_TASK_ID`: explicit prompt value if present; otherwise environment value; otherwise empty string.
- `HANDOFF_BRANCH_PREPARED`: explicit prompt value if present; otherwise environment value; otherwise `0`.
- `HANDOFF_BRANCH_NAME`: explicit prompt value if present; otherwise environment value; otherwise empty string.

Use the Bash tool only for values that were not passed explicitly in the prompt:

```
Bash: printenv HANDOFF_MODE || true
Bash: printenv HANDOFF_TASK_ID || true
Bash: printenv HANDOFF_BRANCH_PREPARED || true
Bash: printenv HANDOFF_BRANCH_NAME || true
```

**Then check `HANDOFF_MODE`:**

#### When `HANDOFF_MODE` is `1` (autonomous Handoff agent)

The Handoff coordinator already manages status transitions and DB writes directly. Do NOT call MCP tools (`handoff_sync_status`, `handoff_push_plan`). Instead:

- **No interactive questions:** Do not use `AskUserQuestion` — use sensible defaults (verbose logging, yes to tests, yes to docs, skip roadmap linkage).
- **Mode default:** If mode is not specified, default to `fast`.
- **Plan annotation (MANDATORY):** If `HANDOFF_TASK_ID` is non-empty, you MUST insert `<!-- handoff:task:<HANDOFF_TASK_ID> -->` as the very first line of the plan entrypoint (`index.md` for ultra; the plan file otherwise), before the title. This annotation links the plan to its Handoff task for bidirectional sync. **Omitting this annotation when HANDOFF_TASK_ID is set is a bug — verify before completing.**

##### Branch ownership under Handoff (CRITICAL)

Handoff owns branch creation at the agent-code level. The skill must NOT create or switch branches when Handoff has prepared one. Apply these rules:

**If `HANDOFF_BRANCH_PREPARED` is `1`:**

- Do **NOT** execute `git checkout`, `git pull`, or `git checkout -b`.
- Treat `--parallel` as disabled for all downstream behavior.
- Do **NOT** create a worktree.
- Read `HANDOFF_BRANCH_NAME` from the prompt / env.
- Validate strict equality:
  ```
  Bash: git rev-parse --abbrev-ref HEAD
  ```
  The output must equal `HANDOFF_BRANCH_NAME` exactly. Do **not** accept partial matches, prefix matches, or "branch contains `/`" heuristics.
- If the current branch does **not** match `HANDOFF_BRANCH_NAME`, STOP. Report a blocker in the plan summary:
  > `Branch drift: expected <HANDOFF_BRANCH_NAME>, actual <current>.`
  Do **NOT** "fix" drift by switching or creating a branch — Handoff classifies that as `BranchIsolationError` / `blocked_external`.
- Use `HANDOFF_BRANCH_NAME` (with `/` replaced by `-`) as the full/ultra plan identifier stem. Full writes `<configured plans dir>/<stem>.md`; ultra writes `<configured plans dir>/<stem>/index.md`. Skip the slug derivation in Step 1.2.

**If `HANDOFF_MODE` is `1` but `HANDOFF_BRANCH_PREPARED` is unset or `0`:**

- Fallback path for older Handoff clients that have not adopted the prepared-branch contract.
- Execute Step 1.4 branch creation normally per `git.create_branches` config.

#### When `HANDOFF_MODE` is NOT `1` (manual Claude Code session)

If polishing an existing plan, extract the Handoff task ID from the `<!-- handoff:task:<id> -->` annotation on the first line of the plan entrypoint (if present). If creating a new plan and no annotation context exists, skip all MCP sync — there is no linked Handoff task.

If a task ID IS found in the plan annotation, sync with Handoff via MCP tools:

- **On start:** Call `handoff_sync_status` with `{ taskId: <extracted-id>, newStatus: "planning", sourceTimestamp: "<current UTC time in ISO 8601 format>", direction: "aif_to_handoff", paused: true }`.
- **On completion:** Call `handoff_push_plan` with `{ taskId: <extracted-id>, planContent: <full plan text> }`. For ultra, serialize the bundle as `index.md` followed by every Phase Index file in order, each prefixed with `<!-- ultra-phase:<relative-path> -->`. Then call `handoff_sync_status` with `{ taskId: <extracted-id>, newStatus: "plan_ready", sourceTimestamp: "<current UTC time in ISO 8601 format>", direction: "aif_to_handoff", paused: true }`.

**CRITICAL:** Always pass `paused: true` with every `handoff_sync_status` call except `done`. This prevents the autonomous Handoff agent from picking up the task while you work manually. Only `done` passes `paused: false`.

Preserve the `<!-- handoff:task:<id> -->` annotation on the first line when rewriting the plan entrypoint.

### Step 0: Load Project Context

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:

- **Paths:** `paths.description`, `paths.architecture`, `paths.roadmap`, `paths.research`, `paths.rules_file`, `paths.plan`, `paths.plans`, `paths.patches`, `paths.evolutions`, `paths.specs`, `paths.rules`, and `paths.archive`
  - Derive `research_bundles_dir = <parent directory of paths.research>/research/` for opt-in ultra research bundles. No additional config key is required.
- **Language:** `language.ui` for AskUserQuestion prompts, `language.artifacts` for generated plan files, and `language.technical_terms` for human-readable technical terminology in plan artifacts
- **Git:** `git.enabled`, `git.base_branch`, `git.create_branches`, and `git.branch_prefix`
- **Workflow:** `workflow.plan_id_format` — controls the full/ultra plan identifier shape. Allowed values: `slug` (default), `timestamp`, `uuid`, `sequential`. Only `slug` and `sequential` are active; `timestamp` and `uuid` are **reserved** and currently behave like `slug` (with an `INFO` log). The `sequential` value writes a full plan as `<NNNN>_<plan_file_stem>.md` and an ultra bundle as `<NNNN>_<plan_file_stem>/index.md` (see Step 1.2 for the canonical stem and algorithm). Treat any unknown value as `slug` and emit `WARN [aif-plan] unknown workflow.plan_id_format=<value>; falling back to slug`.

If config.yaml doesn't exist, use defaults:

- Paths: `.ai-factory/` for all artifacts
- `ui_language`: `en`
- `artifact_language`: `en`
- `technical_terms_policy`: `keep`
- Git: `enabled: true`, `base_branch: main`, `create_branches: true`, `branch_prefix: feature/`
- Workflow: `plan_id_format: slug`

Resolved language values:
- `ui_language = language.ui || "en"`
- `artifact_language = language.artifacts || language.ui || "en"`
- `technical_terms_policy = language.technical_terms || "keep"`

If `technical_terms_policy` is not one of `keep`, `translate`, or `mixed`, treat it as `keep`. Legacy values such as `english` also behave like `keep`.

All AskUserQuestion prompts, progress updates, summaries, and next-step guidance MUST be written in `ui_language`.

Generated plan artifacts under `paths.plan` or `paths.plans` MUST be written in `artifact_language`.
For ultra this applies to `index.md` and every linked phase file.

Templates and examples define structure, not fixed English output. If `artifact_language` is not `en`, translate human-readable headings, labels, task prose, roadmap rationale, research summaries, settings explanations, and dependency notes before saving. Preserve markdown structure, checkbox syntax, task IDs, branch names, commit messages, commands, file paths, config keys, package names, API names, `WARN`/`INFO` labels, raw errors, and the exact ultra marker `<!-- aif:plan-mode:ultra -->` unchanged. Keep `## Research Context`, `Source:`, `Active Summary`, `Updated:`, and `SHA256:` exact because downstream research drift checks parse them as compatibility tokens. Keep `## Requirements Reconciliation` exact because downstream workflow checks parse it as a compatibility token. Apply `technical_terms_policy` to other human-readable terminology.

Exception: the section heading and body of `## Original Request` are fixed raw-source structure and must not be translated, summarized, normalized, or rewritten.

**THEN:** Read `.ai-factory/DESCRIPTION.md` (use path from config) if it exists to understand:

- Tech stack (language, framework, database, ORM)
- Project architecture
- Coding conventions
- Non-functional requirements

**ALSO:** Read the resolved architecture artifact if it exists (`paths.architecture`, default: `.ai-factory/ARCHITECTURE.md`) to understand:

- Chosen architecture pattern
- Folder structure conventions
- Layer/module boundaries
- Dependency rules

Use this context when:

- Exploring codebase (know what patterns to look for)
- Writing task descriptions (use correct technologies)
- Planning file structure (follow project conventions)
- **Follow architecture guidelines from the resolved architecture artifact when planning file structure and task organization**

**ALSO:** Read the resolved rules hierarchy when present:

1. `paths.rules_file` — project-wide axioms
2. `rules.base` — base project conventions
3. named `rules.<area>` entries relevant to the requested work

Treat explicit rules as authoritative requirements. More-specific area rules
override base rules, which override project-wide axioms.

**Read `.ai-factory/skill-context/aif-plan/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**

- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the PLAN.md
  template, ultra bundle, and task format. The templates from TASK-FORMAT.md and
  ULTRA-FORMAT.md are a **base structure**. If a
  skill-context rule says "tasks MUST include X" or "plan MUST have section Y" — you MUST augment
  the template accordingly. Generating a plan that violates skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

**OPTIONAL (recommended):** Read the resolved roadmap artifact if it exists (`paths.roadmap`, default: `.ai-factory/ROADMAP.md`):

- Use it to link this plan to a specific milestone (when applicable)
- This reduces ambiguity in `$aif-implement` milestone completion and `$aif-verify` roadmap gates

**OPTIONAL (recommended):** Resolve at most one relevant research source:

- A legacy source is the configured `paths.research` file.
- An ultra source is `research_bundles_dir/<english-slug>/RESEARCH.md`, but only when the sibling `INDEX.md` contains `<!-- aif:research-mode:ultra -->` exactly once and its `## Artifact Index` links that `RESEARCH.md`. Do not treat arbitrary directories as research bundles.
- When the user supplied a topic or request, source priority is: an explicitly referenced bundle/file; then one clearly topic-matching marked `Status: active` bundle; then the relevant legacy file. An explicit source means a concrete `RESEARCH.md` or bundle path named in the request or follow-up; it is user content, not a stripped command token.
- Resolve a topic match by exact English slug first, then a normalized `Topic:` match, then a unique semantic match against `Purpose` / Active Summary. Never pick by recency. If no single match is clear or multiple sources would change scope, ask instead of merging or guessing. An explicitly referenced `paused` or `superseded` bundle requires a warning before use.
- Read an ultra `INDEX.md` first, then its linked `RESEARCH.md`. Read C4/ADR/dependency artifacts only for rationale needed by this plan. They are not independent requirement sources; material conclusions must already be reflected in the Active Summary.
- Store the chosen file as `selected_research_path`. Treat its `## Active Summary (input for $aif-plan)` as an additional requirements source.
- Carry over constraints/decisions into tasks and plan settings.
- Prefer the summary over raw notes; use `## Sessions` and linked optional artifacts only when you need deeper rationale.
- No-description fallback is the deliberate backward-compatibility exception to the topic-based priority: use the configured `paths.research` Active Summary topic when available. Otherwise use a single active marked ultra bundle; if several active bundles exist, ask the user to choose rather than guessing.
- Track whether research content influenced this plan. Set `research_influenced_plan = true` only when the Active Summary supplies the default description or when constraints, decisions, goals, open questions, or session rationale from the research artifact shape the plan scope, tasks, settings, or tradeoffs. If the research artifact exists but is stale or unrelated to the user's requested task, leave `research_influenced_plan = false`, ignore it for plan requirements, and do not add `## Research Context`.
- If any research content influences the plan, the generated plan MUST include `## Research Context` with canonical ``Source: `<selected_research_path>` (Active Summary, Updated: <timestamp>, SHA256: <digest>)`` metadata. Omitting this plan-owned research copy is a bug because downstream skills treat the embedded Research Context as the plan's authoritative requirements and use the source research file only for drift checks.
- Normalize the copied Active Summary before hashing: include exactly the text that will be pasted under `## Research Context` after the `Source:` line, remove HTML comment blocks, preserve line order and leading whitespace, trim trailing spaces from every line, use LF line endings, and end with exactly one final newline. Calculate the digest without writing any temporary file or repository artifact: feed the normalized text through stdin / inline shell input to `shasum -a 256`; if `shasum` is unavailable, feed the same normalized text to `sha256sum`. Use the first output field as the `SHA256:` value.

### Step 0.1: Resolve Git State

Do **not** auto-run `git init`.

Resolve the current git mode from config first:

- `git.enabled: true` → git-aware workflow is allowed
- `git.enabled: false` → no-git workflow only
- `git.base_branch` → target branch for diffs/merge guidance (default: detected branch or `main`)
- `git.create_branches: true` → full/ultra mode may create a branch/worktree
- `git.create_branches: false` → full/ultra mode still creates its plan artifact, but stays on the current branch / repository state

If `git.enabled = false`:

- Skip all branch/worktree commands
- Save full plans under `paths.plans/<slug>.md` and ultra bundles under `paths.plans/<slug>/index.md`
- Treat `--parallel`, `--list`, and `--cleanup` as unavailable

If `git.enabled = true` but the repository is not actually inside a git work tree:

- Warn the user that git-aware actions are unavailable until the repository is initialized
- Fall back to the same no-git behavior as above

### Step 0.2: Parse Arguments & Select Mode

Extract flags and mode from `$ARGUMENTS`:

```
--parallel  → Enable parallel worktree mode (full/ultra only; requires `git.enabled=true` and `git.create_branches=true`)
--list      → Show all active worktrees, then STOP (git-only)
--cleanup <branch> → Remove worktree and optionally delete branch, then STOP (git-only)
fast        → Fast mode (first word)
full        → Full mode (first word)
ultra       → Ultra mode (first word)
```

**Parsing rules:**

- Strip only recognized command tokens in command positions from `$ARGUMENTS`:
  - `fast`, `full`, or `ultra` only when used as the leading mode token
  - recognized control flags `--parallel`, `--list`, and `--cleanup <branch>`
  - do not remove matching words inside the user's actual request text
- Remaining text becomes the description
- Preserve the remaining text as `original_user_request` when it is non-empty: trim only outer whitespace introduced by command parsing, but keep internal whitespace, line breaks, wording, casing, and punctuation exactly. This is the user's original planning request and MUST be saved into the plan entrypoint later.
- `--list` and `--cleanup` execute immediately and **STOP** (do NOT continue to Step 1+)
- If `git.enabled = false`, reject `--parallel`, `--list`, and `--cleanup` with a short explanation instead of trying git commands
- If `--parallel` is set while `git.create_branches = false`, reject it with a short explanation because parallel mode requires branch creation

**If the description is empty:**

- If the configured `paths.research` file exists and its Active Summary has a non-empty `Topic:`, default the description to that topic (no extra user input required), set it as `selected_research_path`, and leave `original_user_request` empty.
- Otherwise, if exactly one marked active ultra bundle exists and its linked `RESEARCH.md` has a non-empty `Topic:`, use that topic and file. If multiple marked active bundles exist, ask the user to select a topic/source.
- Plans created from a selected `RESEARCH.md` without an explicit user request MUST NOT include an `Original Request` section.
- Otherwise, ask the user for a short feature description. Preserve the user's answer verbatim as `original_user_request` and save it into the plan entrypoint later.

**Original request contract:**

- If the user explicitly supplied a planning request (for example `$aif-plan ТУТ ЗАПРОС НА ПЛАН`, `$aif-plan full ТУТ ЗАПРОС НА ПЛАН`, `$aif-plan ultra ТУТ ЗАПРОС НА ПЛАН`, or an answer to the description prompt), the generated plan entrypoint MUST include `## Original Request`.
- `## Original Request` contains the exact user-provided request text after only recognized command tokens are removed and only outer whitespace is trimmed. Do not rewrite, summarize, translate, or normalize its wording, even when `artifact_language` differs.
- If the description was derived only from a selected `RESEARCH.md` because the user did not provide a request, omit `## Original Request`; the committed source is `## Research Context` instead.
- If the user supplied a request and selected research also influenced the plan, include both `## Original Request` and `## Research Context`.

**If `--list` is present**, jump to [--list Subcommand](#--list-subcommand).
**If `--cleanup` is present**, jump to [--cleanup Subcommand](#--cleanup-subcommand).

**Mode selection:**

- `fast` keyword → fast mode
- `full` keyword → full mode
- `ultra` keyword → ultra mode
- Neither → preserve the pre-ultra interactive contract and ask only between
  full and fast. Ultra is strictly opt-in and is selected only by the explicit
  leading `ultra` mode token:

```
AskUserQuestion: Which planning mode?

Options:
1. Full (Recommended) — richer plan, asks preferences, optional branch/worktree flow when git settings allow it
2. Fast – quick plan, no branch, saves to the resolved fast plan path
```

If the user did not provide a description and a research source was selected:

- Mention that you will default the description to the `Active Summary` topic
- Only ask for `full` vs `fast` (no description prompt needed)
- Do not mention, recommend, or auto-select ultra unless the caller used the
  explicit leading `ultra` token

For concrete parsing examples and expected behavior per command shape, read `references/EXAMPLES.md` (Argument Parsing).

---

## Full and Ultra Modes

### Step 1: Parse Description & Reconnaissance

From the description, extract:

- Core functionality being added
- Key domain terms
- Type (feature, enhancement, fix, refactor)

**Use `Task` tool with `subagent_type: Explore` to understand the relevant parts of the codebase.** This runs as a subagent and keeps the main context clean.

Based on the parsed description, launch 1-2 Explore agents in parallel:

```
Task(subagent_type: Explore, model: sonnet, prompt:
  "In [project root], find files and modules related to [feature domain keywords].
   Report: key directories, relevant files, existing patterns, integration points.
   Thoroughness: quick. Be concise — return a structured summary, not file contents.")
```

**Rules:**

- Full: 1-2 agents max, "quick" thoroughness — this is reconnaissance, not deep analysis
- Ultra: 2-3 focused agents when available; cover architecture, existing patterns, integration/side effects, and test/operations surfaces. This reconnaissance feeds the deeper evidence required by `ULTRA-FORMAT.md`.
- Deep exploration happens later in Step 3
- If `.ai-factory/DESCRIPTION.md` already provides sufficient context, this step can be skipped

### Step 1.2: Generate Full/Ultra Plan Identifier

This step produces two distinct values:

- `branch_name` — the git branch (only when `git.enabled = true` and `git.create_branches = true`)
- `plan_file_stem` — the canonical unprefixed plan stem under `<configured plans dir>/`
- `plan_identifier` — `plan_file_stem` with an optional `NNNN_` prefix; it becomes the full-plan filename stem or ultra directory name

These are derived in a fixed order so the producer here and the branch-based consumers in `$aif-implement` / `$aif-improve` / `$aif-verify` / `$aif-rules-check` always agree on either the full-plan file or ultra entrypoint.

#### 1.2.a — Resolve the canonical `plan_file_stem`

Pick the first matching case:

1. **`HANDOFF_BRANCH_PREPARED = 1`** → `plan_file_stem = HANDOFF_BRANCH_NAME` with every `/` replaced by `-`. Skip slug generation entirely. No `branch_name` is created here (Handoff already owns the branch).
2. **`git.enabled = true` AND `git.create_branches = true`** → generate a description slug, then `branch_name = <git.branch_prefix><slug>` (default prefix: `feature/`). Set `plan_file_stem = branch_name` with every `/` replaced by `-` (for example `feature-user-authentication`).
3. **Otherwise** (`git.enabled = false` OR `git.create_branches = false`) → `plan_file_stem = <description slug>`. No `branch_name` is created.

Slug rules (cases 2 and 3):

- Lowercase, hyphen-separated, max 50 characters
- No special characters except hyphens
- Descriptive but concise

Branch examples (case 2):

- `feature/user-authentication`
- `fix/cart-total-calculation`
- `refactor/api-error-handling`
- `chore/upgrade-dependencies`

**Invariant:** branch-based consumer skills compute their lookup stem as `current-branch-with-slashes-replaced`. Cases 1 and 2 above already match that. Case 3 never has a branch, so consumers fall back to the lone full/ultra plan artifact in `<configured plans dir>/` (see `aif-implement` Step 0.2). Producing a `plan_file_stem` outside these rules breaks discovery.

#### 1.2.b — Apply the `workflow.plan_id_format` prefix

Default: no prefix. Set `plan_identifier = plan_file_stem`. Full writes
`<configured plans dir>/<plan_identifier>.md`; ultra writes
`<configured plans dir>/<plan_identifier>/index.md`.

Format-specific handling:

- `slug` (default) → no prefix.
- `timestamp` / `uuid` → **reserved values; treat as `slug` for now.** Emit `INFO [aif-plan] workflow.plan_id_format=<value> is reserved and behaves like slug; numbering is not applied`. Do NOT invent a stem shape — branch-based consumers do not know how to discover non-`sequential` prefixes.
- Unknown values → already handled in Step 0: emit `WARN [aif-plan] unknown workflow.plan_id_format=<value>; falling back to slug`. Behaves like `slug` here.
- `sequential` → apply the algorithm in 1.2.c to `plan_file_stem` to produce
  `plan_identifier`.

Sequential is **force-disabled** when `HANDOFF_BRANCH_PREPARED = 1`. In that case keep the bare `plan_file_stem` and emit `INFO [aif-plan] sequential numbering disabled under HANDOFF_BRANCH_PREPARED=1`.

#### 1.2.c — Sequential numbering algorithm

Prepend a 4-digit numeric prefix to `plan_file_stem` to produce
`plan_identifier`. Compute the prefix from existing numbered full-plan files and
numbered directories whose `index.md` contains the exact ultra marker in
`<configured plans dir>`. The branch name (when one exists) stays unchanged so
existing git tooling, CI, and PR conventions are unaffected.

```
1. Find existing numbered plans in <configured plans dir>:
     Glob A: <configured plans dir>/[0-9][0-9][0-9][0-9]_*.md
     Glob B: <configured plans dir>/[0-9][0-9][0-9][0-9]_*/index.md
2. Every Glob A match is a numbered full-plan candidate.
   For every Glob B match, Read index.md and keep it only when it contains the
   exact marker <!-- aif:plan-mode:ultra -->. Ignore numbered directories whose
   index.md lacks the marker; they are not plans and cannot consume an ID.
3. Parse the leading 4 digits from each retained full-plan filename or marked
   ultra directory
   basename into an integer. Deduplicate equal prefixes.
   Filter out entries whose relevant basename does not match
   ^[0-9]{4}_.+(\.md)?$.
4. If any matches exist:
     max_existing = max(prefixes)
     If max_existing >= 9999:
       ABORT with error:
         "sequential cap reached: a plan numbered 9999 already exists in <configured plans dir>."
         "Switch workflow.plan_id_format back to slug, or move the 9999-numbered file out of the directory (note: doing so will free 9999 for the next plan to reuse)."
     next = max_existing + 1
   Else:
     next = 1
5. prefix = zero-padded 4-digit string of next   (e.g. 1 → "0001", 42 → "0042")
6. Set plan_identifier = <prefix>_<plan_file_stem>
7. Final artifact:
     full  → <configured plans dir>/<plan_identifier>.md
     ultra → <configured plans dir>/<plan_identifier>/index.md
```

Implementation notes:

- **Use `Glob` to enumerate candidates and `Read` to validate every numbered
  ultra candidate's marker.** Do NOT shell out to `ls` — `aif-plan`'s
  frontmatter does not grant `Bash(ls *)`, so the `ls` path would fail in production.
- The 4-digit `[0-9][0-9][0-9][0-9]` glob is **strict by contract**: the format supports `0001`..`9999` only. The error in step 4 enforces this.
- **`--parallel` scope (TL;DR — source-worktree scoped):**
  - **Where the prefix is computed:** the source worktree's `<configured plans dir>`
    (the repo where `$aif-plan` was invoked) — i.e. exactly here, in Step 1.2.c.
  - **When it is computed:** **before** the optional `cd <WORKTREE>` in Step 1.4.
  - **Where the plan artifact is written:** the same relative full file or ultra
    directory path inside the target worktree, so the prefix and destination
    directory stay consistent.
  - **What you must NOT do:** never recompute the prefix from the target worktree's
    plans dir after `cd <WORKTREE>`. The target dir is typically empty and would
    re-allocate `0001` on every parallel run, breaking the cross-worktree numbering
    contract on merge.

Rules:

- Numbering is **derived from existing full-plan files and marked ultra directories** in `<configured plans dir>`. Unmarked numbered directories are ignored. Deleting or moving a numbered plan out of the directory can free that number for reuse on the next run — keep plans in place if you rely on stable cross-references.
- **Archived plans are excluded from numbering.** Plans moved to `paths.archive/plans/` by `$aif-archive` are not in `<configured plans dir>` and therefore not counted. Archiving the highest-numbered plan frees that number for reuse.
- Numbering is **bounded** — 9999 is a hard cap; the algorithm errors instead of writing `10000_…` so consumer globs (also 4-digit) cannot drift out of contract.
- The prefix lives only on the full-plan filename or ultra directory. The git branch (when present) stays `<branch_prefix><slug>` without a number.
- This setting is ignored for fast plans (`paths.plan` is a single file) and fix plans (`paths.fix_plan` is a single file).

Logging: `INFO [aif-plan] resolved plan artifact: <path> (mode=<mode>, format=<value>)`.

### Step 1.3: Ask About Preferences

**IMPORTANT: Always ask the user before proceeding:**

```
AskUserQuestion: Before we start, a few questions:

1. Should I write tests for this feature?
   a. Yes, write tests
   b. No, skip tests

2. Logging level for implementation:
   a. Verbose (recommended) - detailed DEBUG logs for development
   b. Standard - INFO level, key events only
   c. Minimal - only WARN/ERROR

3. Documentation policy after implementation?
   a. Yes — mandatory docs checkpoint at completion (recommended)
   b. No — warn-only (`WARN [docs]`), no mandatory checkpoint

4. Roadmap milestone linkage (only if the resolved roadmap artifact exists):
   a. Link this plan to a milestone
   b. Skip — no linkage (allowed; `$aif-verify --strict` should report WARN, not fail, for missing linkage alone)

5. Any specific requirements or constraints?
```

**Default to verbose logging.** AI-generated code benefits greatly from extensive logging because:

- Subtle bugs are common and hard to trace without logs
- Users can always remove logs later
- Missing logs during development wastes debugging time

Store all preferences — they will be used in the plan entrypoint and passed to `$aif-implement`.

Docs policy semantics:

- `Docs: yes` → `$aif-implement` MUST show a mandatory documentation checkpoint and route docs changes through `$aif-docs`
- `Docs: no` (or unset) → `$aif-implement` emits `WARN [docs]` and continues without a mandatory docs checkpoint

**If the resolved roadmap artifact exists and the user chose milestone linkage:**

- Read the resolved roadmap artifact and list candidate milestones (prefer unchecked items)
- Ask the user to pick one milestone (or type a custom one)
- Store the selected milestone name and a 1-sentence rationale for inclusion in the plan entrypoint

### Step 1.4: Optional Branch / Worktree Setup

**If `HANDOFF_BRANCH_PREPARED = 1` (Handoff owns the branch):**

- Skip this entire step. Branch validation already happened in Step 0.
- The plan artifact path uses `HANDOFF_BRANCH_NAME` (slashes replaced by `-`) as the stem.
- Do **NOT** run `git checkout`, `git pull`, `git checkout -b`, or `git worktree add`.
- Treat `--parallel` as disabled: do not create a worktree and do not auto-invoke `$aif-implement`.

**If `git.enabled = false` or `git.create_branches = false`:**

- Skip all branch/worktree creation
- Continue with the generated full file or ultra directory under `paths.plans`

**If `--parallel` flag is set → create worktree:**

> **Sequential prefix is already locked in.** Step 1.2.c computed the `NNNN_`
> prefix from the source worktree's `<configured plans dir>` before this step.
> Do NOT recompute it after `cd <WORKTREE>` — the target worktree's plans dir
> is typically empty and would re-allocate `0001`, breaking the numbering
> contract on merge.

#### Worktree Creation

```bash
DIRNAME=$(basename "$(pwd)")
git branch <branch-name> <configured-base-branch>
git worktree add ../${DIRNAME}-<branch-name-with-hyphens> <branch-name>
```

Convert branch name for directory: replace `/` with `-`.

**Example:**

```
Project dir: my-project
Branch: feature/user-auth
Worktree: ../my-project-feature-user-auth
```

Copy context files so the worktree has full AI context:

- Create the parent directories for the resolved DESCRIPTION, ARCHITECTURE, RESEARCH, plan, patch, and evolution paths inside the worktree.
- Copy the resolved DESCRIPTION, ARCHITECTURE, and RESEARCH artifacts into the same configured relative locations inside the worktree.
- Copy `.ai-factory/skill-context/` as-is into the worktree.
- Copy only the latest 10 patch files from the resolved `paths.patches` directory into the same configured relative path inside the worktree.
- Do **not** copy `patch-cursor.json` when you copied only a truncated patch set; that cursor is valid only with the full patch history.
- Copy agent settings (for example `.claude/`) and untracked `CLAUDE.md` when present.

Create changes directory and switch:

```bash
cd "${WORKTREE}"
```

Display confirmation:

```
Parallel worktree created!

  Branch:    <branch-name>
  Directory: <worktree-path>

To manage worktrees later:
  $aif-plan --list
  $aif-plan --cleanup <branch-name>
```

Continue to Step 2.

**If no `--parallel` → create branch normally:**

```bash
git checkout <configured-base-branch>
git pull origin <configured-base-branch>
git checkout -b <branch-name>
```

If branch already exists, ask user:

- Switch to existing branch?
- Create with different name?

---

## Ultra Mode Detail Contract

Ultra uses the full-mode preferences and optional branch/worktree setup above,
then applies a stricter planning gate:

1. Read `references/ULTRA-FORMAT.md` completely.
2. Resolve all cross-cutting decisions that an implementer would otherwise have
   to infer: file placement, public interfaces, data/control flow, compatibility,
   migrations, failure behavior, observability, tests, docs, and rollout where
   applicable.
3. Partition the work into dependency-ordered phases. A phase must be a coherent
   implementation checkpoint, not merely a category heading.
4. Create one phase markdown file per phase. Every task section must satisfy the
   Required Detail Gate in `ULTRA-FORMAT.md`.
5. Create `index.md` last, after phase contents are stable, so its Phase Index,
   task links, dependencies, and commit groups exactly match the phase files.
6. Run the bundle-integrity checks from `ULTRA-FORMAT.md` before presenting the
   plan. Fix broken links, missing/duplicate task IDs, orphan phase files, and
   inconsistent dependencies.

Ultra must not defer material implementation decisions to the smaller model.
If evidence is insufficient for a safe decision, record a blocking open question
in `index.md` and stop the plan as not implementation-ready instead of hiding the
gap behind vague instructions.

---

## Fast Mode

### Step 1: Ask About Preferences

Ask a shorter set of questions:

```
AskUserQuestion: Before we start:

1. Should I include tests in the plan?
   a. Yes, include tests
   b. No, skip tests

2. Any specific requirements or constraints?

3. Roadmap milestone linkage (only if the resolved roadmap artifact exists):
   a. Link this plan to a milestone
   b. Skip — no linkage (allowed; `$aif-verify --strict` should report WARN, not fail, for missing linkage alone)
```

**Plan file:** Always the resolved `paths.plan` file (default: `.ai-factory/PLAN.md`).

---

## Shared Steps (all modes)

### Step 2: Analyze Requirements

From the description, identify:

- Core functionality to implement
- Components/files that need changes
- Dependencies between tasks
- Edge cases to handle

If requirements are ambiguous, ask clarifying questions:

```
I need a few clarifications before creating the plan:
1. [Specific question about scope]
2. [Question about approach]
```

#### Requirements Reconciliation Gate

For work governed by existing requirements, reconcile them before exploring an
implementation:

1. Treat roadmap items and task summaries as scope indicators, not detailed
   behavioral contracts, unless the project explicitly declares otherwise.
2. Follow any source-priority hierarchy declared by the user or project context.
   If no hierarchy exists, do not silently choose between conflicting sources.
3. Record each material behavioral rule with its source path/section. When two
   sources overlap, state whether they agree, one refines the other, or they
   conflict. When an unresolved conflict would change the implementation:
   - manual mode → ask the user and STOP;
   - `HANDOFF_MODE=1` → do not prompt. Emit
     `ERROR [requirement-conflict]`, return
     `handoff_outcome: blocked_external`, and STOP without publishing or
     signaling `plan_ready`.
4. Do not edit source requirements while creating the plan. Report stale or
   contradictory artifacts and route them to their owner command or the user.

When behavior depends on two or more independent selectors, states, modes, or
input shapes, enumerate the supported combinations before writing tasks. Cover
only combinations allowed by the requirements, not a speculative Cartesian
product. For each material combination specify:

- accepted input and validation;
- persisted state;
- returned output or next transition;
- side effects and invariants;
- one verification scenario. If testing is enabled, make it a test; otherwise
  provide a runnable/manual verification step without adding a test task.

When repository data, configuration, fixtures, schemas, or content define the
real contract, include at least one representative existing artifact in an
end-to-end verification path instead of validating only synthetic fixtures.

Add an exact `## Requirements Reconciliation` section to the plan entrypoint
when multiple authoritative sources materially constrain the work, a conflict
was resolved, independent behavior dimensions exist, or a representative real
artifact is required. Keep it compact: declared authority, cited decisions,
applicable combination table, and verification evidence. Omit it for simple
work where none of these signals applies.

### Step 3: Explore Codebase

Before planning, understand the existing code through **parallel exploration**.

**Use `Task` tool with `subagent_type: Explore` to investigate the codebase in parallel.** This keeps the main context clean and speeds up research.

Launch 2-3 Explore agents simultaneously, each focused on a different aspect:

```
Agent 1 — Architecture & affected modules:
Task(subagent_type: Explore, model: sonnet, prompt:
  "Find files and modules related to [feature domain]. Map the directory structure,
   key entry points, and how modules interact. Thoroughness: medium.")

Agent 2 — Existing patterns & conventions:
Task(subagent_type: Explore, model: sonnet, prompt:
  "Find examples of similar functionality already implemented in the project.
   Show patterns for [relevant patterns: API endpoints, services, models, etc.].
   Thoroughness: medium.")

Agent 3 — Dependencies & integration points (if needed):
Task(subagent_type: Explore, model: sonnet, prompt:
  "Find all files that import/use [module/service]. Identify integration points
   and potential side effects of changes. Thoroughness: medium.")
```

**If full/ultra mode passed codebase reconnaissance** from Step 1 — use it as a starting point. Focus Explore agents on areas that need deeper understanding.

For ultra, continue until the plan has code-level evidence for every phase:

- relevant existing paths and symbols
- callers/consumers and side effects
- exact integration and configuration points
- existing tests, fixtures, commands, logging, migration, and documentation patterns

Do not paste entire source files into phase plans; cite only the evidence needed
to make implementation steps deterministic.

**After agents return, synthesize:**

- Which files need to be created/modified
- What patterns to follow (from existing code)
- Dependencies between components
- Potential risks or edge cases

**Fallback:** If Task tool is unavailable, use Glob/Grep/Read directly.

### Step 4: Create Task Plan

Create tasks using `TaskCreate` with clear, actionable items.

**Task Guidelines:**

- Each task should be completable in one focused session
- Tasks should be ordered by dependency (do X before Y)
- Include file paths where changes will be made
- Be specific about what to implement, not vague
- In ultra, keep TaskCreate descriptions concise but include the matching phase
  file link; the bundle remains the durable detailed source after context resets

Use `TaskUpdate` to set `blockedBy` relationships:

- Task 2 blocked by Task 1 if it depends on Task 1's output
- Keep dependency chains logical

### Step 5: Save Plan Artifact

**Determine plan artifact path:** the values were already resolved in Step 1.2.

- **Fast mode** → the resolved `paths.plan`.
- **Full mode** → `<configured plans dir>/<plan_identifier>.md`.
- **Ultra mode** → `<configured plans dir>/<plan_identifier>/index.md` plus
  `phase-NN-<slug>.md` files in the same directory.
- For `slug`, reserved `timestamp` / `uuid`, or Handoff-prepared branches,
  `plan_identifier = plan_file_stem`.
- For active `sequential`, `plan_identifier = <NNNN>_<plan_file_stem>`.

The `plan_file_stem` is **always** the canonical stem from Step 1.2.a (Handoff branch / git branch / description slug — in that order). Branch-based consumers reproduce the same stem at lookup time, so the producer must not deviate.

Before writing any unprefixed full/ultra target (default/reserved slug behavior
or a Handoff-prepared branch), check for the sibling representation
with the same stem: `<plan_identifier>.md` versus `<plan_identifier>/index.md`.
If either already exists, do not silently create a second active representation
or overwrite it. Ask the user to refine the existing plan, choose another
identifier, or explicitly replace it.

**Before saving, ensure directory exists:**

```bash
mkdir -p <configured plans dir>
```

For ultra also create the resolved bundle directory before writing its files.

**Full/fast plan file or ultra `index.md` must include:**

- For ultra only, the exact machine-readable marker
  `<!-- aif:plan-mode:ultra -->` exactly once near the top of `index.md`
  (immediately after the optional first-line Handoff annotation). Never
  translate, rewrite, or omit it. Consumers identify bundles by this marker,
  not by localized human-readable labels such as `Mode`.
- Title with feature name
- Branch and creation date
- `Original Request` section (required when the user explicitly supplied a planning request; omitted when the plan is created solely from `RESEARCH.md`)
- `Settings` section (Testing, Logging, Docs)
- `Roadmap Linkage` section (optional, only if the resolved roadmap artifact exists)
- `Research Context` section (optional, only if research content influenced this plan)
- `Requirements Reconciliation` section (conditional, when required by the gate in Step 2)
- `Tasks` section grouped by phases; in ultra this is the only task-checkbox source
- `Commit Plan` section when there are 5+ tasks

If `original_user_request` is non-empty:

- Write `## Original Request` before `## Settings`
- Preserve the exact user-provided request text after only recognized command tokens are removed and only outer whitespace is trimmed
- Do not translate the saved request; it is raw source input, not generated artifact prose

If the resolved roadmap artifact exists:

- If the user linked a milestone, write `## Roadmap Linkage` with `Milestone: "..."` and `Rationale: ...`
- If the user skipped linkage, write `## Roadmap Linkage` with `Milestone: "none"` and `Rationale: "Skipped by user"`

If research content influenced this plan:

- Include `## Research Context` by copying only the `Active Summary` (do not paste full `Sessions`)
- Include ``Source: `<selected_research_path>` (Active Summary, Updated: <research Updated timestamp>, SHA256: <sha256 of copied Active Summary>)`` so `$aif-implement`, `$aif-verify`, `$aif-improve`, and related consumers know the exact committed research revision
- Compute the hash from the normalized copied Active Summary exactly as described in Step 0: remove HTML comment blocks, preserve line order and leading whitespace, trim trailing spaces from every line, use LF line endings, and end with exactly one final newline. Feed the normalized text to `shasum -a 256` or `sha256sum` through stdin / inline shell input, never through a temp file, and copy the first output field.
- Treat the copied `Research Context` as the plan-owned authoritative requirements copy. A later change to the selected `RESEARCH.md` must not override these requirements without an explicit drift warning and user-requested rebase/refinement.
- Keep it compact; it should be readable as a one-screen requirements snapshot

If research exists but did not influence this plan, do not include `## Research Context`. An existing `RESEARCH.md` for topic A plus an explicit `$aif-plan` request for unrelated topic B must produce an unlinked plan for topic B.

Use the canonical template in `references/TASK-FORMAT.md` for fast/full.
Use `references/ULTRA-FORMAT.md` for ultra and verify every Phase Index link,
task mapping, dependency, and phase file before completion.

The canonical template defines the required sections and ordering only. Render all human-readable plan content in `artifact_language` before writing the file, applying `technical_terms_policy` and preserving stable tokens as described in Step 0.

**Commit Plan Rules:**

- **5+ tasks** → add commit checkpoints every 3-5 tasks
- **Less than 5 tasks** → single commit at the end, no commit plan needed
- Group logically related tasks into one commit
- Suggest meaningful commit messages following conventional commits

### Step 6: Next Steps

**Full/ultra mode + parallel (`--parallel`):** Automatically invoke `$aif-implement` — the whole point of parallel is autonomous end-to-end execution in an isolated worktree. If `HANDOFF_BRANCH_PREPARED = 1`, treat `--parallel` as disabled and do not auto-invoke `$aif-implement`.

```
$aif-implement

CONTEXT FROM $aif-plan:
- Plan artifact: <resolved plan file or ultra directory>      # see Step 1.2 / Step 5
- Testing: yes/no
- Logging: verbose/standard/minimal
- Docs: yes/no  # yes => mandatory docs checkpoint, no => warn-only
```

**Full/ultra mode normal:** STOP after planning. The user reviews the plan and decides when to implement.

The next-step templates below define structure only. Render all human-readable text in these user-facing responses in `ui_language`. Preserve command names, configured paths, task counts, and TaskList references unchanged.

```
Plan created with [N] tasks.
Plan artifact: <resolved plan file or ultra directory>

To start implementation, run:
$aif-implement

To view tasks:
/tasks (or use TaskList)
```

**Fast mode:** STOP after planning.

```
Plan created with [N] tasks.
Plan file: <resolved fast plan path>

To start implementation, run:
$aif-implement

To view tasks:
/tasks (or use TaskList)
```

### Context Cleanup

Suggest the user to free up context space if needed: `/clear` (full reset) or `/compact` (compress history).

---

## --list Subcommand

When `--list` is passed, show all active worktrees and their feature status. Then **STOP**.

```bash
git worktree list
```

For each worktree path:

1. Check whether the resolved plans directory exists under that worktree (`<worktree>/<resolved paths.plans>`, default: `<worktree>/.ai-factory/plans/`) and contains any root `*.md` plans or direct child `*/index.md` entrypoints containing `<!-- aif:plan-mode:ultra -->`
2. Show name and whether it looks complete (has tasks) or is still in progress

**Output format:**

```
Active worktrees:

  /path/to/my-project          (<configured-base-branch>)        <- you are here
  /path/to/my-project-feature-user-auth  (feature/user-auth)  -> Plan: feature-user-auth.md
  /path/to/my-project-feature-billing    (feature/billing)    -> Ultra: feature-billing/index.md
  /path/to/my-project-fix-cart-bug       (fix/cart-bug)        -> No plan yet
```

When `workflow.plan_id_format = sequential`, the displayed file or directory
includes the numeric prefix, e.g. `Plan: 0042_feature-user-auth.md` or
`Ultra: 0042_feature-user-auth/index.md`. Pick the highest-numbered match across
both representations for the worktree's branch stem.

## --cleanup Subcommand

When `--cleanup <branch>` is passed, remove the worktree and optionally delete the branch. Then **STOP**.

```bash
DIRNAME=$(basename "$(pwd)")
BRANCH_DIR=$(echo "<branch>" | tr '/' '-')
WORKTREE="../${DIRNAME}-${BRANCH_DIR}"

git worktree remove "${WORKTREE}"
git branch -d <branch>  # -d (not -D) will fail if unmerged, which is safe
```

If `git branch -d` fails because the branch is unmerged:

```
Branch <branch> has unmerged changes.
To force-delete: git branch -D <branch>
To merge first: git checkout <configured-base-branch> && git merge <branch>
```

If the worktree path doesn't exist, check `git worktree list` and suggest the correct path.

---

## Task Description Requirements

Every `TaskCreate` item MUST include:

- Clear deliverable and expected behavior
- File paths to change/create
- Logging requirements (what to log, where, and levels)
- Dependency notes when applicable
- Requirement source anchors and applicable behavior combinations when the
  Requirements Reconciliation Gate applies

**Never create tasks without logging instructions.**

Use canonical examples in `references/TASK-FORMAT.md`:

- TaskCreate Example
- Logging Requirements Checklist

## Important Rules

1. **NO tests if user said no** — Don't sneak in test tasks
2. **NO reports** — Don't create summary/report tasks at the end
3. **Actionable tasks** — Each task should have clear deliverable
4. **Right granularity** — Not too big (overwhelming), not too small (noise)
5. **Dependencies matter** — Order tasks so they can be done sequentially
6. **Include file paths** — Help implementer know where to work
7. **Commit checkpoints for large plans** — 5+ tasks need commit plan with checkpoints every 3-5 tasks
8. **Plan artifact location** – Fast: `paths.plan`. Full:
   `paths.plans/<plan_identifier>.md`. Ultra:
   `paths.plans/<plan_identifier>/index.md` plus phase files.
   `plan_identifier` uses the canonical handoff/branch/slug stem and optional
   sequential prefix from Step 1.2; `timestamp` and `uuid` fall back to `slug`.
9. **Ownership boundary** – This command owns plan artifacts only (the resolved fast plan path and full/ultra artifacts under `paths.plans`). Use owner commands (`$aif-roadmap`, `$aif-rules`, `$aif-explore`) for their artifacts.
10. **Roadmap linkage (when available)** — If the resolved roadmap artifact exists, include a `## Roadmap Linkage` section in the plan (or explicitly state it was skipped).
11. **Reconcile before planning** — Do not let a roadmap summary, lower-priority
    source, or internally consistent implementation override an unresolved
    behavioral requirement.

## Plan File Handling

**Fast mode (`paths.plan`, default: `.ai-factory/PLAN.md`)**

- Temporary plan for quick work
- `$aif-implement` may offer deletion after completion

**Full mode (`paths.plans/<plan_identifier>.md` — default)**

- Long-lived plan for feature delivery
- The canonical `plan_file_stem` comes from Step 1.2.a: Handoff branch name (slashes replaced) → git branch name (slashes replaced) → description slug, in that order
- When `workflow.plan_id_format = sequential`, the filename becomes
  `paths.plans/<NNNN>_<plan_file_stem>.md` — the prefix is the next 4-digit
  number after the highest existing numbered plan in the directory, capped at
  `9999`. Numbers are derived from currently existing files: deleting or moving
  a numbered plan out of the directory can free that number for reuse on the
  next run. The Handoff branch contract force-disables the prefix (see Step
  1.2.b–c).
- `timestamp` and `uuid` are reserved values; both currently behave like
  `slug` (no prefix is applied)

**Ultra mode (`paths.plans/<plan_identifier>/index.md`)**

- Long-lived plan bundle for high-fidelity delegation
- Uses the same canonical stem, branch/worktree behavior, and optional
  sequential prefix as full mode; the prefix is on the directory
- `index.md` is the manifest and progress source; direct child phase files are
  implementation specifications
- Consumers must read the bundle using `references/ULTRA-FORMAT.md`; do not
  flatten it into a local single-file plan

For concrete end-to-end flows (fast/full/ultra/parallel/interactive), read `references/EXAMPLES.md` (Flow Scenarios).
