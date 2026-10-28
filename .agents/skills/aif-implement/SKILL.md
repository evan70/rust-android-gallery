---
name: aif-implement
description: Execute implementation tasks from the current plan. Works through tasks sequentially, marks completion, and preserves progress for continuation across sessions. Use when user says "implement", "start coding", "execute plan", or "continue implementation".
argument-hint: '[--list] [--without-plan <description>] [@plan-file-or-directory] [task-id or "status"]'
allowed-tools: Read Write Edit Glob Grep Bash TaskList TaskGet TaskUpdate AskUserQuestion Questions mcp__handoff__handoff_sync_status mcp__handoff__handoff_push_plan mcp__handoff__handoff_get_task mcp__handoff__handoff_list_tasks mcp__handoff__handoff_update_task
disable-model-invocation: false
---

<!-- aif-ext:aifhub-extension:aif-implement:prepend:start -->
## AIFHub Implement OpenSpec-native Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-implement` body. When any rule below conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Once artifact classification permits local execution, follow [test quality and readiness waits](../../skills/shared/TEST-QUALITY.md) before selecting or changing an automated check or a readiness wait. This applies to OpenSpec-native and classic legacy execution; preserve marker-first delegation and no-test fallbacks.

Resolve the policy from the AIFHub extension root; in an installed project it is `.ai-factory/extensions/aifhub-extension/skills/shared/TEST-QUALITY.md`. Do not create a replacement policy in the consumer project.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Use the built-in `/aif-implement` skill as the canonical execution command for both OpenSpec-native changes and the extension's legacy companion plan workflow.

### Skill-Context Resolution

Read skill-context in this order:

1. `.ai-factory/skill-context/aif-implement/SKILL.md`
2. `.ai-factory/skill-context/aif-implement-plus/SKILL.md` as legacy compatibility fallback

If both exist, `aif-implement` wins.

### Mode Detection

Before resolving an implementation target, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with Legacy AI Factory-only mode and state that no OpenSpec-native protocol was detected.

### Legacy ultra marker-first boundary

In Legacy AI Factory-only mode, classify the normalized project-relative plan entrypoint with `classifyLegacyPlanShape()` from `scripts/legacy-plan-migration.mjs` before folder-only detection, companion discovery, companion plan-file creation, execution metadata hydration, or any `status.yaml` write. Marker validation is first; known companion filenames are considered only after the classifier returns a classic shape.

- For `ultra-valid`, stop all AIFHub companion logic and return the exact upstream handoff `/aif-implement <entrypoint>`. Do not create or synchronize a sibling `<plan-id>.md`, `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, `explore.md`, or AIFHub task state; upstream owns the complete marked bundle atomically.
- For `ultra-invalid` or `collision`, fail closed before any write and report only bounded `shape`, safe `entrypoint`, and classifier `code` values.
- For `classic-pair` or `classic-folder-only`, continue with the classic companion rules below. An unrelated directory is not a plan.
- Diagnostics may include only `shape`, safe project-relative `entrypoint`, and `handoff`; never include marker bodies, phase contents, request/research bodies, credentials, raw stdout, or raw stderr.

### Task coordination after classification

After classification permits local execution and the canonical tasks have been read, follow [task conflicts and bounded batches](../../skills/shared/TASK-COORDINATION.md) before implementation edits or first worker dispatch. Apply it to OpenSpec-native and classic legacy execution; marker-first ultra delegation and invalid/collision stops remain prior boundaries.

Resolve the policy from the AIFHub extension root; in an installed project it is `.ai-factory/extensions/aifhub-extension/skills/shared/TASK-COORDINATION.md`. The coordinator owns the preflight scan, explicit batch manifest, and per-item reconciliation. Workers consume only the selected scope. Do not create a replacement policy in the consumer project.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-implement` executes implementation tasks for the active OpenSpec change.

#### SessionBrief binding

For changes with `## SDD Profile Inputs`, `.ai-factory/sdd-policy.json`, or existing SDD runtime artifacts, validate `ai-factory aifhub-session-brief status --change <change-id> --json` before editing. `buildImplementationContext()` also checks this binding before creating runtime directories and exposes `sessionBrief` with its digest and current brief. Prefer that brief as compact execution context, then resolve its exact canonical/task/spec/rules references at full fidelity for the selected task. The brief selects context; source, policy, and permissions remain authoritative.

Missing, stale, research, or invalid context stops implementation and returns the planning-owner compile handoff (or `/aif-explore` for research). Do not silently reuse the old brief or treat it as QA. Supply the exact consumed digest as `trace.sessionBriefDigest` to `writeExecutionTrace()`; its writer revalidates the binding before writes. Write the trace before marking canonical task checkboxes, then recompile after progress/sync changes before the next task or session. Material scope discoveries require the normal canonical planning-owner workflow before continuation.

When helpers are unavailable for an opted-in change, return the owner handoff; do not pretend the brief was validated. Existing unopted changes with no SDD runtime artifacts retain canonical filesystem fallback. A quick profile cannot disable project-required tests/security/review/human approval, migration/rollback, `/aif-verify`, or `/aif-done`. Briefs and profile decisions stay under `.ai-factory/state/<change-id>/`; protected validation artifacts are read in full, never semantically rewritten into the brief.

After implementation on an SDD-managed change, the planning owner or review side may run `ai-factory aifhub-plan-compliance check --change <change-id> --json`; the worker never writes `plan-compliance.json` itself and never treats the receipt as QA evidence. A `replan_required` or `blocked` outcome returns the remaining tasks to the planning owner before further edits. Tracer runtime artifacts under `.ai-factory/state/<change-id>/tracer/` never satisfy completion gates; only an owner `promote` decision followed by canonical updates and a fresh production SessionBrief moves the work forward.

For plan content, read only the canonical OpenSpec artifacts listed below. Do not inspect or mutate `.ai-factory/plans/**` after an OpenSpec change resolves.

Use `buildImplementationContext(options)` from `scripts/openspec-execution-context.mjs` when available before editing implementation files. Treat the returned resolver diagnostics, canonical artifacts, generated rules, OpenSpec apply instructions, runtime paths, warnings, and errors as the machine-readable implementation context. If the helper is unavailable, fall back to the explicit filesystem reads and runtime boundaries in this section.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Resolve the active change using `scripts/active-change-resolver.mjs` when available:

- Prefer an explicit `<change-id>` or `@openspec/changes/<change-id>` input when provided.
- Otherwise use `resolveActiveChange` behavior: current working directory, current branch mapping, current pointer, then single active change.
- Treat selected source, candidate list, warnings, and errors as user-visible implementation context.
- If an explicit or inferred `<change-id>` cannot be resolved as an OpenSpec change, check for matching legacy AI Factory plan artifacts through `detectMigrationNeed(options)` from `scripts/legacy-plan-migration.mjs` or equivalent read-only detection. If migration is suggested, do not auto-migrate. Show exactly:

```text
Found legacy AI Factory plan artifacts for `<change-id>` but no OpenSpec change at `openspec/changes/<change-id>`.
Run the legacy migration script with:

ai-factory aifhub-migrate-legacy-plans <change-id> --dry-run
ai-factory aifhub-migrate-legacy-plans <change-id>
```

Read canonical OpenSpec artifacts before editing implementation files:

- `openspec/specs/**`
- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`

Treat planning source sections as read-only implementation context:

- Use `## Original Request` as the raw intent anchor that explains why the change exists. Do not translate, normalize, rewrite, or use it instead of the executable `tasks.md`, design decisions, or requirements.
- When `proposal.md` contains `## Research Context`, use its embedded Active Summary and source revision as the authoritative committed scope.
- When `proposal.md` contains `## Requirements Reconciliation`, treat it as the committed requirement-consistency record: read the declared authority and cited decisions before dispatching work, and never mutate the section during implementation.
- For an embedded ultra source, pass its exact project-relative `RESEARCH.md` path to `resolveUltraResearchSource()` from `scripts/ultra-research-resolver.mjs` and consume only its structured `source`, `revision`, and `diagnostic`. This centrally revalidates the sibling marker/index/status/link and normalized Active Summary digest; do not implement local selection or hashing heuristics.
- Compare the exact regular or ultra source only for revision drift and optional rationale. Missing/invalid source, changed `Updated`, or changed normalized `SHA256` emits `WARN [research-drift] change-id=<change-id> source=<path> expected=<embedded revision> current=<live revision>` and continues from the embedded snapshot. Recency never selects a replacement source.
- Do not silently apply requirements from newer research, expand task scope, or mutate `## Original Request`, `## Research Context`, or `## Requirements Reconciliation` during implementation.
- Before marking a task `in_progress` or delegating it, reconcile the task's expected behavior against the authoritative requirements (canonical specs, `## Requirements Reconciliation`, committed `## Research Context`) and the `## Original Request` raw intent anchor. A clear unresolved conflict emits `ERROR [requirement-conflict]` and stops the task without edits, without marking it `in_progress`, and without merge, commit, review, or done signals; ambiguity that leaves the contract unclear emits `WARN [requirement-ambiguity]` and stops for clarification instead of guessing. Do not resolve a conflict by rewriting source requirements. When behavior depends on independent selectors, states, modes, or input shapes, confirm the task covers each applicable supported combination across validation, persistence, output/transition, and side effects; do not infer a restriction on one dimension from a rule about another.
- Keep credentials, raw provider output, and the full request/research bodies out of drift diagnostics and execution traces.

Hydrate runtime todo state from canonical OpenSpec tasks before editing implementation files:

- Treat `openspec/changes/<change-id>/tasks.md` as the source checklist.
- If the current runtime exposes a todo or plan tool, use it before editing. In Codex, use `update_plan` when available.
- Map checked tasks to `completed`, mark only the selected unfinished task, tightly coupled task group, or explicit small same-shape batch as `in_progress`, and leave other unfinished tasks as `pending`.
- If no todo or plan tool is available, report a concise task snapshot in the normal response and continue from canonical `tasks.md`.
- Report missing todo-tool support as a capability fallback, not as an implementation failure.
- Hydrating runtime todo state does not authorize broad task expansion; `/aif-implement` executes one task, one tightly coupled task group, or one explicit small same-shape batch validated under `skills/shared/TASK-COORDINATION.md`. Complete each item only after reconciling its expected change with the actual diff and evidence.

#### Persistent execution and worker result acceptance

For both configured OpenSpec and complete classic plans after marker-first classification, use the installed `ai-factory aifhub-execution <action> --json` helper with a JSON object on stdin. Read the installed extension's `docs/workflow-mechanics.md` for payload contracts and follow the durable execution section of [task coordination](../../skills/shared/TASK-COORDINATION.md). This supporting execution state is separate from SessionBrief context compilation and authoritative Handoff/QA gates. The helper selects effective paths/tools, requires a complete classic pair, and delegates valid ultra before writing state.

Apply the service-mechanics boundary in TASK-COORDINATION.md: extract and propagate helper JSON fields through the existing helper/direct tool path, without a separate AI worker. Preserve the recorded actor, current version, exact digests, freshness checks and parent decision for every transition.

Include the selected execution form and its short reason in the existing dispatch manifest or assignment/preflight notes before admission. Keep helper payloads unchanged. Select eligible low-risk same-shape items through the bounded batch policy; keep shared-interface, overlapping-file, unresolved-dependency or different-risk work coupled/serial or report its exact blocker.

Before selecting the next group, reconcile available observed setup/orientation/integration costs with the previous execution-form reason. Keep estimates and unavailable measurements explicit, and preserve all required checks and independent roles.

For authorized parallel implementation in host-provided worktrees, follow [isolated execution](../../skills/shared/ISOLATED-EXECUTION.md) and the installed extension's `docs/isolated-execution.md`. Register fresh single-task worker runs as one independent group before edits. Workers use ordinary execution actions in their own roots; the parent uses `isolation-*` in the integration root, observes actual worker stops, and verifies each transfer against the combined tree. In this mode, local worker `accept` grants no checkbox authority: defer progress synchronization until `isolation-close` and fresh read-only `isolation-resume`, then apply only `closure.accepted`. Recover the existing group/worker records on re-entry. Worktree preparation, cancellation and cleanup belong to the host; absent capability keeps execution serial.

- Before editing or delegating, the parent calls `start` with one canonical `task_id`, a unique `run_id`, stable `owner` and `worker` labels, `role: implement`, and explicit project-relative file/directory `scope`. Include extra policy, ignored inputs, or an existing SessionBrief in `context_paths` when they influenced the assignment. Serialize coupled/shared-file tasks through fresh assignments. For an independent 2-5 item manifest use `batch-start` with exact files, expected changes, checks/fallbacks, dependencies and preflight references; one `run_id` owns the whole batch.
- On re-entry, call read-only `resume` before any edits or checkpoint writes. Reuse the saved task, scope, progress, and next step only when fresh. A stale checkpoint, context, revision, or evidence stops that run; the parent must reread current inputs and explicitly create a new assignment. Never overwrite a checkpoint merely to clear a stale result.
- After an in-scope work step, call `checkpoint` with the current `version`, worker `actor`, and `progress: {completed_steps, next_step, blocker?}`. Preserve the returned version. Keep notes concise and free of secrets or raw provider output. Write existing implementation traces through `writeExecutionTrace()` as before.
- A delegation admission handle means only that work was admitted. The worker submits `result` with an explicit terminal status, exact `changed_files`, observed check exit codes, and existing evidence paths. `completed` is not parent acceptance, a QA verdict, or permission to finalize.
- The parent reviews the actual diff and evidence, then calls `accept` with its actor label, the current version, and exact `result_digest`. Only a fresh completed result can become `accepted`. Worker results do not update canonical tasks; parent-owned checkbox updates happen after acceptance. Actor labels correlate assignments and do not authenticate another OS process.
- Local execution uses the same sequence with one actor label for owner and worker. Do not stage or commit while a run is active; HEAD, branch, index, context, and file contents are pinned. Concurrent writes in a shared worktree can invalidate another run; coordinate a quiet checkpoint/acceptance boundary.
- Failed, blocked, cancelled, or timed-out work stays terminal and unaccepted. Never infer completion from silence, a timeout, a duplicate reply, or an old successful check. Exact delivery retries are idempotent; conflicting replies fail closed.
- For a batch, the worker saves a final `checkpoint`, stops edits, reruns affected checks, and uses `batch-seal` to freeze per-item evidence at one final snapshot. It submits `batch-result` per task with the returned seal digest and current shared version. The parent reviews each item, uses `batch-accept` sequentially, and `batch-close` before updating only accepted checkboxes. Call `resume` immediately before synchronization; changed source/worktree/evidence forbids it. Omitted items remain unfinished; historical close replay is not current completion authority.
- A stale or interrupted dispatch is retired through exact read-only `inspect` and owner `interrupt`, preserving results and pending attempt history. Use the host's existing cancellation capability for the actual owned worker when available; record `stopped` only after observing exit with sanitized evidence. Otherwise record `running`/`unknown`, keep scope reserved, and use owner `stop-confirm` after later observation. These actions never cancel a process themselves. Do not clear orphan locks or bypass a pending journal; explicit v1 `upgrade` requires quiescent predecessor history and its inspected inventory digest.
- If this helper is absent in an older installation, report the capability fallback and use the existing local trace workflow without claiming durable resume or structured acceptance. A present helper's stale/conflict/error response is not permission to bypass it.

After parent acceptance, continue the existing read-only gates and `/aif-verify <change-id>` Handoff. These records must never replace Handoff gate summaries, QA evidence, or done receipts.

#### Roadmap lifecycle deferral

In OpenSpec-native mode, this section overrides the upstream roadmap completion step.

