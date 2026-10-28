---
name: aif-verify
description: >-
  Verify completed implementation against the plan. Checks that all tasks were implemented,
  nothing was forgotten, code compiles, tests pass, and quality standards are met.
  Use after "$aif-implement" completes, or when user says "verify", "check work", "did we miss anything".
argument-hint: "[--strict]"
allowed-tools: Read Edit Glob Grep Bash(git *) Bash(npm *) Bash(npx *) Bash(yarn *) Bash(pnpm *) Bash(bun *) Bash(go *) Bash(python *) Bash(php *) Bash(composer *) Bash(cargo *) Bash(make *) Bash(task *) Bash(just *) Bash(mage *) Bash(shasum -a 256 *) Bash(sha256sum *) TaskList TaskGet AskUserQuestion Questions
disable-model-invocation: false
metadata:
  author: AI Factory
  version: "1.0"
  category: quality
---

<!-- aif-ext:aifhub-extension:aif-verify:prepend:start -->
## AIFHub Verify OpenSpec-native Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-verify` body. When this guidance conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Use the built-in `/aif-verify` skill as the canonical verification command for OpenSpec-native changes and the extension's legacy companion plan workflow.

### Configured Validation Providers

`aifhub.providers` is independent of `aifhub.tools.openspec`: OpenSpec and HLV may be enabled together. Each provider is opted in by its boolean under `aifhub.tools` (`hlv: true` or `lekalo: true`); false or an omitted tool disables invocation and new evidence writes. Enabled providers default to `policy: required`; `policy: optional` changes failures to nonblocking warnings. After native project gates, before emitting the final verify verdict, run `ai-factory aifhub-providers verify --change <change-or-plan-id> --write --json`. Resolve one safe change/plan ID first. The wrapper is a no-op when no provider is enabled for verify. Apply its normalized `blocking` and provider gate statuses to final verification: required unavailable/unsupported/failed providers block PASS; optional failures produce degraded notes. Provider failures remain separate from implementation/test failures. Retain original diagnostic codes from normalized operations; never copy raw streams into `verify.md`.

Provider QA files are derived evidence under `.ai-factory/qa/<id>/providers/`; no provider output may rewrite OpenSpec, HLV, or Lekalo canonical artifacts. Missing enabled-tool scaffolding is initialized through `ai-factory aifhub-mode init --json` before validation and revision binding, following the shared tool contract. Reuse existing HLV projects without reinit. Never install or update provider binaries or run provider-owned sync automatically. `hlv check` can execute project-configured gates, so it runs only in an explicitly enabled validation phase, never as detection or doctor. Lekalo remains `unsupported` until its versioned provider protocol is published.

For upstream ultra bundles, preserve the bundle and upstream verification ownership. Explicitly configured provider evidence is a separate post-verification overlay: record the upstream receipt first and report a provider blocker separately without rewriting the upstream bundle or its verdict.

### Skill-Context Resolution

Read skill-context in this order:

1. `.ai-factory/skill-context/aif-verify/SKILL.md`
2. `.ai-factory/skill-context/aif-verify-plus/SKILL.md` as legacy compatibility fallback

If both exist, `aif-verify` wins.

### Mode Detection

Before resolving verification scope, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with Legacy AI Factory-only mode and state that no OpenSpec-native protocol was detected.

### Legacy ultra marker-first boundary

In Legacy AI Factory-only mode, classify the normalized project-relative plan entrypoint with `classifyLegacyPlanShape()` from `scripts/legacy-plan-migration.mjs` before folder-only detection, companion discovery, companion plan-file creation, verification-file selection, or any `status.yaml`/`verify.md` write. Marker validation is first; known companion filenames are considered only after the classifier returns a classic shape.

- For `ultra-valid`, stop all AIFHub companion logic and return the exact upstream handoff `/aif-verify <entrypoint>`. Upstream owns bundle verification atomically. After upstream verification emits its one final validated `aif-gate-result`, pass that exact structured gate outcome and the normalized entrypoint to `writeLegacyUltraVerificationReceipt()` from `scripts/legacy-ultra-verification-receipt.mjs`. Record both `pass` and non-pass outcomes; the helper binds the receipt to the current bundle, source revision, and deterministic worktree and writes only `.ai-factory/state/legacy-ultra-verification/<entrypoint-digest>.json`. A missing or invalid final gate must not create a receipt.
- The receipt and explicitly configured provider overlay are the only AIFHub writes permitted for `ultra-valid`. Do not create or synchronize a sibling `<plan-id>.md`, `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, `explore.md`, any OpenSpec artifact, unrelated QA evidence, or finalization artifact. Never edit the ultra bundle while recording the receipt.
- For `ultra-invalid` or `collision`, fail closed before any write and report only bounded `shape`, safe `entrypoint`, and classifier `code` values.
- For `classic-pair` or `classic-folder-only`, continue with the classic companion rules below. An unrelated directory is not a plan.
- Diagnostics may include only `shape`, safe project-relative `entrypoint`, and `handoff`; never include marker bodies, phase contents, request/research bodies, credentials, raw stdout, or raw stderr.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-verify` verifies implementation against the active OpenSpec change.