- `/aif-implement` must not edit the configured roadmap and must not mark a milestone, phase, slice, or managed lifecycle row complete, even when all implementation tasks are checked.
- It may read `## Roadmap Linkage` from the canonical proposal and report only the detected linkage fields plus the fact that lifecycle ownership is deferred.
- Implementation output must not claim roadmap completion or copy unrelated roadmap content.
- Route authoritative validation to `/aif-verify <change-id>`. Only after verification passes may `/aif-done <change-id>` own the successful finalization transition defined by the canonical change.

Read generated rules as derived implementation guidance when present:

- `.ai-factory/rules/generated/openspec-merged-<change-id>.md`
- `.ai-factory/rules/generated/openspec-change-<change-id>.md`
- `.ai-factory/rules/generated/openspec-base.md`

#### Evidence-driven development cycle

For each testable behavior change whose plan, project policy, or existing test conventions call for an automated check, use this bounded cycle before declaring the task complete:

1. **RED** - select or add the narrowest focused automated check, record it as `testCheck`, and run it before the production edit. Record `redResult` only after observing the expected behavioral failure; a syntax, fixture, dependency, or environment failure is not valid RED evidence.
2. **GREEN** - make the smallest in-scope production change that satisfies the focused check, rerun the same check, and record `greenResult`. Do not broaden implementation merely to make unrelated checks green.
3. **REFACTOR** - perform only safe cleanup justified by the task, rerun the same focused check, and record `refactorResult`. Skip cleanup when it would expand scope.

Persist `testCheck`, `redResult`, `greenResult`, `refactorResult`, and `fallbackDecision` through `writeExecutionTrace()` under `.ai-factory/state/<change-id>/implementation/`. For documentation-only work, generated artifacts, user-authorized no-test work, or a task with no useful automated check, do not fabricate RED evidence: record a bounded `fallbackDecision` and run the narrowest applicable non-test verification instead.

This cycle is supporting runtime evidence only. It does not write QA evidence, weaken project-specific test policy, or replace the authoritative `/aif-verify <change-id>` gate.

Execution trace and runtime state boundaries:

- Prefer `writeExecutionTrace(changeId, trace, options)` from `scripts/openspec-execution-context.mjs` for implementation traces.
- Write implementation progress, task execution traces, degraded capability notes, and runner metadata only under `.ai-factory/state/<change-id>/`.
- Do not write runtime-only files, summaries, validation output, or execution traces under `openspec/changes/<change-id>/`.
- Do not create legacy plan-folder execution artifacts in OpenSpec-native mode.
- QA evidence belongs under `.ai-factory/qa/<change-id>/` and is owned by `/aif-verify`; implementation may name the path in normal output but should not write verification results there.

Normal implementation responses should report:

- selected `change-id` and resolver source;
- canonical artifacts read;
- generated rules freshness or missing/stale `WARN`;
- runtime state path under `.ai-factory/state/<change-id>/`;
- task progress from the OpenSpec `tasks.md`;
- runtime todo hydration summary or task snapshot capability fallback;
- next step `/aif-verify <change-id>` when implementation is ready.

After implementation, optional read-only gates are:

- `/aif-rules-check`
- `/aif-review`
- `/aif-security-checklist`

The authoritative final verification remains `/aif-verify <change-id>`.

Do not install OpenSpec skills or slash commands.
Do not route users to deprecated workflow aliases or legacy `*-plus` command names.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve the existing legacy companion plan workflow.

After the owning classic workflow establishes a complete pair, apply the persistent execution contract above and the shared durable task coordination rules. Ordinary status/history/current-task updates are projected out of execution identity; canonical task, policy and binding changes still invalidate it. Helper admission never repairs a missing companion or substitutes a retained OpenSpec tree.

Resolve all of these inputs to one active plan pair before execution starts:

- `.ai-factory/plans/<plan-id>.md`
- `.ai-factory/plans/<plan-id>/`
- `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, or `explore.md` inside a plan folder

If only the folder exists, create the missing companion plan file first and record the migration event in `status.yaml.history`.

Legacy AI Factory-only workflow rules:

- `/aif-implement` is the canonical execution command for this extension workflow.
- When no plan exists yet, route the user through `/aif-plan full "<task>" -> /aif-improve`.
- `/aif-implement` owns git strategy resolution and must persist `execution.git.*` in `status.yaml`.
- `/aif-implement` also owns `execution.mode`, `execution.runtime`, and `execution.subagent` updates.
- After tasks complete, route to `/aif-verify`; a passing verification leaves the plan ready for optional `/aif-done` finalization.
- Do not route users to deprecated workflow aliases or legacy `*-plus` command names.

### Subagent Compatibility

When checking optional Claude worker availability in Legacy AI Factory-only mode, support both current and legacy filenames:

- prefer `.claude/agents/implement-coordinator.md`
- support `.claude/agents/implement-worker.md`
- support legacy `.claude/agents/implementer.md`
- support legacy `.claude/agents/implementer-isolation.md`

When persisting `execution.subagent`, allow:

- `implement-coordinator`
- `implementer`
- `implementer-isolation`
- `null`

Prefer `implement-coordinator` when available.

### Execution Metadata

- Preserve sibling keys when updating `execution.*`.
- In Legacy AI Factory-only mode, record git-strategy decisions, runtime changes, legacy upgrades, and mode switches in `status.yaml.history`.
- In OpenSpec-native mode, write runtime state only under `.ai-factory/state/<change-id>/`.
- When the implementation flow needs a manual checkpoint, the next command is `/aif-verify`, not a deprecated finalize alias.

### Compatibility Note

If historical docs or plan notes still mention `aif-implement-plus`, interpret that as `/aif-implement`.
<!-- aif-ext:aifhub-extension:aif-implement:prepend:end -->

# Implement - Execute Task Plan

Execute tasks from the plan, track progress, and enable session continuation.

## Workflow

### Step 0 (pre): Detect Handoff Mode

Determine Handoff mode. If the caller passed `HANDOFF_MODE` and `HANDOFF_SKIP_REVIEW` as explicit text in the prompt, use those values. Otherwise, use the Bash tool:

```
Bash: printenv HANDOFF_MODE || true
Bash: printenv HANDOFF_SKIP_REVIEW || true
```

**Then check `HANDOFF_MODE`:**

#### When `HANDOFF_MODE` is `1` (autonomous Handoff agent)

The Handoff coordinator already manages status transitions and DB writes directly. Do NOT call MCP tools. Instead:

- **No interactive questions:** Do not use `AskUserQuestion` — use sensible defaults (auto-commit at checkpoints, skip pause prompts).
- **No pause/resume prompts:** Execute all tasks sequentially without stopping
  for confirmation. Blocking integrity, safety, or requirement conflicts still
  stop with their defined non-interactive outcome.

#### When `HANDOFF_MODE` is NOT `1` (manual Claude Code session)

Handoff sync is handled inline — see **Step 0.2** (after reading the plan file) for the task ID extraction and MCP sync trigger. The sync points are:

- **On start (Step 0.2):** `handoff_sync_status` → `"implementing"` (with `paused: true`)
- **On checklist update (Step 3.6):** `handoff_push_plan` with updated plan content
- **On completion (Step 5):** `handoff_push_plan` with final plan, then `handoff_sync_status` → `"review"` (with `paused: true`) or `"done"` (with `paused: false` when `HANDOFF_SKIP_REVIEW=1`)

**CRITICAL:** Always pass `paused: true` with every `handoff_sync_status` call except `done`. This prevents the autonomous Handoff agent from picking up the task while you work manually. Only `done` passes `paused: false`.

### Step 0: Check Current State

**FIRST:** Determine what state we're in:

```
1. Read `.ai-factory/config.yaml` if it exists to resolve:
   - `paths.description`, `paths.architecture`, `paths.rules_file`, `paths.roadmap`, `paths.research`; derive `research_bundles_dir = <parent directory of paths.research>/research/`
   - `paths.plan`, `paths.plans`, `paths.fix_plan`, `paths.patches`
   - `paths.archive`
   - `paths.rules`
   - `language.ui`, `language.artifacts`
   - `git.enabled`, `git.base_branch`, `git.create_branches`
   - `workflow.plan_id_format` (default: `slug`) — used by branch-based plan discovery.
     Active values: `slug` and `sequential`. Discovery treats a root `*.md` as
     a full plan unless it is the resolved `paths.plan` or `paths.fix_plan`.
     Treat a direct child `*/index.md` as an ultra bundle only after reading it
     and confirming that it contains exactly one
     `<!-- aif:plan-mode:ultra -->`; ignore unrelated directories and never count
     phase files independently.
     When `sequential`, resolve both
     `<paths.plans>/[0-9]{4}_<branch-slug>.md` and
     `<paths.plans>/[0-9]{4}_<branch-slug>/index.md`, then choose the
     highest-numbered matching artifact.
     `timestamp` and `uuid` are **reserved values** and currently behave like `slug`.
     Treat any unknown value as `slug`.
   - `rules.base` plus any named `rules.<area>` entries
2. Parse arguments:
   - --list → list available plans only (no implementation; STOP)
   - --without-plan <description> → inline implementation mode; skip plan discovery and jump to Step 0.inline
   - @<path> → explicit plan file, ultra directory, or ultra `index.md` override (highest priority)
   - <number> → start from specific task
   - status → status-only mode
   - Optional inline-mode flag: --docs=yes|no|warn (only valid with --without-plan; default: warn)
3. If `git.enabled = true`, check for uncommitted changes (`git status`)
4. If `git.enabled = true`, check current branch
```

### Step 0.list: List Available Plans (`--list`)

If `$ARGUMENTS` contains `--list`, run read-only plan discovery and stop.

```
1. Get current branch:
   git branch --show-current (git mode only)
2. Convert branch to canonical stem: replace "/" with "-" (git mode only)
3. Check existence of:
   - <configured plans dir>/<branch-stem>.md and
     <configured plans dir>/<branch-stem>/index.md (git mode only); Read the
     directory entrypoint and report it only when it contains exactly one
     `<!-- aif:plan-mode:ultra -->`
   - when `workflow.plan_id_format = sequential`: also glob
     `<configured plans dir>/[0-9][0-9][0-9][0-9]_<branch-stem>.md` and
     `<configured plans dir>/[0-9][0-9][0-9][0-9]_<branch-stem>/index.md`;
     Read every directory entrypoint, discard those without exactly one marker,
     and report all valid matches (highest-numbered first)
   - if git mode is off or branch creation is disabled: any root `*.md` full
     plan or declared-ultra direct child `*/index.md` entrypoint in
     `<configured plans dir>/`; exclude the resolved fast/fix plan paths
   - <resolved fast plan path>
   - <resolved fix plan path>
4. Print plan availability summary and usage hints
5. STOP.
```

**Important:** In `--list` mode:

- Do not execute tasks
- Do not modify files
- Do not update TaskList statuses

For detailed output format and examples, see:

- `skills/aif-implement/references/IMPLEMENTATION-GUIDE.md` → "List Available Plans (`--list`)"

### Step 0.inline: Inline Implementation Mode (`--without-plan`)

If `$ARGUMENTS` contains `--without-plan`, execute a single scoped task from the description WITHOUT creating or reading any plan file. This is the lightweight path for small `feat`/`chore` tasks that do not justify a full plan but are not bug fixes either (use `$aif-fix` for bugs).

**Argument parsing:**

```
1. description = everything after `--without-plan`, excluding any recognized flag tokens (`--docs=...`).
2. docs_policy = value of `--docs=yes|no|warn` if present, else `warn` (default).
3. Validation:
   - description is empty →
     ERROR: "Usage: $aif-implement --without-plan <description> [--docs=yes|no|warn]"
     → STOP
   - arguments also contain `@<path>`, `status`, or a bare task id number →
     ERROR: "`--without-plan` is mutually exclusive with @plan-file, status, and task id."
     → STOP
   - `--docs=<value>` where <value> not in {yes, no, warn} →
     ERROR: "Invalid --docs value. Expected yes|no|warn."
     → STOP
```

**Scope guard (prevent silent mega-tasks):**

Before executing, assess the description. If it looks too broad for a one-shot inline task — multiple unrelated imperatives joined by "and"/"и", references to multiple subsystems, or roughly more than ~300 characters of scope — do NOT attempt to guess a plan. Instead print:

```
Description looks too broad for inline implementation. Recommended:
  $aif-plan fast <description>