For a change with SDD inputs/policy or existing SDD runtime artifacts, check `ai-factory aifhub-session-brief status --change <change-id> --json` before consuming its brief. Missing/stale/invalid context or an unavailable helper returns a planning-owner handoff; verification must not compile or repair briefs. The brief is supporting context, never QA evidence. Read canonical requirements/scenarios, `aif-gate-result`, `coverage.json`, `done-readiness.json`, and verification receipts at full fidelity without semantic compaction. Quick planning cannot disable project-required tests/security/review/human approval, migration/rollback, `/aif-verify`, or `/aif-done`. Unopted changes retain canonical filesystem fallback, and `/aif-verify` remains authoritative.

For plan content, read only the canonical OpenSpec artifacts listed below. Do not inspect or mutate `.ai-factory/plans/**` after an OpenSpec change resolves.

Before running lint, tests, code review, security review, or rules review, resolve the active change, ensure runtime layout, and use `scripts/openspec-verification-context.mjs` with `scripts/openspec-runner.mjs` when available. Request validation through `validateOpenSpecChange(changeId)` and status through `getOpenSpecStatus(changeId)` from the shared runner. Resolve effective policy through `scripts/openspec-policy.mjs`. Fail invalid OpenSpec artifacts before code checks. Treat missing CLI, generated rules, rules gate evidence, and coverage evidence as degraded warnings unless strict config such as `aifhub.openspec.requireCliForVerify` or the matching verify policy flag requires failure. Use `shouldRunCodeVerification` as the handoff signal: `false` blocks code checks and routes to `/aif-fix <change-id>`; `true` allows normal code verification to continue.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Resolve the active change using `scripts/active-change-resolver.mjs` when available:

- Prefer an explicit `<change-id>` or `@openspec/changes/<change-id>` input when provided.
- Otherwise use `resolveActiveChange` behavior: current working directory, current branch mapping, current pointer, then single active change.
- Treat selected source, candidate list, warnings, and errors as user-visible verification context.
- If an explicit or inferred `<change-id>` cannot be resolved as an OpenSpec change, check for matching legacy AI Factory plan artifacts through `detectMigrationNeed(options)` from `scripts/legacy-plan-migration.mjs` or equivalent read-only detection. If migration is suggested, do not auto-migrate. Show exactly:

```text
Found legacy AI Factory plan artifacts for `<change-id>` but no OpenSpec change at `openspec/changes/<change-id>`.
Run the legacy migration script with:

ai-factory aifhub-migrate-legacy-plans <change-id> --dry-run
ai-factory aifhub-migrate-legacy-plans <change-id>
```

Validate and review against canonical OpenSpec artifacts:

- `openspec/specs/**`
- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`

Treat planning source sections as read-only verification context:

- Use `## Original Request` as the raw intent anchor, while canonical specs, design, tasks, and implemented behavior remain the verification contract.
- When `proposal.md` contains `## Research Context`, use the embedded snapshot and source revision as authoritative committed scope.
- When `proposal.md` contains `## Requirements Reconciliation`, verify the requirement chain it records: the cited authoritative requirements, plan decisions, implemented behavior, and verification evidence must stay consistent, including each material combination of independent selectors, states, modes, or input shapes the section enumerates. When repository data, configuration, fixtures, schemas, or content define the contract, verify at least one representative existing artifact through the primary integration path when such an artifact is available and relevant.
- Sources conflicting without a declared authority emit `WARN [requirement-ambiguity]` in normal mode and `ERROR [requirement-ambiguity]` in strict mode; do not guess between them. A clear contradiction between the implementation and an authoritative requirement emits `ERROR [requirement-conflict]` and blocks verification even when plan, code, and tests agree with one another.
- For an embedded ultra source, pass its exact project-relative `RESEARCH.md` path to `resolveUltraResearchSource()` from `scripts/ultra-research-resolver.mjs` and consume only its structured `source`, `revision`, and `diagnostic`. This centrally revalidates the sibling marker/index/status/link and normalized Active Summary digest; do not implement local selection or hashing heuristics.
- Compare the exact regular or ultra source only for drift and rationale. Missing/invalid source, changed `Updated`, or changed normalized `SHA256` emits `WARN [research-drift] change-id=<change-id> source=<path> expected=<embedded revision> current=<live revision>` without expanding verification scope. Recency never selects a replacement source.
- Do not mutate or silently rebase either source section or `## Requirements Reconciliation`. Keep credentials, raw provider output, and full request/research bodies out of QA diagnostics.

#### Roadmap linkage validation

Roadmap linkage validation is read-only in OpenSpec-native mode.

- Read the canonical proposal's standardized `## Roadmap Linkage` fields and, when present, compare them with bounded managed lifecycle evidence from the configured roadmap.
- `/aif-verify` must not edit the configured roadmap, create a managed lifecycle row, or change milestone, phase, or slice state.
- For malformed linkage or lifecycle markers, emit a bounded `WARN [roadmap]` when the condition is non-blocking. Use `ERROR [roadmap]` only when contradictory local evidence or an explicit canonical requirement blocks verification. Include the `change-id`, a bounded reason, and the exact handoff `/aif-roadmap check`.
- Diagnostics must not copy the managed lifecycle block, full proposal body, unrelated roadmap content, credentials, or private provider output.
- Missing linkage alone remains a warning unless a canonical requirement makes linkage mandatory; do not invent issue, milestone, or roadmap assignments to remove the warning.

Read generated rules as derived verification guidance when present:

- `.ai-factory/rules/generated/openspec-merged-<change-id>.md`
- `.ai-factory/rules/generated/openspec-change-<change-id>.md`
- `.ai-factory/rules/generated/openspec-base.md`

Runtime state and QA evidence boundaries:

- Read implementation runtime state from `.ai-factory/state/<change-id>/` when present.
- Write verification findings, verdicts, command results, and review evidence only under `.ai-factory/qa/<change-id>/`.
- Record OpenSpec validation/status evidence under `.ai-factory/qa/<change-id>/` before code verification.
- Build and write the OpenSpec coverage matrix at `.ai-factory/qa/<change-id>/coverage.json` using `scripts/openspec-coverage-matrix.mjs` when available.
- Treat missing requirement coverage as `fail` in strict mode and `warn` in normal mode.
- Read durable rules gate evidence from `.ai-factory/qa/<change-id>/rules.md` when present; generated rules readiness does not satisfy `requireRulesPassForVerify`.
- Do not write QA evidence or runtime-only files into `openspec/changes/<change-id>/`.
- Do not archive. `/aif-verify` records verification evidence only; `/aif-done <change-id>` owns OpenSpec archive/finalization.
- Do not create legacy plan-folder verification artifacts in OpenSpec-native mode.

Normal verification responses should report:

- selected `change-id` and resolver source;
- canonical artifacts inspected;
- generated rules freshness or missing/stale `WARN`;
- effective policy summary and any policy-derived blockers or warnings;
- OpenSpec validation status and `shouldRunCodeVerification`;
- coverage summary and policy result;
- QA evidence path under `.ai-factory/qa/<change-id>/`;
- verdict and finding counts;
- fix guidance `/aif-fix <change-id>` when verification fails;
- optional finalization guidance `/aif-done <change-id>` when verification passes.

Optional read-only gates are available before verification starts: `/aif-rules-check`, `/aif-review`, and `/aif-security-checklist`. The authoritative final verification remains `/aif-verify <change-id>`. This next-step routing is one-way with terminal states: when verification passes, suggest `/aif-done <change-id>` and do not suggest a rerun of `/aif-rules-check` or `/aif-verify`; when verification fails, route to `/aif-fix <change-id>`; the single fail-path exception is a blocking failure caused by missing, stale, or invalid durable rules gate evidence, where the recovery step is rerunning `/aif-rules-check <change-id>` and persisting its final gate block through `ai-factory aifhub-write-gate-evidence` — this exception is prose guidance and is never encoded into the machine-readable `suggested_next` field; do not suggest `/aif-rules-check` as remediation for implementation findings after verification has already run. The final `aif-gate-result` block keeps `suggested_next` `null` when `status` is `pass`; the `/aif-done <change-id>` suggestion stays in prose, never inside the gate result block.

End verification output and `.ai-factory/qa/<change-id>/verify.md` with exactly one final fenced `aif-gate-result` JSON block using `"gate": "verify"` and lowercase JSON `status`: `pass`, `warn`, or `fail`. Use `fail` for blocking OpenSpec validation, coverage, generated-rules, test, lint, build, review, security, or rules failures; use `warn` only for non-blocking notes after verification completes.

Do not install OpenSpec skills or slash commands.
Do not redirect the user to legacy finalize aliases.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve the current companion plan verification contract.

Resolve the active target as a companion pair:

- `.ai-factory/plans/<plan-id>.md`
- `.ai-factory/plans/<plan-id>/`

If verification enters through a legacy folder-only plan, create the missing companion plan file before verification and record the migration in `status.yaml.history`.

Plan-folder contract:

- read `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, optional `constraints-*.md`, optional `explore.md`
- update only `status.yaml` and `verify.md`
- keep source code and project context files read-only

Workflow integration:

- In the extension workflow, `/aif-implement` hands off to `/aif-verify`.
- Route failing verification to `/aif-fix`.
- On `PASS` or `PASS with notes`, stop at the verified state and recommend `/aif-done` only when archive/commit/PR/follow-up finalization is needed.
- Never archive into `.ai-factory/specs/`, never create `spec.md`, never update `specs/index.yaml`, and never set `status.yaml.status` to `done`.
- When `--check-only` is present, keep the same no-archive behavior and return a verification-only gate result for downstream review/finalization flows.
- Do not redirect the user to legacy finalize aliases, and do not present `/aif-done` as a replacement for `/aif-verify`; `/aif-done` is an optional post-verify AIFHub finalizer.
<!-- aif-ext:aifhub-extension:aif-verify:prepend:end -->

# Verify — Post-Implementation Quality Check

Verify that the completed implementation matches the plan, nothing was missed, and the code is production-ready.

**This skill is optional** — invoked after `$aif-implement` finishes all tasks, or manually at any time.

---

## Step 0: Load Context

### 0.0 Load config.yaml

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.description`, `paths.architecture`, `paths.rules_file`, `paths.roadmap`, `paths.research`, `paths.plan`, `paths.plans`, `paths.fix_plan`, `paths.specs`, `paths.rules`, and `paths.archive`; derive `research_bundles_dir = <parent directory of paths.research>/research/`
- **verify_mode:** default verification strictness (`strict` | `normal` | `lenient`)
- **Git:** `git.enabled`, `git.base_branch`, `git.create_branches`
- **Rules hierarchy:** the resolved RULES.md path + `rules.base` + named `rules.<area>` entries
- **Language:** `language.ui` for prompts, user-visible explanations, verification reports, context-gate summaries, issue remediation prompts, and next-step guidance
- **Workflow:** `workflow.plan_id_format` (default: `slug`) — used by branch-based plan discovery in Step 0.2.
  Active values: `slug` and `sequential`. Discovery treats root `*.md` files as
  full plans except the resolved fast/fix paths, and direct child `*/index.md`
  files as ultra entrypoints only when they contain exactly one
  `<!-- aif:plan-mode:ultra -->`. When
  `sequential`, search both numbered shapes and choose the highest prefix.
  `timestamp` and `uuid` are **reserved values** and currently behave like `slug`.
  Treat any unknown value as `slug`.

**verify_mode priority:**
1. `--strict` CLI flag → always use `strict`
2. config.yaml `workflow.verify_mode` → use configured value
3. Default → `normal`

If config.yaml doesn't exist, use defaults:
- Paths: `.ai-factory/` for all artifacts
- research: `.ai-factory/RESEARCH.md`
- verify_mode: `normal`
- Rules: RULES.md only
- `ui_language`: `en`
- `workflow.plan_id_format`: `slug`

Resolved language value:
- `ui_language = language.ui || "en"`

All AskUserQuestion prompts, user-visible explanations, verification reports, context-gate summaries, issue remediation prompts, and next-step guidance MUST be written in `ui_language`.

Preserve machine-readable `aif-gate-result` JSON schema fields and enum values (`pass`, `warn`, `fail`) unchanged. Preserve `WARN`/`ERROR` gate labels, commands, paths, config keys, code identifiers, package names, API names, and raw command output unchanged.

### 0.1 Load Ownership and Gate Contract

- Read `references/CONTEXT-GATES-AND-OWNERSHIP.md` first.
- Read `references/GATE-RESULT-CONTRACT.md` for the machine-readable quality gate summary.
- Treat it as the canonical source for:
  - command-to-artifact ownership,
  - read-only behavior for `aif-commit`/`aif-review`/`aif-verify`,
  - normal vs strict context-gate thresholds.
- If this contract conflicts with older examples in this file, follow the contract.

### 0.2 Find Plan Artifact

Same logic as `$aif-implement` — produce the **canonical branch stem** before any plans-dir glob so producer and consumers agree by construction.

```
1. Check current git branch:
   git branch --show-current
2. Convert branch to filename stem (git mode only):
   branch_stem = current branch with every "/" replaced by "-"
   Example: feature/user-auth → feature-user-auth
3. Resolve the plan artifact using <branch_stem>:
   → When `workflow.plan_id_format = sequential`, glob both
       <configured plans dir>/[0-9][0-9][0-9][0-9]_<branch_stem>.md
       <configured plans dir>/[0-9][0-9][0-9][0-9]_<branch_stem>/index.md
     Read every directory candidate and retain it only when `index.md` contains
     exactly one <!-- aif:plan-mode:ultra -->. Choose the highest prefix across
     valid artifacts and warn when multiple valid candidates exist; prefer ultra
     if both shapes share the highest prefix.
   → Otherwise/fallback check the ultra entrypoint and full file:
       <configured plans dir>/<branch_stem>/index.md
       <configured plans dir>/<branch_stem>.md
     Read the directory entrypoint before selection and ignore it unless it
     contains exactly one <!-- aif:plan-mode:ultra -->. If both valid shapes
     exist, warn and prefer ultra.
4. If the branch-based plan is missing or git mode is off:
   → Count root `*.md` full plans and declared-ultra direct child `*/index.md`
     entrypoints as artifacts; exclude resolved fast/fix paths and do not count
     phase files
   → If exactly one exists, use it
   → If multiple exist, ask the user to choose or use `@<path>` via `$aif-implement`
5. No named full/ultra plan → Check the resolved fast plan path
6. No regular plan and no resolved fast plan → fall back to standalone verification choices
```

**Note:** Plan discovery scans `paths.plans/` only. Plans archived to `paths.archive/plans/` by `$aif-archive` are excluded from discovery. If a plan is found only in the archive, emit `WARN [aif-verify] plan <name> is archived; verifying archived plan`.