```

→ STOP.

Small, focused descriptions (e.g. "add GET /healthz returning 200 with {status:\"ok\"}") proceed.

**Surprise-warn on existing plan artifacts (non-blocking):**

Inline mode ignores plan files by design. If any of these exist on disk, emit a `WARN [inline]` line so the user notices the intentional skip (do NOT read them, do NOT redirect):

- `<configured plans dir>/<branch>.md` or
  `<configured plans dir>/<branch>/index.md` (git mode only) — or their
  `[0-9]{4}_<branch>` sequential forms
- resolved fast plan path (`paths.plan`)
- resolved fix plan path (`paths.fix_plan`)

Example: `WARN [inline] paths.plan exists but is ignored in --without-plan mode.`

**Load project context (same as regular implement):**

Use the resolved config from Step 0:

- `paths.description` (DESCRIPTION.md) if present
- `paths.architecture` (ARCHITECTURE.md) if present
- `paths.rules_file` (RULES.md) + `rules.base` + named `rules.<area>` entries
- `.ai-factory/skill-context/aif-implement/SKILL.md` — MANDATORY if the file exists (same precedence and enforcement as regular mode in Step 0.1)
- `language.ui`, `language.artifacts`

**Plan artifact policy:** inline mode does NOT load or use plan/fix-plan files. Plan files are never read, parsed, or executed. A minimal existence probe is permitted (see the surprise-warn section above) solely to emit the `WARN [inline]` line — nothing is read from disk. Also skip: resume/recovery reconciliation, TaskList loading, checkbox state comparison.

**Execute the task (one-shot):**

1. Announce: `Inline implementation: <description>`
2. Read only files relevant to the described scope
3. Apply changes following existing code patterns and skill-context rules
4. Apply verbose logging per `references/LOGGING-GUIDE.md`
5. Do not add tests by default. Add them only if the description explicitly requests tests (e.g. "with tests", "add tests for X") OR if existing project conventions / touched code paths clearly require them (e.g. a test file mirrors every source file in the area being changed, or a RULES.md / skill-context rule mandates test coverage for this kind of change). When in doubt, prefer NO tests and let the user follow up via `$aif-plan` if wider test coverage is needed.
6. Verify the change compiles/runs and the described behavior works

**Prohibited in inline mode:**

- Do NOT create or read `paths.plan` / `paths.plans/*` / `paths.fix_plan`.
- Do NOT invoke `$aif-plan` or `$aif-fix`.
- Do NOT create entries under `paths.patches` (no `[FIX]` self-improvement patch — this is not a bugfix flow).
- Do NOT call `TaskList` / `TaskGet` / `TaskUpdate` (no plan = no persisted tasks).
- Do NOT search for or modify plan checkboxes on disk.
- Do NOT trigger the roadmap milestone completion check, docs checkpoint-from-plan-setting, plan-file cleanup prompt, or worktree merge prompt (those belong to the plan-backed workflow).

**Handoff inline support:**

> Naming clarification: `--without-plan` means "without a **local** plan artifact on disk" (no `paths.plan` / `paths.plans/*` / `paths.fix_plan`). When a Handoff task is linked, the task is still represented as a synthetic plan **inside Handoff** via `handoff_push_plan` — that's a remote representation, not a local file. The local-no-plan contract is preserved; only the remote sync surface is unchanged.

**When `HANDOFF_MODE` is `1` (autonomous Handoff agent invoked inline mode):**

- Do NOT call any `mcp__handoff__*` tool (the coordinator manages status/sync directly — same rule as Step 0 (pre)).
- Do NOT create local plan artifacts (the regular Prohibited list above still applies).
- Do NOT switch branches, create worktrees, merge worktrees, or otherwise alter the branch/worktree the coordinator set up — inline mode operates on the working tree it was invoked in.
- Proceed with the one-shot execution; the coordinator marks the task complete after the skill returns.

**When `HANDOFF_MODE` is NOT `1` and `HANDOFF_TASK_ID` is set (manual Claude Code session linked to a Handoff task):**

1. Build synthetic plan content:

   ```markdown
   # Inline implementation
   - [ ] <description>
   ```

2. Call `handoff_sync_status` with `{ taskId: <HANDOFF_TASK_ID>, newStatus: "implementing", sourceTimestamp: "<current UTC ISO 8601>", direction: "aif_to_handoff", paused: true }`.
3. Call `handoff_push_plan` with `{ taskId: <HANDOFF_TASK_ID>, planContent: <synthetic content above> }`.
4. After successful execution, flip the checkbox to `- [x]` in the synthetic content and call `handoff_push_plan` again with the updated text.
5. Finalize sync:
   - If `HANDOFF_SKIP_REVIEW` is `1` → `handoff_sync_status` → `"done"` with `paused: false`.
   - Otherwise → `handoff_sync_status` → `"review"` with `paused: true`.

If `HANDOFF_TASK_ID` is missing → skip all MCP sync for this run.

**Docs policy (inline mode, driven by `--docs`):**

- `--docs=yes` → after completion, show the docs checkpoint (same AskUserQuestion as `Docs: yes` in regular mode) and route changes through `$aif-docs`.
- `--docs=no` → suppress the documentation checkpoint, emit `WARN [docs] --docs=no in inline mode; documentation checkpoint skipped`.
- `--docs=warn` (default) → emit `WARN [docs] Inline mode default is warn-only; documentation checkpoint skipped. Pass --docs=yes to enable.`

**Context maintenance in inline mode:**

- Resolved description artifact updates: allowed, same rules as regular mode (only factual deltas for new deps/integrations).
- Resolved architecture artifact + `AGENTS.md`: allowed only if new modules/folders were actually created.
- Resolved roadmap artifact: NOT updated in inline mode (no milestone linkage available without a plan).
- Resolved rules file: NOT edited in inline mode (same as regular).

**Completion output (inline mode):**

```
## Inline Implementation Complete

Task: <description>

Files modified:
- <file> (created|modified)
Documentation: <outcome per --docs>

What's next?

1. 🔍 $aif-verify — Verify the change (recommended)
2. 💾 $aif-commit — Commit directly
```

Then offer:

```
AskUserQuestion: Inline task complete. What's next?

Options:
1. Verify first — Run $aif-verify (recommended)
2. Skip to commit — Go straight to $aif-commit
```

→ **STOP** after the chosen follow-up completes. No summary document, no report file.

### Step 0.0: Resume / Recovery (after a break or after /clear)

If the user is resuming **the next day**, says the session was **abandoned**, or you suspect context was lost (e.g. after `/clear`), rebuild local context from the repo **before** continuing tasks:

If `git.enabled = true`:

```
1. git status
2. git branch --show-current
3. git log --oneline --decorate -20
4. (optional) git diff --stat
5. (optional) git stash list
```

If `git.enabled = false`, skip git recovery commands and reconcile only from the resolved plan/fix-plan paths plus the working tree state.

Then reconcile plan/task state:

- Ensure the current plan artifact matches the current branch when git branch plans are in use (`@path` override wins; otherwise a branch-named full file or ultra directory takes priority over the resolved fast plan).
- If `git.enabled = false` or full/ultra plans were created without a branch, prefer:
    - explicit `@plan-file-or-directory`,
    - then the only root `*.md` full plan or declared-ultra direct child
      `*/index.md` entrypoint in the configured plans dir, excluding the
      resolved fast/fix plan paths,
    - then the resolved fast plan path.
- Compare `TaskList` statuses vs plan-entrypoint checkboxes.
    - If code changes for a task appear already implemented but the task is not marked completed, verify quickly and then `TaskUpdate(..., status: "completed")` and update the plan checkbox.
    - If a task is marked completed but the corresponding code is missing (rebase/reset happened), mark it back to pending and discuss with the user.

**If uncommitted changes exist:**

```
AskUserQuestion: You have uncommitted changes. Commit them first?

Options:
1. Yes, commit now ($aif-commit)
2. No, stash and continue
3. Cancel
```

**Based on choice:**

- Yes → run `$aif-commit`, then continue to plan discovery
- No → `git stash push -m "aif-implement: stash before plan execution"`, then continue
- Cancel → inform the user: "Implementation cancelled." → **STOP**

**If NO plan file exists but the resolved fix plan exists:**

A fix plan was created by `$aif-fix` in plan mode. Redirect to fix workflow:

```
Found a fix plan at the resolved fix plan path.

This plan was created by $aif-fix and should be executed through the fix workflow
(it creates a patch and handles cleanup automatically).

Running $aif-fix to execute the plan...
```

→ **Invoke `$aif-fix`** (without arguments – it will detect the resolved fix plan and execute it).
→ **STOP** — do not continue with implement workflow.

**If NO plan file exists AND no resolved fix plan (all tasks completed or fresh start):**

```
AskUserQuestion: No active plan found. Current branch: <current-branch>.
What would you like to do?

Options:
1. Start new feature from current branch
2. Return to configured base branch and start new feature
3. Create quick task plan (no branch)
4. Nothing, just checking status
```

**Based on choice:**

- New feature from current → `$aif-plan full <description>`
- Return to base branch → `git checkout <configured-base-branch>`, then `git pull origin <configured-base-branch>` → `$aif-plan full <description>` (git mode only)
- Quick task → `$aif-plan fast <description>`
- Nothing, just checking status → display branch info and recent commits summary → **STOP**

If `git.enabled = false`, replace option 2 with:

- `2. Create rich full plan without branch creation`
- Route it to `$aif-plan full <description>` without any git commands

**If plan file exists → continue to Step 0.1**

### Step 0.1: Load Project Context & Past Experience

Use the resolved config from Step 0:

- **Paths:** description, architecture, RULES.md, roadmap, research, plan files, patches, and rules dir
- **Language:** `language.ui` for prompts, `language.artifacts` for generated content
- **Rules hierarchy:** the resolved RULES.md file + `rules.base` + named `rules.<area>` entries

**Read `.ai-factory/DESCRIPTION.md`** (use path from config) if it exists to understand:

- Tech stack (language, framework, database, ORM)
- Project architecture and conventions
- Non-functional requirements

**Read the resolved architecture artifact** if it exists (`paths.architecture`, default: `.ai-factory/ARCHITECTURE.md`) to understand:

- Chosen architecture pattern and folder structure
- Dependency rules (what depends on what)
- Layer/module boundaries and communication patterns
- Follow these conventions when implementing — file placement, imports, module boundaries

**Read the resolved RULES.md path** if it exists:

- These are project-specific rules and conventions added by the user
- **ALWAYS follow these rules** when implementing — they override general patterns
- Rules are short, actionable — treat each as a hard requirement

**Read rules hierarchy** (paths from config):

1. **RULES.md** – axioms (universal project rules)
2. **rules/base.md** — project-specific base conventions (naming, structure, patterns)
3. **rules.<area>** — area-specific rule entries resolved from config (for example `rules.api`, `rules.frontend`)

Load all available rule files and merge them. More specific rules override general ones.

**Read `.ai-factory/skill-context/aif-implement/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**

- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the code
  you write and how you update plan checkboxes. If a skill-context rule says "code MUST follow X"
  or "implementation MUST include Y" — you MUST comply. Writing code that violates skill-context
  rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

**Patch fallback (limited, only when skill-context is missing):**

- If `.ai-factory/skill-context/aif-implement/SKILL.md` does not exist and the resolved patches dir exists:
    - Use `Glob` to find `*.md` files in the resolved patches dir
    - Sort patch filenames ascending (lexical), then select the last **10** (or fewer if less exist)
    - Read those selected patch files only
    - Prioritize **Root Cause** and **Prevention** sections
- If skill-context exists, do **not** read all patches by default.
    - Optionally read a few targeted recent patches only when a task clearly matches a known failure pattern.