**If no plan artifact is found:**
```
AskUserQuestion: No plan artifact found. What should I verify?

Options:
1. Verify last commit — Check the most recent commit for completeness
2. Verify branch diff — Compare current branch against the configured base branch
3. Cancel
```

### 0.3 Read Plan & Tasks

- Read the selected plan entrypoint to understand what was supposed to be implemented
- An automatically discovered directory entrypoint is a plan only when it
  contains `<!-- aif:plan-mode:ultra -->`; ignore unrelated `*/index.md` files.
- For ultra, validate all Phase Index links and read every linked phase file
  before verification. Verify implementation against the detailed per-task
  interfaces, edge cases, logging, acceptance criteria, and commands—not only
  the short checkbox text in `index.md`. A missing/broken/escaping phase link is
  a blocking plan-integrity failure because the committed specification is incomplete.
- Cross-check every `Task N` in `index.md` against exactly one matching
  `## Task N` section in its linked phase file. Unmapped tasks, duplicate task
  sections, and unlinked `phase-*.md` files are blocking plan-integrity failures.
- `TaskList` → get all tasks and their statuses
- Read `.ai-factory/DESCRIPTION.md` (use path from config) for project context (tech stack, conventions)
- Read `.ai-factory/ARCHITECTURE.md` (use path from config) for dependency and boundary rules (if present)
- Read **rules hierarchy** (use paths from config):
  1. **RULES.md** — axioms (universal project rules)
  2. **rules/base.md** — project-specific base conventions
  3. **rules.<area>** — area-specific rule entries resolved from config (for example `rules.api`, `rules.frontend`)
- Read `.ai-factory/ROADMAP.md` (use path from config) for milestone alignment checks (if present)
- If the plan entrypoint contains `## Original Request`, treat it as useful original scope context. Use it to understand the user's starting intent, while the task list, committed `## Research Context`, and ultra phase specifications remain the executable verification inputs.
- If the plan entrypoint contains `## Research Context`, treat its embedded copy as the committed requirements snapshot. Parse the first `Source:` / `Reference:` line with canonical `^(?:Source|Reference):\s+\x60([^\x60]+)\x60\s+\(` syntax; for older bare-path lines, fall back to `^(?:Source|Reference):\s+(.+?)\s+\(`. Fall back to configured `paths.research` only when neither form identifies a usable path.
- If the parsed source is inside `research_bundles_dir`, require its sibling `INDEX.md` to contain `<!-- aif:research-mode:ultra -->` exactly once and link that `RESEARCH.md` from `## Artifact Index`; otherwise emit `WARN [research-drift]`. Sibling C4/ADR/dependency files remain rationale only.
- When `SHA256:` is present, extract the current source text strictly between `<!-- aif:active-summary:start -->` and `<!-- aif:active-summary:end -->`, remove HTML comment blocks, preserve line order and leading whitespace, trim trailing spaces from every line, use LF endings, and end with one newline. Hash through stdin with `shasum -a 256` or `sha256sum`; the digest is authoritative. Use `Updated:` only as a legacy fallback when `SHA256:` is absent. A missing/invalid source or revision mismatch emits `WARN [research-drift]`; verify against the embedded Research Context and do not expand scope unless the user explicitly requests a rebase/refinement. Skipping this drift check is a verification bug.
- Compatibility wording for the same rule: emit `WARN [research-drift]` and
  verify against the plan's embedded Research Context.

**Read `.ai-factory/skill-context/aif-verify/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the Verification
  Report template. If a skill-context rule says "verification MUST check X" or "report MUST include
  section Y" — you MUST augment the report accordingly. Generating a verification that ignores
  skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

### 0.3 Gather Changed Files

```bash
# All files changed during this feature/plan
git diff --name-only <configured-base-branch>...HEAD
# Or if on the base branch / in no-git mode, check recent commits
git diff --name-only HEAD~$(number_of_tasks)..HEAD
```

If `git.enabled = false`, skip branch diffing entirely and gather changed files from:
- the working tree (if uncommitted changes exist), or
- the recent commit window that corresponds to the implemented tasks.

Store as `CHANGED_FILES`.

---

## Step 1: Task Completion Audit

Go through **every task** in the plan and verify it was actually implemented.

For each task:

### 1.1 Read Task Description

```
TaskGet(taskId) → Get full description, requirements, acceptance criteria
```

### 1.2 Verify Implementation Exists

For each requirement in the task description:
- Use `Glob` and `Grep` to find the code that implements it
- Read the relevant files to confirm the implementation is complete
- Check that the implementation matches what was described, not just that "something was written"

### 1.3 Build Checklist

For each task, produce a verification result:

```
✅ Task #1: Create user model — COMPLETE
   - User model created at src/models/user.ts
   - All fields present (id, email, name, createdAt, updatedAt)
   - Validation decorators added