**Use this context when implementing:**

- Follow the specified tech stack
- Use correct import patterns and conventions
- Apply proper error handling and logging as specified
- Avoid pitfalls documented in skill-context rules and relevant fallback patches

### Step 0.2: Find Plan Artifact

Normalize every selected artifact to:

- `plan_entrypoint` — the single markdown file containing settings and `## Tasks`
- `plan_bundle_dir` — empty for fast/full/fix plans; the ultra directory otherwise
- `phase_files` — empty for single-file plans; ordered links from the ultra
  entrypoint's `## Phase Index` otherwise

**If `$ARGUMENTS` contains `@<path>`:**

1. Resolve the path relative to project root (absolute paths are also valid).
2. Accept an existing markdown file, an ultra directory containing `index.md`,
   or that ultra bundle's `index.md`.
3. If neither representation exists, report the missing path with examples for
   a fast file, full file, and ultra directory, then STOP.
4. Before normalizing a directory or treating an explicit `index.md` as ultra,
   Read `index.md` and require exactly one
   `<!-- aif:plan-mode:ultra -->`; otherwise STOP with a plan-integrity error.
5. If the selected file is `paths.fix_plan`, invoke `$aif-fix` and STOP.
6. Otherwise use it and skip automatic discovery.

**Without an explicit override, resolve in this order:**

```
1. Branch-based full/ultra artifact:
   a. Compute <branch-stem> by replacing "/" with "-".
   b. For workflow.plan_id_format=sequential, glob both:
        <paths.plans>/[0-9][0-9][0-9][0-9]_<branch-stem>.md
        <paths.plans>/[0-9][0-9][0-9][0-9]_<branch-stem>/index.md
      Read every directory candidate and retain it only when `index.md` contains
      exactly one <!-- aif:plan-mode:ultra -->. Choose the highest numeric prefix
      across valid artifacts. If multiple valid candidates exist, emit
      WARN [aif-implement] and name the chosen artifact; if both shapes share
      the highest prefix, prefer ultra.
   c. If no valid sequential candidate exists, or sequential mode is inactive,
      check:
        <paths.plans>/<branch-stem>/index.md
        <paths.plans>/<branch-stem>.md
      Read the directory entrypoint before selection and ignore it unless it
      contains exactly one <!-- aif:plan-mode:ultra -->. If both valid shapes
      exist, emit WARN and prefer the ultra entrypoint.
2. If no branch artifact resolves, count active named artifacts as:
     - each root <paths.plans>/*.md file except resolved paths.plan and paths.fix_plan
     - each direct child <paths.plans>/*/index.md containing <!-- aif:plan-mode:ultra -->
   Exactly one total → use it. More than one → ask the user to choose or use
   @<path>; do not count phase files as independent plans.
3. No named artifact → paths.plan.
4. No regular plan → paths.fix_plan, then redirect to $aif-fix and STOP.
```

Priority remains: explicit path → branch-based full/ultra artifact → single
named full/ultra artifact → fast plan → fix-plan redirect. Discovery scans only
`paths.plans`; archived files and directories under `paths.archive/plans` are
excluded.

**Read the selected artifact:**

- If an automatically discovered directory entrypoint does not contain exactly
  one `<!-- aif:plan-mode:ultra -->`, it is not an AI Factory plan. Ignore it and
  continue discovery. If it contains the marker but its Phase Index is malformed,
  STOP with a plan-integrity error instead of falling back to another plan.
- Always read `plan_entrypoint` completely.
- If it is ultra (contains `<!-- aif:plan-mode:ultra -->`),
  validate every relative Phase Index link, reject paths escaping the bundle,
  record the ordered phase files, and ensure every indexed task maps to exactly
  one `## Task N` section. Warn and STOP on a broken bundle rather than guessing.
- Read a task's complete phase file before implementing that task. On resume,
  re-read the active phase even if it was read in a previous session.
- `index.md` is the only progress source; phase files must not own duplicate
  task checkboxes.
- Treat `## Original Request` as useful original scope context. Executable inputs
  remain settings, dependencies, the task checklist, committed Research Context,
  and (for ultra) linked phase specifications.
- Apply the research-drift contract to Research Context in the entrypoint. Parse
  the first `Source:` / `Reference:` line with canonical
  `^(?:Source|Reference):\s+\x60([^\x60]+)\x60\s+\(` syntax; capture the backtick-delimited
  path so spaces and brackets remain intact. For older bare-path lines, fall back
  to `^(?:Source|Reference):\s+(.+?)\s+\(`. Fall back to `paths.research` only
  when neither form identifies a usable path.
- If the parsed source is inside `research_bundles_dir`, require its sibling
  `INDEX.md` to contain `<!-- aif:research-mode:ultra -->` exactly once and link
  that `RESEARCH.md` from `## Artifact Index`; otherwise emit
  `WARN [research-drift]`. Valid sibling C4/ADR/dependency artifacts remain
  rationale only and do not expand committed scope.
- When `SHA256:` is present, extract the current source text strictly between
  `<!-- aif:active-summary:start -->` and `<!-- aif:active-summary:end -->`,
  remove HTML comment blocks, preserve line order and leading whitespace, trim
  trailing spaces from every line, use LF endings, and end with one newline.
  Hash through stdin with `shasum -a 256` or `sha256sum`; the digest is
  authoritative. Use `Updated:` only as a legacy fallback when `SHA256:` is
  absent. On a missing/invalid source or revision mismatch, emit
  `WARN [research-drift]` and continue using committed plan context.
  In the single-file wording of this contract: emit `WARN [research-drift]` and continue using the plan's embedded Research Context as scope.

**Immediately after reading `plan_entrypoint`, check its first line for
`<!-- handoff:task:<uuid> -->`:**

- In manual mode, extract it and call `handoff_sync_status` with status
  `implementing`, the actual current UTC timestamp, direction
  `aif_to_handoff`, and `paused: true`.
- In autonomous Handoff mode, do not call MCP.
- If absent, skip MCP sync for this session.

When pushing ultra progress to Handoff, serialize the full bundle as entrypoint
content followed by each linked phase file in order, prefixed with
`<!-- ultra-phase:<relative-path> -->`.

### Step 1: Load Current State

```
TaskList → Get all tasks with status
```

Find:

- Next pending task (not blocked, not completed)
- Any in_progress tasks (resume these first)

### Step 2: Display Progress

```
## Implementation Progress

✅ Completed: 3/8 tasks
🔄 In Progress: Task #4 - Implement search service
⏳ Pending: 4 tasks

Current task: #4 - Implement search service
```

### Step 3: Execute Current Task

For each task:

**3.1: Fetch full details**

```
TaskGet(taskId) → Get description, files, context
```

For an ultra bundle, resolve the task's details link from `index.md`, then read
the entire linked phase file before marking the task in progress. Treat its
ordered implementation steps, interfaces, edge cases, logging, acceptance
criteria, and verification as requirements. Do not substitute a new approach
merely because TaskGet contains a shorter summary. If phase instructions conflict
with current code or project context, stop and report the concrete drift; do not
silently make the architectural choice that ultra was meant to pre-plan.

**3.1.1: Run the requirement consistency gate**

Before marking a behavior-changing task in progress:

1. Read the plan's `## Requirements Reconciliation` section when present.
   Resolve citations to the selected research source against the embedded
   `## Research Context`, which remains the committed requirements snapshot.
   Use the live research file only for the Step 0.2 drift warning; never use it
   to change task requirements without an explicit rebase. Re-read relevant
   passages from other cited authoritative sources.
2. Apply a source-priority hierarchy only when it is declared by the user or
   project. Treat roadmap text as scope rather than a detailed contract unless
   explicitly declared otherwise.
3. Compare the task's proposed behavior with the cited requirements. If they
   clearly conflict, emit `ERROR [requirement-conflict]`, leave the task pending,
   and STOP. In manual mode, ask for clarification. In `HANDOFF_MODE=1`, do not
   prompt; return `handoff_outcome: blocked_external` so the coordinator can
   route the task, and do not signal implementation completion or review.
   Do not implement first and rewrite a requirement artifact afterwards.
4. If behavior depends on independent selectors, states, modes, or input shapes,
   confirm that the task covers each applicable supported combination across
   validation, persistence, output/transition, and side effects. Do not infer a
   restriction on one dimension from a rule about another.

If an old plan lacks `## Requirements Reconciliation`, perform this gate from
the task, Original Request, committed Research Context, project rules, and any
authoritative sources they explicitly reference. The embedded Research Context
still wins over its live drift source. Do not expand into unrelated research.

**3.2: Mark as in_progress**

```
TaskUpdate(taskId, status: "in_progress")
```

**3.3: Implement the task**

- Read relevant files
- Make necessary changes
- Follow existing code patterns
- **NO tests unless plan includes test tasks**
- **NO reports or summaries**
- Treat requirement, research, roadmap, rules, and architecture artifacts as
  read-only unless the active task explicitly requires changing the owning
  artifact. This does not block the factual context maintenance explicitly
  allowed later in this workflow, but those updates must not change behavioral
  requirements merely to match the implementation.

**3.4: Verify implementation**

- Check code compiles/runs
- Verify functionality works
- Fix any immediate issues
- Run the verification scenarios assigned to the task. When a representative
  repository artifact defines the contract, exercise that artifact through the
  primary path rather than relying only on synthetic fixtures.

**3.5: Mark as completed**

```
TaskUpdate(taskId, status: "completed")
```

**3.6: Update checkbox in plan entrypoint**

**IMMEDIATELY** after completing a task, update the checkbox in the plan entrypoint
(`index.md` for ultra):

```markdown
# Before

- [ ] Task 1: Create user model

# After

- [x] Task 1: Create user model
```

**This is MANDATORY** — checkboxes must reflect actual progress:

- Use `Edit` tool to change `- [ ]` to `- [x]`
- Do this RIGHT AFTER each task completion
- Even if deletion will be offered later
- Plan entrypoint is the source of truth for progress
- Never add or update duplicate progress checkboxes in ultra phase files

**Handoff sync (manual mode ONLY — skip when `HANDOFF_MODE` is `1`):** If a Handoff task ID was extracted in Step 0.2, call `handoff_push_plan` with `{ taskId: <id>, planContent: <full updated plan text> }` to sync the checklist progress. For ultra, use the bundle serialization defined in Step 0.2.

**3.7: Update the resolved description artifact if needed**

If during implementation:

- New dependency/library was added
- Tech stack changed (e.g., added Redis, switched ORM)
- New integration added (e.g., Stripe, SendGrid)
- Architecture decision was made

→ Update the resolved description artifact (`paths.description`, default: `.ai-factory/DESCRIPTION.md`) to reflect the change:

```markdown
## Tech Stack

- **Cache:** Redis (added for session storage)
```

This keeps the resolved description artifact as the source of truth.

**3.7.1: Update AGENTS.md and ARCHITECTURE.md if project structure changed**

If during implementation:

- New directories or modules were created
- Project structure changed significantly (new `src/modules/`, new API routes directory, etc.)
- New entry points or key files were added

→ Update `AGENTS.md` — refresh the "Project Structure" tree and "Key Entry Points" table to reflect new directories/files.

→ Update the resolved architecture artifact — if new modules or layers were added that should be documented in the folder structure section.

**Only update if structure actually changed** — don't rewrite on every task. Check if new directories were created that aren't in the current structure map.

**3.8: Check for commit checkpoint**

If the plan has commit checkpoints and current task is at a checkpoint:

```
AskUserQuestion: ✅ Tasks <first>-<last> completed. This is a commit checkpoint. Ready to commit? Suggested message: "<conventional commit message>"

Options:
1. Yes, commit now ($aif-commit)
2. No, continue to next task
3. Skip all commit checkpoints
```

**Based on choice:**

- Yes, commit now → invoke `$aif-commit` with the suggested message, then continue to next task
- No, continue to next task → proceed to the next task without committing
- Skip all commit checkpoints → for all subsequent checkpoints within this `$aif-implement` run, skip the prompt automatically and proceed directly to the next task (as if user selected "No, continue to next task" each time). This is in-context memory — resets on `/clear` or new session

**3.9: Move to next task or pause**

### Step 4: Session Persistence

Progress is automatically saved via TaskUpdate.

**To pause:**

```
Current progress saved.

Completed: 4/8 tasks
Next task: #5 - Add pagination support

To resume later, run:
$aif-implement
```

**To resume (next session):**

```
$aif-implement
```

→ Automatically finds next incomplete task

### Step 5: Completion

**Handoff sync (manual mode ONLY — skip entirely when `HANDOFF_MODE` is `1`):** If a Handoff task ID was extracted from the plan annotation AND `HANDOFF_MODE` is NOT `1`:
1. Call `handoff_push_plan` with `{ taskId: <id>, planContent: <final updated plan text> }`; serialize the complete bundle for ultra.
2. If `HANDOFF_SKIP_REVIEW` is `1`: call `handoff_sync_status` with `{ taskId: <id>, newStatus: "done", sourceTimestamp: "<current UTC time in ISO 8601 format>", direction: "aif_to_handoff", paused: false }`.
3. Otherwise: call `handoff_sync_status` with `{ taskId: <id>, newStatus: "review", sourceTimestamp: "<current UTC time in ISO 8601 format>", direction: "aif_to_handoff", paused: true }`.

When all tasks are done:

```
## Implementation Complete

All 8 tasks completed.

Branch: feature/product-search
Plan artifact: .ai-factory/plans/feature-product-search.md
Files modified:
- src/services/search.ts (created)
- src/api/products/search.ts (created)
- src/types/search.ts (created)
Documentation: updated existing docs | created docs/<feature-slug>.md | skipped by user | warn-only (Docs: no/unset)

What's next?

1. 🔍 $aif-verify — Verify nothing was missed (recommended)
2. 💾 $aif-commit — Commit the changes directly
```

**Check ROADMAP.md progress:**

If the resolved roadmap artifact exists:

1. Read it
   1.1. If the plan entrypoint includes `## Roadmap Linkage` with a non-`none` milestone, prefer that milestone for completion marking
2. Check if the completed work corresponds to any unchecked milestone
3. If yes — mark it `[x]` and add entry to the Completed table with today's date
4. Tell the user which milestone was marked done

### Context Maintenance (Artifacts)

Only do this step when there is something concrete to capture.

**DESCRIPTION.md (allowed in this command):**

- If this plan introduced new dependencies/integrations or changed the stack, update the resolved description artifact with factual deltas only.
- Do not rewrite unrelated sections.

**ARCHITECTURE.md + AGENTS.md (allowed in this command):**

- If new modules/layers/folders were added (or dependency rules changed), update the resolved architecture artifact to reflect the new structure and constraints.
- If you maintain `AGENTS.md` structure maps or entry points, refresh them only when they are now incorrect.

**ROADMAP.md (allowed, limited):**

- This command may mark milestone completion when evidence is clear.
- If milestone mapping is ambiguous, emit `WARN [roadmap] ...` and suggest the owner command:
    - `$aif-roadmap check`
    - or `$aif-roadmap <short update request>`

**RULES.md (NOT allowed in this command):**

- Never edit the resolved `paths.rules_file` artifact from `$aif-implement`.
- If you discovered repeatable conventions/pitfalls during implementation, propose up to 3 candidate rules and ask the user to add them via `$aif-rules`.
- Do not invoke `$aif-rules` automatically (it is user-invoked).

If candidate rules exist:

```
AskUserQuestion: Capture new project rules in the resolved RULES.md artifact?

Options:
1. Yes — output `$aif-rules ...` commands (recommended)
2. No — skip
```

**Documentation policy checkpoint (after completion, before plan cleanup):**

Read the plan entrypoint setting `Docs: yes/no`.

If plan setting is `Docs: yes`:

```
AskUserQuestion: Documentation checkpoint — how should we document this feature?

Options:
1. Update existing docs (recommended) — invoke $aif-docs
2. Create a new feature doc page — invoke $aif-docs with feature-page context
3. Skip documentation
```