⚠️ Task #3: Add password reset endpoint — PARTIAL
   - Endpoint created at src/api/auth/reset.ts
   - MISSING: Email sending logic (task mentioned SendGrid integration)
   - MISSING: Token expiration check

❌ Task #5: Add rate limiting — NOT FOUND
   - No rate limiting middleware detected
   - No rate-limit related packages in dependencies
```

Statuses:
- `✅ COMPLETE` — all requirements verified in code
- `⚠️ PARTIAL` — some requirements implemented, some missing
- `❌ NOT FOUND` — implementation not detected
- `⏭️ SKIPPED` — task was intentionally skipped by user during implement

---

## Step 2: Code Quality Verification

### 2.1 Build & Compile Check

Detect the build system and verify the project compiles:

| Detection | Command |
|-----------|---------|
| `go.mod` | `go build ./...` |
| `tsconfig.json` | `npx tsc --noEmit` |
| `package.json` with `build` script | `npm run build` (or pnpm/yarn/bun) |
| `pyproject.toml` | `python -m py_compile` on changed files |
| `Cargo.toml` | `cargo check` |
| `composer.json` | `composer validate` |

If build fails → report errors with file:line references.

### 2.2 Test Check

If the project has tests and they were part of the plan:

| Detection | Command |
|-----------|---------|
| `jest.config.*` or `vitest` | `npm test` |
| `pytest` | `pytest` |
| `go test` | `go test ./...` |
| `phpunit.xml*` | `./vendor/bin/phpunit` |
| `Cargo.toml` | `cargo test` |

If tests fail → report which tests failed and whether they relate to the implemented tasks.

If no tests exist or testing was explicitly skipped in the plan → note it but don't fail.

### 2.3 Lint Check

If linters are configured:

| Detection | Command |
|-----------|---------|
| `eslint.config.*` / `.eslintrc*` | `npx eslint [changed files]` |
| `.golangci.yml` | `golangci-lint run ./...` |
| `ruff` in pyproject.toml | `ruff check [changed files]` |
| `.php-cs-fixer*` | `./vendor/bin/php-cs-fixer fix --dry-run --diff` |

Only lint the changed files to keep output focused.

### 2.4 Import & Dependency Check

- Verify no unused imports were left behind
- Check that new dependencies mentioned in tasks were actually added (`package.json`, `go.mod`, `requirements.txt`, `composer.json`)
- Check for missing dependencies (imports that reference packages not in dependency files)

---

## Step 3: Consistency Checks

### 3.0 Requirements Provenance and Semantic Consistency

Verify the full chain `authoritative requirements → plan → implementation →
verification evidence`, not only plan-to-code agreement.

1. If the plan contains `## Requirements Reconciliation`, resolve citations to
   the selected research source against the embedded `## Research Context`; the
   live research file remains drift-only unless the user explicitly rebases the
   plan. Re-read other cited source passages. Otherwise use the Original Request,
   committed Research Context, project rules, and authoritative sources they
   explicitly reference.
2. Follow a source-priority hierarchy only when declared by the user or project.
   Treat roadmap text as scope rather than a detailed behavioral contract unless
   explicitly declared otherwise. If sources conflict without declared
   authority, do not guess: emit `WARN [requirement-ambiguity]` in normal or
   lenient mode, and `ERROR [requirement-ambiguity]` in strict mode.
3. A clear contradiction between the implementation and a higher-priority
   requirement is `ERROR [requirement-conflict]` and blocks verification even
   when the plan, code, and tests agree with one another.
4. When behavior depends on independent selectors, states, modes, or input
   shapes, enumerate only the supported combinations and verify each material
   combination across validation, persistence, output/transition, and side
   effects. Separate tests for each dimension do not prove their interaction.
5. When repository data, configuration, fixtures, schemas, or content define the
   contract, verify at least one representative existing artifact through the
   primary integration path when such an artifact is available and relevant.

Respect the plan's testing setting. If tests were intentionally skipped, use
static inspection plus the plan's runnable/manual verification evidence; report
missing evidence as a warning in normal mode and a failure in strict mode.

### 3.1 Plan vs Code Drift

Check for discrepancies between what the plan says and what was built:

- **Naming**: Do variable/function/endpoint names match what the plan specified?
- **File locations**: Are files where the plan said they should be?
- **API contracts**: Do endpoint paths, request/response shapes match the plan?

### 3.2 Leftover Artifacts

Search for things that should have been cleaned up:

```
Grep in CHANGED_FILES: [T][O][D][O]|[F][I][X][M][E]|HACK|[X][X][X]|TEMP|PLACEHOLDER|console\.log\(.*debug|print\(.*debug
```

Report any found — they might be intentional, but flag them.

### 3.3 Configuration & Environment

Check if the implementation introduced any new config requirements:

- New environment variables referenced but not documented
- New config files mentioned in code but not created
- Database migrations created but not documented in README/docs

```
Grep in CHANGED_FILES: process\.env\.|os\.Getenv\(|os\.environ|env\(|getenv\(|config\(
```

Cross-reference with `.env.example`, `.env.local`, README, or docs to ensure they're documented.

### 3.4 DESCRIPTION.md Sync

Check if `.ai-factory/DESCRIPTION.md` reflects the current state:

- New dependencies/libraries added during implementation → should be listed
- Architecture changes → should be reflected
- New integrations → should be documented

### 3.5 Context Gates (Architecture / Roadmap / Rules)

Apply the canonical contract from `references/CONTEXT-GATES-AND-OWNERSHIP.md`.

Evaluate and report each gate explicitly:

- **Architecture gate**
  - Pass: implementation follows documented boundaries and dependency rules
  - Warn: architecture mapping is ambiguous or stale
  - Fail: clear violation of explicit architecture constraints

- **Rules gate**
  - Pass: implementation follows explicit project rules
  - Warn: relevance/verification is ambiguous
  - Fail: clear violation of explicit rule text

- **Roadmap gate**
  - Pass: work aligns with existing milestone direction (prefer `## Roadmap Linkage` from the plan when present)
  - Warn: `.ai-factory/ROADMAP.md` missing, ambiguous mapping, or no milestone linkage for `feat`/`fix`/`perf` scope
  - Fail (strict mode): clear roadmap contradiction after all available roadmap context is considered

Normal mode behavior:
- Architecture/rules clear violations fail verification.
- Roadmap mismatch and missing milestone linkage are warnings unless contradiction is explicit and severe.

Strict mode behavior:
- Architecture and rules clear violations fail verification.
- Clear roadmap mismatch fails verification.
- Missing milestone linkage for `feat`/`fix`/`perf` remains a warning (even when `.ai-factory/ROADMAP.md` exists).

Human logging/reporting format:
- Non-blocking findings: `WARN [gate-name] ...`
- Blocking findings: `ERROR [gate-name] ...`

If the user wants a standalone rules-only pass, suggest `$aif-rules-check`. Keep human context-gate labels at `WARN` / `ERROR`, then derive the final machine-readable gate result from the full verification report.

Machine-readable gate result:
- Append one final fenced `aif-gate-result` JSON block after the human-readable verification report.
- Use `"gate": "verify"`.
- Use `"status": "pass|warn|fail"` where:
  - `fail` = incomplete required tasks, failed blocking quality checks, strict-mode context gate failures, or other blockers requiring remediation.
  - `warn` = only non-blocking warnings remain, optional checks were skipped, docs/test gaps were accepted as warnings, or context drift is ambiguous.
  - `pass` = no blocking or warning findings remain.
- Use `"blocking": true|false`; set it to `true` only when the result should stop commit or merge flow.
- Include only blocking findings in `"blockers": [`; keep non-blocking notes in the human summary.
- Include changed or implicated paths in `"affected_files": [`.
- Set `"suggested_next": {` to `$aif-fix`, `$aif-rules`, `$aif-architecture`, `$aif-roadmap`, `$aif-commit`, or `null` according to `references/GATE-RESULT-CONTRACT.md`.

### 3.6 Context Drift (Optional Remediation)

`$aif-verify` is **read-only** for context artifacts. Do not edit or regenerate `.ai-factory/*` files here.

If you detect that a context artifact is stale, missing, or ambiguous, report it as a drift finding and provide the owner-command remediation:

- `DESCRIPTION.md` drift → suggest `$aif` (or note that `$aif-implement` should have updated it during implementation)
- `ARCHITECTURE.md` drift → suggest `$aif-architecture`
- `ROADMAP.md` drift → suggest `$aif-roadmap check` (or `$aif-roadmap <update request>`)
- `RULES.md` drift → suggest `$aif-rules <rule text>`

Ask the user a single optional question **only if** drift was detected and fixing it now would materially improve correctness:

```
AskUserQuestion: Context drift detected. Capture updates now?

Options:
1. Yes — show the exact commands to run (recommended)
2. No — proceed without updating context
```

---

## Step 4: Verification Report

### 4.1 Display Results

Write the human-readable verification report in `ui_language`. The template below defines structure only; keep stable technical tokens and the final `aif-gate-result` JSON schema unchanged.

```
## Verification Report

### Task Completion: 7/8 (87%)
| # | Task | Status | Notes |
|---|------|--------|-------|
| 1 | Create user model | ✅ Complete | |
| 2 | Add registration endpoint | ✅ Complete | |
| 3 | Add password reset | ⚠️ Partial | Missing: email sending |
| 4 | Add JWT auth middleware | ✅ Complete | |
| 5 | Add rate limiting | ✅ Complete | |
| 6 | Add input validation | ✅ Complete | |
| 7 | Add error handling | ✅ Complete | |
| 8 | Update API docs | ❌ Not found | No changes in docs/ |

### Code Quality
- Build: ✅ Passes
- Tests: ✅ 42 passed, 0 failed
- Lint: ⚠️ 2 warnings in src/api/auth/reset.ts

### Requirements Consistency
- Source reconciliation: ✅ Passes
- Supported combinations: ✅ Covered
- Representative artifact path: ✅ Verified / ⏭️ Not applicable

### Issues Found
1. **Task #3 incomplete** — Password reset endpoint created but email sending not implemented (SendGrid integration missing)
2. **Task #8 not done** — API documentation not updated despite plan requirement
3. **2 unfinished markers found** — src/services/auth.ts:45, src/middleware/rate-limit.ts:12
4. **New env var undocumented** — `SENDGRID_API_KEY` referenced but not in .env.example

### No Issues
- All imports resolved
- No unused dependencies
- DESCRIPTION.md up to date
- No leftover debug logs
```