Handling:

- Option 1 → invoke `$aif-docs` to update README/docs based on completed work
- Option 2 → invoke `$aif-docs` with context to create `docs/<feature-slug>.md`, include sections (Summary, Usage/user-facing behavior, Configuration, API/CLI changes, Examples, Troubleshooting, See Also), and add a README docs-table link
- Option 3 → do not invoke `$aif-docs`; emit `WARN [docs] Documentation skipped by user`

If plan setting is `Docs: no` or setting is unset:

- Do **not** show a mandatory docs checkpoint prompt
- Do **not** invoke `$aif-docs` automatically
- Emit `WARN [docs] Docs policy is no/unset; skipping documentation checkpoint`

**Always include documentation outcome in the final completion output:**

- `Documentation: updated existing docs`
- `Documentation: created docs/<feature-slug>.md`
- `Documentation: skipped by user`
- `Documentation: warn-only (Docs: no/unset)`

**Handle plan artifact after completion:**

- **If the resolved fast plan path** (from `$aif-plan fast`):

  ```
  AskUserQuestion: Would you like to delete the resolved fast plan file? (It's no longer needed)

  Options:
  1. Yes, delete it
  2. No, keep it
  ```

  **Based on choice:**
    - "Yes, delete it" → delete the file:
      ```bash
      rm <resolved fast plan path>
      ```
    - "No, keep it" → leave the file as is, continue to the next step

- **If branch-named full file or ultra bundle directory**:
    - Keep it - documents what was done
    - User can delete before merging if desired

**Check if running in a git worktree:**

Detect worktree context:

```bash
# If .git is a file (not a directory), we're in a worktree
[ -f .git ]
```

**If we ARE in a worktree**, offer to merge back and clean up:

```
You're working in a parallel worktree.

  Branch:    <current-branch>
  Worktree:  <current-directory>
  Main repo: <main-repo-path>

AskUserQuestion: Would you like to merge this branch into the configured base branch and clean up?

Options:
1. Yes, merge and clean up (recommended)
2. No, I'll handle it manually
```

**Based on choice:**

- "Yes, merge and clean up" → follow the Worktree Merge procedure below
- "No, I'll handle it manually" → show a reminder:
  ```
  To merge and clean up later:
    cd <main-repo-path>
    git merge <branch>
    $aif-plan --cleanup <branch>
  ```

#### Worktree Merge

1. **Ensure everything is committed** — check `git status`. If uncommitted changes exist, suggest `$aif-commit` first and wait.

2. **Get repository root path:**

   ```bash
   MAIN_REPO=$(git rev-parse --git-common-dir | sed 's|/\.git$||')
   BRANCH=$(git branch --show-current)
   ```

3. **Switch to the repository root:**

   ```bash
   cd "${MAIN_REPO}"
   ```

4. **Merge the branch:**

   ```bash
   git checkout <configured-base-branch>
   git pull origin <configured-base-branch>
   git merge "${BRANCH}"
   ```

   If merge conflict occurs:

   ```
   ⚠️  Merge conflict detected. Resolve manually:
     cd <main-repo-path>
     git merge --abort   # to cancel
     # or resolve conflicts and git commit
   ```

   → STOP here, do not proceed with cleanup.

5. **Remove worktree and branch (only if merge succeeded):**

   ```bash
   git worktree remove <worktree-path>
   git branch -d "${BRANCH}"
   ```

6. **Confirm:**

   ```
   ✅ Merged and cleaned up!

     Branch <branch> merged into <configured-base-branch>.
     Worktree removed.

   You're now in: <main-repo-path> (<configured-base-branch>)
   ```

→ **STOP** — worktree merged and removed, no further steps needed.

### Final Step — Verify or Commit

```
AskUserQuestion: All tasks complete. What's next?

Options:
1. Verify first — Run $aif-verify to check completeness (recommended)
2. Skip to commit — Go straight to $aif-commit
```

**Based on choice:**

- "Verify first" → invoke `$aif-verify` → after it completes, continue to context cleanup below
- "Skip to commit" → invoke `$aif-commit` → after it completes, continue to context cleanup below

**Context cleanup (after verify or commit):**

Suggest the user to free up context space if needed: `/clear` (full reset) or `/compact` (compress history).

**IMPORTANT: NO summary reports, NO analysis documents, NO wrap-up tasks.**

## Commands

### Start/Resume Implementation

```
$aif-implement
```

Continues from next incomplete task.

### List Available Plans

```
$aif-implement --list
```

Lists the resolved fast/fix plan paths and current-branch full/ultra artifacts
(including sequential identifiers), then exits without implementation.

### Use Explicit Plan Artifact

```
$aif-implement @my-custom-plan.md
$aif-implement @.ai-factory/plans/feature-user-auth.md status
$aif-implement @.ai-factory/plans/feature-user-auth status
```

Uses the provided plan file, ultra directory, or ultra `index.md` instead of
auto-detecting by branch/default artifacts.

### Inline Implementation (No Plan)

```
$aif-implement --without-plan add GET /healthz endpoint returning {"status":"ok"}
$aif-implement --without-plan rename LogLevel.VERBOSE to LogLevel.TRACE --docs=yes
```

One-shot execution of a small task without any plan file. Mutually exclusive with `@plan-file`, `status`, and task id. Does not create `FIX_PLAN.md` or patches. Default docs policy is `warn`; pass `--docs=yes` to run the docs checkpoint, `--docs=no` to silence the warning. See **Step 0.inline** for the full flow.

### Start from Specific Task

```
$aif-implement 5
```

Starts from task #5 (useful for skipping or re-doing).

### Check Status Only

```
$aif-implement status
```

Shows progress without executing.

## Execution Rules

### DO:

- ✅ Execute one task at a time
- ✅ Mark tasks in_progress before starting
- ✅ Mark tasks completed after finishing
- ✅ Follow existing code conventions
- ✅ Follow `$aif-best-practices` guidelines (naming, structure, error handling)
- ✅ Create files mentioned in task description
- ✅ Handle edge cases mentioned in task
- ✅ Stop and ask if task is unclear

### DON'T:

- ❌ Write tests (unless explicitly in task list)
- ❌ Create report files
- ❌ Create summary documents
- ❌ Add tasks not in the plan
- ❌ Skip tasks without user permission
- ❌ Mark incomplete tasks as done
- ❌ Violate the resolved architecture artifact conventions for file placement and module boundaries

## Artifact Ownership Boundaries

- Primary ownership in this command: task execution state and plan progress checkboxes.
- Allowed context artifact updates: the resolved description artifact, the resolved architecture artifact, and roadmap milestone completion in the resolved roadmap artifact when implementation evidence justifies it.
- Read-only context in this command by default: the resolved `paths.rules_file`, configured `paths.research`, and any exact research source linked from the plan entrypoint.
- Context-gate findings should be communicated as `WARN`/`ERROR` outputs only; this does not replace the required verbose implementation logging rules below.

For progress display format, blocker handling, session continuity examples, and full flow examples → see `references/IMPLEMENTATION-GUIDE.md`

## Critical Rules

1. **NEVER write tests** unless task list explicitly includes test tasks
2. **NEVER create reports** or summary documents after completion
3. **ALWAYS mark task in_progress** before starting work
4. **ALWAYS mark task completed** after finishing
5. **ALWAYS update checkbox in plan file** - `- [ ]` → `- [x]` immediately after task completion
6. **PRESERVE progress** - tasks survive session boundaries
7. **ONE task at a time** - focus on current task only

## CRITICAL: Logging Requirements

**ALWAYS add verbose logging when implementing code.** For logging guidelines, patterns, and management requirements → read `references/LOGGING-GUIDE.md`

Key rules: log function entry/exit, state changes, external calls, error context. Use structured logging, configurable log levels (LOG_LEVEL env var).

**DO NOT skip logging to "keep code clean" - verbose logging is REQUIRED during implementation, but MUST be configurable.**