### 4.2 Determine Overall Status

- **All Green** — everything verified, no issues
- **Minor Issues** — small gaps that can be fixed quickly
- **Significant Gaps** — tasks missing or partially done, needs re-implementation

### 4.2.1 Append Machine-Readable Gate Result

After the human-readable report and overall status, append exactly one final `aif-gate-result` fenced JSON block.

```aif-gate-result
{
  "schema_version": 1,
  "gate": "verify",
  "status": "pass",
  "blocking": false,
  "blockers": [],
  "affected_files": [],
  "suggested_next": {
    "command": "$aif-commit",
    "reason": "Verification passed without blockers."
  }
}
```

Schema reminder: `"status": "pass|warn|fail"`, `"blocking": true|false`, `"blockers": [`, `"affected_files": [`, `"suggested_next": {`.

### 4.3 Action on Issues

If issues were found:

```
AskUserQuestion: Verification found issues. What should we do?

Options:
1. Fix now (recommended) — Use $aif-fix to address all issues
2. Fix critical only — Use $aif-fix for incomplete tasks, skip warnings
3. Fix directly here — Address issues in this session without $aif-fix
4. Accept as-is — Mark everything as done, move on
```

**If "Fix now" or "Fix critical only":**
- First suggest using `$aif-fix` and pass a concise issue summary as argument
- Example:
  - `$aif-fix complete Task #3 password reset email flow, implement Task #8 docs update, remove unfinished markers in src/services/auth.ts and src/middleware/rate-limit.ts, document SENDGRID_API_KEY in .env.example`
- If user agrees, proceed via `$aif-fix`
- If user declines `$aif-fix`, continue with direct implementation in this session
- For each incomplete/partial task — implement the missing pieces (follow the same implementation rules as `$aif-implement`)
- For unfinished markers/debug artifacts — clean them up
- For undocumented config — update `.env.example` and docs
- After fixing, re-run the relevant verification checks to confirm

**If "Accept as-is":**
- Note the accepted issues in the plan entrypoint as a comment
- Continue to Step 5

---

## Step 5: Suggest Follow-Up Skills

After verification is complete, suggest next steps based on result:

- If unresolved issues remain (accepted or deferred), suggest `$aif-fix` first
- If all green, suggest security/review/commit flow

```
## Verification Complete

Suggested next steps:

1. 🛠️ $aif-fix [issue summary] — Fix remaining verification issues
2. 🔒 $aif-security-checklist — Run security audit on the new code
3. 👀 $aif-review — Code review of the implementation
4. 💾 $aif-commit — Commit the changes

Which would you like to run? (or skip all)
```

```
AskUserQuestion: Run additional checks?

Options:
1. Fix issues — Run $aif-fix with verification findings
2. Security check — Run $aif-security-checklist on changed files
3. Code review — Run $aif-review on the implementation
4. Both — Run security check, then code review
5. Skip — Proceed to commit
```

**If fix issues selected** → suggest invoking `$aif-fix <issue summary>`
**If security check selected** → suggest invoking `$aif-security-checklist`
**If code review selected** → suggest invoking `$aif-review`
**If both** → suggest security first, then review
**If skip** → suggest `$aif-commit`

### Context Cleanup

Suggest the user to free up context space if needed: `/clear` (full reset) or `/compact` (compress history).

---

## Strict Mode

When invoked with `--strict`:

```
$aif-verify --strict
```

- **All tasks must be COMPLETE** — no partial or skipped allowed
- **Build must pass** — fail verification if build fails
- **Tests must pass** — fail verification if any test fails (tests are required in strict mode)
- **Lint must pass** — zero warnings, zero errors
- **No unfinished task markers** in changed files
- **No undocumented environment variables**
- **Architecture gate must pass** — fail on clear boundary/dependency violations
- **Rules gate must pass** — fail on clear rule violations
- **Roadmap gate must pass** — fail on clear roadmap mismatch
- **Requirements consistency gate must pass** — fail on clear requirement
  conflicts, unresolved source ambiguity, or missing evidence for material
  supported combinations
- Missing milestone linkage for `feat`/`fix`/`perf` is a warning even in strict mode
- Do not fail strict verification solely because milestone linkage is missing

Strict mode is recommended before merging to the configured base branch or creating a pull request.

---

## Usage

### After implement (suggested automatically)
```
$aif-verify
```

### Strict mode before merge
```
$aif-verify --strict
```

### Standalone (no plan, verify branch diff)
```
$aif-verify
→ No plan found → verify branch diff against the configured base branch
```
