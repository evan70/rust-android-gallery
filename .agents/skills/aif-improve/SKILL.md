---
name: aif-improve
description: Refine an existing implementation plan with a second iteration. Re-analyzes the codebase for gaps, missing tasks, and wrong dependencies. Use after $aif-plan or to improve an $aif-fix plan. Optional +check flag validates refinements via a fresh-context subagent.
argument-hint: "[--list] [+check] [@plan-file-or-directory] [improvement prompt or empty for auto-review]"
allowed-tools: Read Write Edit Glob Grep Bash(git *) Bash(shasum -a 256 *) Bash(sha256sum *) Task Agent TaskCreate TaskUpdate TaskList TaskGet AskUserQuestion Questions
disable-model-invocation: false
---

<!-- aif-ext:aifhub-extension:aif-improve:prepend:start -->
## AIFHub Improve OpenSpec-native Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-improve` body. When any rule below conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Use the built-in `/aif-improve` skill as the canonical refinement command for both OpenSpec-native changes and the extension's legacy companion plan workflow.

### Skill-Context Resolution

Read skill-context in this order:

1. `.ai-factory/skill-context/aif-improve/SKILL.md`
2. `.ai-factory/skill-context/aif-improve-plus/SKILL.md` as legacy compatibility fallback

If both exist, `aif-improve` wins.

### Mode Detection

Before resolving a target, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with Legacy AI Factory-only mode and state that no OpenSpec-native protocol was detected.

### Legacy ultra marker-first boundary

In Legacy AI Factory-only mode, classify the normalized project-relative plan entrypoint with `classifyLegacyPlanShape()` from `scripts/legacy-plan-migration.mjs` before folder-only detection, companion discovery, plan-file creation, refinement, or any `status.yaml` write. Marker validation is first; known companion filenames are considered only after the classifier returns a classic shape.

- For `ultra-valid`, stop all AIFHub companion logic and return the exact upstream handoff `/aif-improve <entrypoint>`. Do not create or synchronize a sibling `<plan-id>.md`, `task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, or `explore.md`; upstream owns the complete marked bundle atomically.
- For `ultra-invalid` or `collision`, fail closed before any write and report only bounded `shape`, safe `entrypoint`, and classifier `code` values.
- For `classic-pair` or `classic-folder-only`, continue with the classic companion rules below. An unrelated directory is not a plan.
- Diagnostics may include only `shape`, safe project-relative `entrypoint`, and `handoff`; never include marker bodies, phase contents, request/research bodies, credentials, raw stdout, or raw stderr.

### Research and interface choices

After target classification permits local refinement, read [research delegation and design alternatives](../../skills/shared/RESEARCH-DESIGN.md) when an affected design needs independent fact-finding or comparison of materially different interfaces. Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/RESEARCH-DESIGN.md`. Apply it to OpenSpec-native and classic legacy refinement, preserving settled decisions and unrelated plan content. The coordinating parent owns support dispatch and acceptance; the authorized refinement writer retains its assigned artifact scope and reconciles evidence before edits. A delegated polisher returns missing questions to its parent without nested workers. Small refinements remain direct. Valid legacy ultra handoffs and invalid/collision stops above take precedence.

### Executable task design

After mode and target checks permit local planning/refinement, follow [executable task design](../../skills/shared/TASK-DESIGN.md) for new task breakdowns, wide migrations or unclear prerequisites. Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/TASK-DESIGN.md`. Preserve current task identities and the existing bounded inline-verification migration; this guidance does not authorize restructuring an unrelated plan, tracker writes or an implementation runtime.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-improve` refines an existing OpenSpec-native change.

For plan content, read only the canonical OpenSpec artifacts listed below. Do not inspect or mutate `.ai-factory/plans/**` after an OpenSpec change resolves.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Resolve the active change using the shared vocabulary from `scripts/active-change-resolver.mjs`:

- Prefer an explicit `<change-id>` or `@openspec/changes/<change-id>` input when provided.
- Otherwise use `resolveActiveChange` behavior: current working directory, current branch mapping, current pointer, then single active change.
- Treat the selected change ID, selected source, candidate list, warnings, and errors as user-visible refinement context.
- If the resolved path is under `openspec/changes/archive/**`, do not edit silently. Archived changes are immutable by default; report the archived target clearly and suggest creating a new change for further work.
- If an explicit or inferred `<change-id>` cannot be resolved as an OpenSpec change, check for matching legacy AI Factory plan artifacts through `detectMigrationNeed(options)` from `scripts/legacy-plan-migration.mjs` or equivalent read-only detection. If migration is suggested, do not auto-migrate. Show exactly:

```text
Found legacy AI Factory plan artifacts for `<change-id>` but no OpenSpec change at `openspec/changes/<change-id>`.
Run the legacy migration script with:

ai-factory aifhub-migrate-legacy-plans <change-id> --dry-run
ai-factory aifhub-migrate-legacy-plans <change-id>
```

Refine only these canonical OpenSpec artifacts for the active change:

- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`

Legacy companion plan artifacts, including `task.md`, `context.md`, `rules.md`, `verify.md`, and `status.yaml` are not OpenSpec-native refinement targets.

Before selecting or changing a capability, inventory accepted specs under `openspec/specs/**`, separately from active changes. Reuse each existing capability's exact path, including nested directories. Read relevant accepted specs in full, including scenarios, before deciding what is covered. If using the optional CLI, `openspec list --specs` inventories specs; plain `openspec list` inventories changes and cannot substitute for it. Filesystem discovery remains sufficient when the CLI is unavailable; this does not add Store selection or initialization authority.

Before refining implementation-dependent artifacts, refresh the relevant implementation, nearby tests, configuration, and docs outside `openspec/` in a read-only pass proportional to the proposed changes. Resolve the actual target repository if it differs from `planningHome.root`; ask only when context cannot identify it. For greenfield work, inspect existing structure and setup docs. Separate observed facts, assumptions, and proposals; cite direct file evidence, disclose unavailable source, and surface conflicts between accepted specs and code before a dependent design decision. Do not defer this discovery into generic future codebase-exploration tasks.

#### Task Quality Refinement

When refining an OpenSpec-native change, audit normalized task quality across the canonical artifacts:

- `proposal.md` for intent, scope, non-goals, assumptions, risks, and open questions
- `design.md` for C4 impact, ADR candidates, dependency notes, integration points, alternatives, and risks
- `tasks.md` for an executable checklist whose task descriptions state how completion is verified
- `specs/**/spec.md` for behavior deltas

For a new capability, refine its delta's `## Purpose` into a meaningful description before first sync/archive. An existing main spec's Purpose is not replaced by a delta's Purpose: if strict validation reports a placeholder, record the affected accepted-spec path and bounded direct remediation instead of manufacturing a requirement delta. Do not rewrite archived changes to clear historical placeholders.

Classify open questions as `blocker`, `warn`, or `info` when useful, without requiring classification in trivial changes or forcing a specific table format.

Also flag as task-quality findings: missing or incorrect requirement citations, unresolved source conflicts, a missing `## Requirements Reconciliation` section when the reconciliation signals apply, and missing verification of a supported behavior combination or a representative real repository artifact. Respect the plan's testing setting: add a test only when testing is enabled, otherwise add a runnable/manual verification step.

Patch only affected sections and avoid whole-file regeneration unless structurally unusable.

Preservation rules:

- Read current artifact content before editing.
- Treat the complete `## Original Request` heading and body as immutable raw source. Preserve its exact bytes, including line endings, whitespace, punctuation, casing, and line breaks; patch other sections around it instead of reconstructing `proposal.md`.
- Treat an existing `## AIFHub Source Binding` as reserved identity metadata. Preserve its complete heading and body byte-for-byte during ordinary refinement; `Provider`, `Primary source`, and `External ID` are immutable, and `Branch` may change only for an explicit branch-rebind request that keeps the same primary source and passes source-binding validation. In a legacy classic plan, update the Markdown entrypoint and `status.yaml.source_binding.branch` as one logical rebind and require `parseSynchronizedWorkItemSourceBinding(markdown, status)` to pass before reporting success.
- Treat an existing `## Research Context` body and `Source` revision metadata as the committed requirements snapshot. Do not translate, normalize, regenerate, or replace it unless the user explicitly requests a research rebase.
- Treat an existing `## Requirements Reconciliation` as an upstream-owned compatibility record parsed by downstream workflow checks. Keep the fixed heading exact and preserve its declared authority, citations, applicable combination table, and verification evidence, updating them only to stay synchronized with refined tasks from the plan's committed evidence. Research-backed decisions stay frozen until an explicit rebase; never resolve a recorded conflict by editing the section — surface unresolved conflicts to the user or the owning workflow instead of patching around them.
- For an embedded ultra source, pass its exact project-relative `RESEARCH.md` path to `resolveUltraResearchSource()` from `scripts/ultra-research-resolver.mjs`. Consume the structured `source`, `revision`, and `diagnostic` so the sibling marked `INDEX.md`, active status, Artifact Index link, path confinement, and normalized Active Summary digest are all revalidated centrally.
- For an explicit research rebase, use resolver precedence: safe explicit `RESEARCH.md` path, exact slug, then exactly one caller-reviewed relevant active candidate. Ambiguity stops; recency never chooses a source.
- When the exact regular or ultra source is missing/invalid, or its `Updated`/normalized `SHA256` differs, emit `WARN [research-drift] change-id=<change-id> source=<path> expected=<embedded revision> current=<live revision>`. Keep the embedded snapshot authoritative and do not log its full body, credentials, raw provider output, or sibling artifact bodies.
- Live research `## Sessions` may be consulted only for rationale. Do not apply requirements from a newer Active Summary without an explicit user rebase request.
- On an explicit research rebase, copy the selected current Active Summary, recompute the stable digest, update the `Source` path plus `Updated` and `SHA256` metadata, and report that committed scope changed.
- Preserve user-written sections unless they are explicitly obsolete or contradict the refined requirement.
- Prefer patch-style edits over whole-file regeneration.
- In `tasks.md`, require every task checkbox to state its completion verification inline through a test, command, observable behavior, or delivered artifact. Keep a separate verification task only when it checks broader integration or system behavior spanning multiple implementation tasks.
- For a `tasks.md` created before this rule, treat every checkbox missing inline completion verification as affected by a bounded checklist migration, even when its implementation action is otherwise unrelated to the requested refinement.
- For each migrated checkbox, preserve its task number, checked/unchecked state, order, and original action and intent. Append only the concrete verification clause needed for compliance; do not split, merge, reorder, renumber, reopen, complete, or broaden the task.
- Leave already compliant unrelated checkboxes unchanged unless the requested refinement affects them.
- If an artifact is missing, create only missing artifacts needed by the requested refinement.
- When a delta spec exists, update the relevant requirement in an existing delta spec instead of regenerating the whole file.
- Keep unrelated requirements, scenarios, already compliant task checkboxes, and design notes intact. A checkbox missing required inline verification is part of the bounded checklist migration rather than unrelated content.

Legacy checklist migration example:

Before:

```markdown
- [x] 1.1 Add parser support
- [ ] 1.2 Update usage docs
```

After:

```markdown
- [x] 1.1 Add parser support; verify focused parser tests pass
- [ ] 1.2 Update usage docs; verify documentation contract tests pass
```

Validation and runtime state:

- Read base specs from `openspec/specs/**` and generated rules from `.ai-factory/rules/generated/` when they are needed to preserve canonical requirement intent.
- Before validation, resolve the effective OpenSpec policy through `ai-factory aifhub-mode status --json`.
- Run or recommend OpenSpec validation through `validateOpenSpecChange(changeId)` from `scripts/openspec-runner.mjs`, or equivalent shared-runner behavior.
- Validation should correspond to `openspec validate <change-id> --type change --strict --json --no-interactive --no-color`.
- Missing or unsupported OpenSpec CLI is degraded validation unless the effective policy has `requireCliForImprove: true`; when required, treat the missing CLI as a refinement blocker and do not report the change as validation-ready.
- Summarize effective policy, validation success, failure, degraded status, or policy-blocked status in the normal response.
- Runtime state notes may be written only under `.ai-factory/state/<change-id>/`.
- QA evidence belongs under `.ai-factory/qa/<change-id>/` and should not be written into canonical OpenSpec change artifacts.
- Prefer `ensureRuntimeLayout(changeId)` when runtime directories are needed.
- Valid persisted runtime evidence includes `.ai-factory/state/<change-id>/improve-summary.md` and `.ai-factory/state/<change-id>/last-validation.json`.
- Do not write runtime-only files or validation evidence under `openspec/changes/<change-id>/`.

Output summary:

```text
Changed:
- proposal.md: ...
- design.md: ...
- tasks.md: ...
- specs/<capability>/spec.md: ...

Preserved:
- ...
```

The response must report the selected change ID, selected source, changed canonical artifact paths, preserved user-written areas, and validation status. Do not install OpenSpec skills or slash commands.
Report `Original Request`, `AIFHub Source Binding`, `Research Context`, and `Requirements Reconciliation` as preserved section names when applicable, but do not duplicate their raw bodies in output.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve the extension's companion plan behavior:

- Resolve all of these inputs to one active pair:
  - `.ai-factory/plans/<plan-id>.md`
  - `.ai-factory/plans/<plan-id>/`
  - any plan-local artifact path inside `.ai-factory/plans/<plan-id>/`
- When a legacy folder-only plan is selected:
  - create `.ai-factory/plans/<plan-id>.md` before refinement continues
  - preserve the existing folder artifacts
  - record the upgrade in `status.yaml.history`
- Update the plan file summary and the plan-folder artifacts together.
- Keep `status.yaml` as the canonical execution state file.
- When refinement completes successfully and the next step is execution, route to `/aif-implement`.
- Do not send the user to deprecated workflow aliases or legacy `*-plus` command names.

### Compatibility Note

If historical docs or plan notes still mention `aif-improve-plus`, interpret that as `/aif-improve`.

### Codex Runtime

When running in Codex app/CLI:

- The refinement stage (`/aif-improve`) should run in Codex Plan mode when structured clarifying questions are needed.
- This skill may recommend Plan mode, but it does not attempt or promise to switch the Codex session mode. The user controls the mode.
- In Codex Plan mode, use `request_user_input` only for 1-3 short questions.
- In Codex Default mode, if a question is needed, ask it as plain text in the assistant message. Do not use `question(...)`, `questionnaire(...)`, or `request_user_input`.
- If another CLI or IDE runtime exposes a planning mode, use that available planning-mode mechanism for structured planning questions; do not fabricate unavailable tools or client actions.
- In autonomous or subagent mode, do not ask interactive questions. Record assumptions and return blockers/open questions to the parent.
- See `skills/shared/QUESTION-TOOL.md` for the full runtime question format mapping.
<!-- aif-ext:aifhub-extension:aif-improve:prepend:end -->

# Improve - Plan Refinement (Second Iteration)

Refine an existing plan by re-analyzing it against the codebase. Finds gaps, missing tasks, wrong dependencies, and enhances task quality.

## Core Idea

```
existing plan + deeper codebase analysis + user feedback (optional)
    ↓
find gaps, missing edge cases, wrong assumptions
    ↓
enhanced plan with better tasks, correct dependencies, more detail
```

## Workflow

### Step 0: Load Config & Parse Arguments

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.plan`, `paths.plans`, `paths.fix_plan`, `paths.research`, `paths.description`, `paths.architecture`, `paths.roadmap`, `paths.rules_file`, `paths.rules`, `paths.patches`, and `paths.archive`; derive `research_bundles_dir = <parent directory of paths.research>/research/`
- **Language:** `language.ui` for prompts and summaries, `language.artifacts` for plan artifact updates, and `language.technical_terms` for human-readable technical terminology in plan artifacts
- **Git:** `git.enabled`, `git.base_branch`, `git.create_branches`
- **Workflow:** `workflow.plan_id_format` (default: `slug`) — used by branch-based plan discovery.
  Active values: `slug` and `sequential`. A root `*.md` is a named full plan
  unless it is the resolved fast/fix path; a direct child `*/index.md` is an
  ultra bundle entrypoint only when it contains exactly one
  `<!-- aif:plan-mode:ultra -->`. When `sequential`,
  search both `[0-9]{4}_<branch-slug>.md` and
  `[0-9]{4}_<branch-slug>/index.md`, then choose the highest prefix.
  `timestamp` and `uuid` are **reserved values** and currently behave like `slug`.
  Treat any unknown value as `slug`.

If config.yaml doesn't exist, use defaults:
- plan: `paths.plan` (default: `.ai-factory/PLAN.md`)
- plans/: `.ai-factory/plans/`
- fix plan: `paths.fix_plan` (default: `.ai-factory/FIX_PLAN.md`)
- research: `.ai-factory/RESEARCH.md`
- patches/: `.ai-factory/patches/`
- DESCRIPTION.md: `.ai-factory/DESCRIPTION.md`
- architecture: `.ai-factory/ARCHITECTURE.md`
- roadmap: `.ai-factory/ROADMAP.md`
- rules file: `.ai-factory/RULES.md`
- rules directory: `.ai-factory/rules/`
- `ui_language`: `en`
- `artifact_language`: `en`
- `technical_terms_policy`: `keep`
- `workflow.plan_id_format`: `slug`

Resolved language values:
- `ui_language = language.ui || "en"`
- `artifact_language = language.artifacts || language.ui || "en"`
- `technical_terms_policy = language.technical_terms || "keep"`

If `technical_terms_policy` is not one of `keep`, `translate`, or `mixed`, treat it as `keep`. Legacy values such as `english` also behave like `keep`.

All AskUserQuestion prompts, progress updates, refinement reports, summaries, and next-step guidance MUST be written in `ui_language`.

Any generated or updated plan artifact content under `paths.plan`, `paths.plans`, or `paths.fix_plan` MUST be written in `artifact_language`.

Exception: an existing `## Original Request` section is raw source input, not generated artifact prose. Preserve its heading and body exactly as read from the plan file on every edit or regeneration, even when `artifact_language` differs. Do not translate, summarize, normalize, trim, or rewrite it.

Templates and examples define structure, not fixed English output. If `artifact_language` is not `en`, translate human-readable headings, labels, task prose, roadmap rationale, research summaries, improvement notes, and dependency notes before saving. Preserve markdown structure, checkbox syntax, task IDs, numeric prefixes, branch names, commit messages, commands, file paths, config keys, package names, API names, `WARN`/`INFO` labels, and raw errors unchanged. Keep `## Research Context`, `Source:`, `Active Summary`, `Updated:`, and `SHA256:` exact because downstream research drift checks parse them as compatibility tokens. Keep `## Requirements Reconciliation` exact because downstream workflow checks parse it as a compatibility token. Apply `technical_terms_policy` to other human-readable terminology.

**First parse arguments:**

```
- --list    → list available plans only (read-only, then STOP)
- +check    → after refinement, validate findings via a fresh-context subagent
- @<path>   → explicit plan file, ultra directory, or ultra `index.md` override (highest priority)
- remaining argument text → optional improvement prompt
```

`+check` is orthogonal to the other flags and may appear anywhere in `$ARGUMENTS`. Strip it from the argument string before resolving `@<path>` and the improvement prompt.

When `--list` is present, it wins and no refinement is executed. `+check` is silently ignored in `--list` mode (there is nothing to validate before refinement runs).

### Step 0.list: List Available Plans (`--list`)

If `$ARGUMENTS` contains `--list`, execute the procedure in `references/LIST-MODE.md` and STOP. That document is the single source of truth for the discovery rules, output shape, and read-only contract (no refinement, no file modifications, `+check` is silently ignored). Do not duplicate its content here.

### Step 1: Resolve Active Plan

This step runs in the default (non-`--list`) mode and picks **one** plan artifact for refinement using the priority chain below. The discovery-list logic for `--list` lives in `references/LIST-MODE.md` and is independent of this step.

**Locate the active plan artifact using this priority:**

```
1. If `$ARGUMENTS` contains `@<path>`:
   - Resolve the path (relative to project root; absolute paths allowed)
   - Accept a markdown file, an ultra directory containing `index.md`, or that `index.md`
   - If missing → show "Plan artifact not found: <path>" and STOP
   - Before normalizing a directory or treating an explicit `index.md` as ultra,
     Read `index.md` and require exactly one
     `<!-- aif:plan-mode:ultra -->`; otherwise STOP with a plan-integrity error
2. No explicit `@<path>` override → Check current git branch:
   git branch --show-current
   → Convert branch name to filename: replace "/" with "-" (this is <branch-slug>)
   → When `workflow.plan_id_format = sequential`, glob both:
     `<configured plans dir>/[0-9][0-9][0-9][0-9]_<branch-slug>.md`
     `<configured plans dir>/[0-9][0-9][0-9][0-9]_<branch-slug>/index.md`
     Read every directory candidate and retain it only when `index.md` contains
     exactly one `<!-- aif:plan-mode:ultra -->`. Choose the highest-numbered
     valid artifact and emit `WARN [aif-improve]` when multiple valid candidates
     exist; if both shapes share the highest prefix, prefer ultra.
   → If no valid sequential match exists, or sequential mode is inactive, check
     `<configured plans dir>/<branch-slug>/index.md` and
     `<configured plans dir>/<branch-slug>.md`. Read the directory entrypoint
     before selection and ignore it unless it contains exactly one ultra marker;
     if both valid shapes exist, warn and prefer ultra.
   Example (slug):       feature/user-auth → .ai-factory/plans/feature-user-auth.md
   Example (sequential): feature/user-auth → .ai-factory/plans/0042_feature-user-auth.md
3. If the branch-based plan is missing or git mode is off:
   → Count root `*.md` full plans plus declared-ultra direct child `*/index.md`
     entrypoints; exclude resolved `paths.plan` and `paths.fix_plan`
   → If exactly one artifact exists, use it
   → Do not count ultra phase files as plans
   → If multiple exist, ask the user to choose or require `@<path>`
4. No named full/ultra plan → Check the resolved fast plan path (from $aif-plan fast)
5. No regular plan and no resolved fast plan → Check the resolved fix plan path (from $aif-fix plan mode)
```

**Note:** Plan discovery scans `paths.plans/` only. Plans archived to `paths.archive/plans/` by `$aif-archive` are excluded from discovery.
Any automatically discovered `*/index.md` candidate must be read before
selection and ignored unless it contains exactly one
`<!-- aif:plan-mode:ultra -->`. A marked ultra bundle with broken links is a
blocking integrity error, not a fallback opportunity.

**If NO plan file found at any location:**

```
No active plan found.

To create a plan first, use:
- $aif-plan full <description>  — for a rich single-file feature plan
- $aif-plan ultra <description> — for an exhaustive multi-file plan bundle
- $aif-plan fast <description>  — for a quick task plan
- $aif-fix <bug description>    - for a bugfix plan (use the resolved fix plan path)
```

→ **STOP here.** Do not proceed without a plan artifact.

**If a plan artifact is found → proceed to Step 2 (Load Context).**

### Step 2: Load Context

**2.1: Read the plan artifact**

Read the selected entrypoint completely. If it is an ultra `index.md`, validate
its Phase Index links and read every linked phase file in order before analyzing
or regenerating the bundle. Treat the bundle as one atomic plan. Understand:
- Feature scope and goals
- `## Original Request`, when present: the original user intent and immutable scope anchor for the plan. Treat this section as raw source input and preserve it exactly on any plan edit or regeneration.
- Current tasks (subjects, descriptions, dependencies)
- Settings (testing, logging preferences)
- Commit checkpoints
- Which tasks are already completed (checkboxes `- [x]`)
- For ultra: the exact paths/symbols, implementation steps, contracts, risks,
  acceptance criteria, and verification in every task's phase file

**2.2: Read project context**

Read `.ai-factory/DESCRIPTION.md` (use path from config) if it exists:
- Tech stack
- Architecture
- Conventions
- Non-functional requirements

Read the resolved architecture and roadmap artifacts when present. Read the
resolved rules hierarchy in order: `paths.rules_file`, `rules.base`, then named
`rules.<area>` entries relevant to the plan. More-specific rules override
general ones. Treat roadmap text as scope unless the project explicitly declares
it authoritative for detailed behavior.

If the plan contains `## Research Context`, treat its embedded copy as the committed requirements snapshot. Parse the first `Source:` / `Reference:` line with canonical `^(?:Source|Reference):\s+\x60([^\x60]+)\x60\s+\(` syntax; for older bare-path lines, fall back to `^(?:Source|Reference):\s+(.+?)\s+\(`. Fall back to configured `paths.research` only when neither form identifies a usable path.

If the parsed source is inside `research_bundles_dir`, require its sibling `INDEX.md` to contain `<!-- aif:research-mode:ultra -->` exactly once and link that `RESEARCH.md` from `## Artifact Index`; otherwise emit `WARN [research-drift]`. Valid sibling C4/ADR/dependency artifacts are rationale only and must not expand plan scope.

When `SHA256:` is present, extract the current source text strictly between `<!-- aif:active-summary:start -->` and `<!-- aif:active-summary:end -->`, remove HTML comment blocks, preserve line order and leading whitespace, trim trailing spaces from every line, use LF endings, and end with one newline. Hash through stdin with `shasum -a 256` or `sha256sum`; the digest is authoritative. Use `Updated:` only as a legacy fallback when `SHA256:` is absent. A missing/invalid source or revision mismatch emits `WARN [research-drift]`; refine against the embedded Research Context unless the user explicitly requests a rebase.

When adding `## Research Context` to an unlinked plan, use the same normalization algorithm and first output field as the `SHA256:` value. Calculate it through stdin / inline shell input without a temporary repository artifact.

Otherwise, select at most one relevant research source using the same conservative order as `$aif-plan`: an explicit source, one clearly matching direct child bundle whose `INDEX.md` contains `<!-- aif:research-mode:ultra -->` exactly once, declares `Status: active`, and links its `RESEARCH.md` from `## Artifact Index`, then the configured legacy file. Do not merge multiple sources or choose by recency alone.

**2.3: Read patches (limited fallback)**

Use patches as fallback context, not the default source:

- If `.ai-factory/skill-context/aif-improve/SKILL.md` does not exist and the resolved patches dir exists:
  - `Glob: <resolved patches dir>/*.md`
  - Sort patch filenames ascending (lexical), then select the last **10** (or fewer if less exist)
  - Read those selected patch files only
  - Focus on reusable Prevention/Root Cause patterns that affect planning quality
- If skill-context exists, do **not** read all patches by default.
  - Optionally inspect a small targeted subset when refining around a known recurring issue.

**Read `.ai-factory/skill-context/aif-improve/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the Plan
  Refinement Report and any plan modifications. If a skill-context rule says "tasks MUST include X"
  or "plan structure MUST have Y" — you MUST apply these when refining. Generating a refinement
  report that ignores skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

**2.4: Load current task list**

```
TaskList → Get all tasks with statuses
```

Understand what's already been created, what's in progress, what's completed.

### Step 3: Deep Codebase Analysis

Now do a **deeper** codebase exploration than what `$aif-plan` did initially:

**3.1: Trace through existing code paths**

For each task in the plan, find the relevant files:
```
Glob + Grep: Find files mentioned in tasks
Read: Understand current implementation
```

Look for:
- Existing patterns the plan should follow
- Code that already partially implements what a task describes
- Hidden dependencies the plan missed
- Shared utilities or services the plan should use instead of creating new ones

**3.2: Check for integration points**

Look for things the plan might have missed:
- API routes that need updating
- Database migrations needed
- Config files that need changes
- Import/export updates
- Middleware or guards that apply
- Existing validation patterns

**3.3: Check for edge cases**

Based on the tech stack and codebase:
- Error handling patterns used in the project
- Null/undefined safety patterns
- Authentication/authorization checks needed
- Rate limiting, caching considerations
- Data validation at boundaries

**3.4: Reconcile requirements and behavior combinations**

- Read any `## Requirements Reconciliation` section in the plan. Resolve
  research-backed citations against the embedded `## Research Context`; the
  live research source is only a drift signal unless the user explicitly asks
  to rebase. Verify other cited passages against the current authoritative
  context.
- Follow a source-priority hierarchy only when the user or project declares it.
  If conflicting sources have no declared authority, report the ambiguity
  instead of choosing silently.
- Treat roadmap items as scope indicators rather than detailed behavioral
  contracts unless the project explicitly says otherwise.
- When behavior depends on independent selectors, states, modes, or input
  shapes, check the supported combinations across input validation, persistence,
  output/transition, and side effects. Look for combinations that were described
  separately but never verified together.
- When real repository data, configuration, fixtures, schemas, or content define
  the contract, check that the plan exercises a representative existing artifact
  through the primary integration path.

### Step 4: Identify Improvements

Compare the plan against what you found. Use `## Original Request` as the original intent / scope anchor when it exists, together with the current task list and any committed `## Research Context`. A proposed refinement is in scope only when it supports that original request or an explicitly approved user refinement prompt; otherwise route it to 4.6 (`out_of_scope`) instead of adding it to the active plan. Categorize issues:

**4.1: Missing tasks**
- Tasks that should exist but don't (e.g., migration, config update, index creation)
- Tasks for edge cases not covered

**4.2: Task quality issues**
- Descriptions too vague (no file paths, no specific implementation details)
- Missing logging requirements
- Missing error handling details
- Incorrect file paths
- Missing or incorrect requirement citations, unresolved source conflicts, or a
  missing `## Requirements Reconciliation` section when Step 3.4 requires one
- Missing verification of a supported behavior combination or representative
  real artifact. Respect the plan's testing setting: add a test only when testing
  is enabled; otherwise add a runnable/manual verification step.

**4.3: Dependency issues**
- Wrong task order (task A depends on B but B comes after A)
- Missing dependencies (task C needs task A's output but isn't blocked by it)
- Unnecessary dependencies (tasks could run in parallel)

**4.4: Redundant or duplicate tasks**
- Two tasks doing the same thing
- Task that's unnecessary because the code already exists
- Task that duplicates existing functionality

**4.5: Task size issues**
- Tasks too large (should be split)
- Tasks too small (should be merged)
- Split/merge findings go into the "📝 Task Improvements" report section (`improvements` group, alongside 4.2) — they restructure existing tasks rather than add or remove them.

**4.6: Out-of-scope tasks**
- Tasks already in the plan that look useful in themselves but are unrelated to the feature this plan is about (gold-plating)
- On approval these are removed from the active plan — the same drop action as `removals` (see Step 6.4). The difference is the report only: an out-of-scope task goes to its own "💡 Out of scope" section instead of being lumped into "🗑️ Removals", so the user sees a useful-but-unrelated idea before it is dropped and can choose to capture it elsewhere. The skill itself does not persist out-of-scope items anywhere.

**4.7: User-prompted improvements (if $ARGUMENTS provided)**

If the user provided specific improvement instructions in `$ARGUMENTS` (excluding `--list`, `+check`, and `@<path>` tokens):
- Apply the user's feedback to the plan
- Look for tasks that need modification based on the prompt
- Add new tasks if the user's prompt requires them

This is a dispatcher step, not a separate finding category. Each finding it produces is routed to its natural group based on its nature: a new task goes to 4.1 (`missing`), a rewording or expansion of an existing task goes to 4.2 (`improvements`), an explicit removal request goes to 4.4 (`removals`), and a "useful-but-out-of-scope" idea goes to 4.6 (`out_of_scope`). There is no separate 4.7 group in the Step 5 report or in `+check` validation.

### Optional: `+check` validation between Step 4 and Step 5

When the `+check` flag is set (and `--list` is not), run the validation procedure from `references/CHECK-MODE.md` here, between Step 4 and Step 5. It re-reads cited files via a fresh-context subagent, then drops invented items, rewrites partially-correct ones, and recomputes dependencies on the filtered list. For ultra, the validator must receive the entrypoint plus all linked phase files as the plan artifact; validating `index.md` alone is incomplete. Without `+check`, skip this entirely — the output has no validator-related lines and the Summary block stays in its default shape without the two `+check` counter rows.

### Step 5: Present Improvements

Show the user what you found in a clear format. The emoji-grouped sections are kept for scannability, but the items inside "🆕 Missing Tasks", "📝 Task Improvements", "🗑️ Removals", and "💡 Out of scope" all follow the same prose shape — no labeled `Why:` / `Issue:` / `Fix:` fields:

1. **Behavioral impact** — what breaks or becomes harder if the plan stays as-is (missing capability, vague task that will be misimplemented, redundant task that wastes effort).
2. **Optional note** — short citation from the codebase, an existing pattern the plan should match, or a consequence. Include only when it adds signal.
3. **Plan anchor** — `Task #X` reference (or "after Task #X" for new tasks).
4. **Suggested edit** — concrete change: what to add / how to reword / what to remove.

The "🔗 Dependency Fixes" group is **not** restated in this shape — it is always computed after the four other groups (and after `+check` filtering when the flag is set, see `references/CHECK-MODE.md`) and uses the short legacy form: `Task #X should depend on Task #Y. Reason: …`. The dependency entries reference only tasks that survived filtering.

The Step 5 report template below defines structure only. Render all human-readable text in this user-facing response in `ui_language`. Preserve command names, paths, task IDs, section structure, option structure, task counts, numeric counts, `WARN`/`INFO` labels, and raw errors unchanged.

```
## Plan Refinement Report

Plan: [plan artifact path]
Tasks analyzed: N

### Findings

#### 🆕 Missing Tasks (N found)
1. The plan currently leaves authenticated requests without a session refresh step — long-running clients silently lose access after the access-token TTL. The existing middleware in `src/middleware/auth.ts` already exposes a `refresh()` hook, so the plan should reuse it instead of inventing a new one. After Task #3. Add a new task: "Wire `authMiddleware.refresh()` into the login flow and cover the expired-token path with an explicit test."

#### 📝 Task Improvements (N found)
1. Task #4 ("Add validation") gives no field-by-field contract — implementer will either over-validate or skip the email format check that the rest of the codebase enforces via `validators/email.ts`. Task #4. Rewrite as: "Validate `email` (via `validators/email.ts`), `password` (min 12 chars), and `displayName` (1-64 chars) in `RegisterRequest`; return 422 with field-level errors when validation fails."

#### 🔗 Dependency Fixes (N found)
1. Task #5 should depend on Task #2. Reason: Task #5 consumes the session helper introduced in Task #2.

#### 🗑️ Removals (N found)
1. Task #7 ("Create UserRepository") duplicates `src/repos/user.ts:12` which already exposes the same query surface — keeping the task will lead to a parallel implementation. Task #7. Remove the task; rely on the existing repository and adjust Task #8 to import it.

#### 💡 Out of scope — for later (N found)
1. Task #11 ("Refactor the logging module") looks reasonable on its own but is unrelated to the login feature this plan is about — keeping it expands scope without any concrete trigger from the current code paths. Task #11. Drop it from the active plan; the idea is surfaced here so you can capture it elsewhere (issue tracker, backlog note) if it's worth revisiting as its own feature later.

#### 📋 Summary
- Missing tasks: N
- Tasks to improve: N
- Dependencies to fix: N
- Tasks to remove: N
- Out of scope: N

When `+check` ran successfully, two extra rows (`Hidden by +check: N`, `Adjusted by +check: M`) are appended to the Summary block — the exact wording and failure-mode replacements live in `references/CHECK-MODE.md`.

AskUserQuestion: Apply these improvements?

Options:
1. Yes, apply all
2. Let me pick which ones
3. No, keep the plan as is
```

**Based on choice:**
- Yes, apply all → apply all improvements to the plan file
- Let me pick which ones → present each improvement individually for approval
- No, keep the plan as is → exit without modifications

**If no improvements found:**

The completion templates below define structure only. Render all human-readable text in these user-facing responses in `ui_language`. Preserve command names, paths, task counts, and numeric counts unchanged.

```
## Plan Review Complete

The plan looks solid! No significant gaps or issues found.

Plan: [plan artifact path]
Tasks: N

Ready to implement:
$aif-implement
```

### Step 6: Apply Approved Improvements

Based on user's choice:

**6.1: Apply task improvements**

For existing tasks that need better descriptions:
```
TaskGet(taskId) → read current
TaskUpdate(taskId, description: "improved description", subject: "improved subject")
```

**6.2: Add missing tasks**

For new tasks:
```
TaskCreate(subject, description, activeForm)
TaskUpdate(taskId, addBlockedBy: [...]) → set dependencies
```

**6.3: Fix dependencies**

```
TaskUpdate(taskId, addBlockedBy: [...])
```

**6.4: Remove redundant or out-of-scope tasks**

Both `removals` and `out_of_scope` translate to the same plan-file action — drop the task:

```
TaskUpdate(taskId, status: "deleted")
```

The difference between the two is the report only. `removals` are dead-weight duplicates: mentioned once and forgotten. `out_of_scope` items appear in the "💡 Out of scope" section so the user sees the idea was noticed and consciously dropped from this plan, not removed without a trace. The skill does not persist out-of-scope tasks anywhere — capturing the idea elsewhere (issue tracker, backlog) is the user's call.

**6.5: Update the plan artifact**

**CRITICAL:** After all changes, update the selected plan artifact to reflect the new state:

- Add new tasks to the correct phase with `- [ ]` checkboxes
- Update task descriptions if they changed
- Fix task ordering if dependencies changed
- Remove deleted tasks
- Update commit checkpoints if task count changed significantly
- Preserve any `- [x]` checkboxes for already completed tasks
- Preserve existing `## Original Request` exactly, including heading, body text, whitespace, and line breaks. Do not translate, summarize, normalize, trim, or rewrite it; this section is raw source input and is exempt from `artifact_language` rewriting.
- Preserve existing `## Research Context` and its `Source:` / revision marker exactly on any rewrite, unless the user explicitly asks to rebase the plan to current research
- Preserve or update `## Requirements Reconciliation` from the plan's committed
  evidence; keep the exact heading, declared authority, citations, applicable
  combination table, and verification scenarios synchronized with the refined
  tasks. When linked research has drifted, preserve its research-backed
  decisions until the user explicitly requests a rebase; only that rebase may
  update them from the live research source
- If an unlinked plan is refined using current research, add `## Research Context` by copying the relevant Active Summary and write ``Source: `<selected research path>` (Active Summary, Updated: <research Updated timestamp>, SHA256: <sha256 of copied Active Summary>)``
- Compute that hash with the canonical drift normalization: remove HTML comment blocks, preserve line order and leading whitespace, trim trailing spaces from every line, use LF line endings, and end with exactly one final newline. Feed the normalized text to `shasum -a 256` or `sha256sum` through stdin / inline shell input, never through a temp file, and copy the first output field.
- If a linked plan has research drift, keep the committed Research Context and source revision in the plan and include `WARN [research-drift]` in the refinement report
- For ultra, update `index.md` and the affected phase files atomically:
  - task checkboxes remain only in `index.md`
  - every task ID maps to exactly one phase section
  - add/remove/rename phase files only together with Phase Index and task-link updates
  - preserve completed task status while improving its detail
  - rerun link, orphan, dependency, and task-mapping integrity checks
  - maintain the ultra detail gate: exact paths/symbols, ordered edits, contracts,
    error/logging behavior, test policy, acceptance criteria, and verification

Use `Edit` to make surgical changes to a file, or `Write` to regenerate affected
files when changes are extensive. Never regenerate only `index.md` when phase
details have changed.

When editing or regenerating the plan artifact, keep all human-readable artifact content in `artifact_language`; the examples above are structural only. Preserve completed `- [x]` checkboxes exactly. The `## Original Request` section is the explicit language-policy exception: keep it verbatim because it is the user's raw request, not generated artifact prose.

**Identifier invariant:** when the existing full-plan filename or ultra
directory name matches `^[0-9]{4}_.*(\.md)?$`, preserve the
exact numeric prefix on rewrite. Never renumber a plan during an improve pass —
the prefix is permanent and must survive any regeneration. Write back to the
same file or bundle directory you read from.

**6.6: Confirm completion**

The Step 6.6 completion template below defines structure only. Render all human-readable text in this user-facing response in `ui_language`. Preserve command names, paths, task counts, and numeric counts unchanged.

```
## Plan Refined

Changes applied:
- Added N new tasks
- Improved N task descriptions
- Fixed N dependencies
- Removed N redundant tasks

Updated plan: [plan artifact path]
Total tasks: N

Ready to implement:
$aif-implement
```

### Context Cleanup

Suggest the user to free up context space if needed: `/clear` (full reset) or `/compact` (compress history).

## Artifact Ownership

- Primary ownership: the plan artifact being refined (resolved branch-plan path, named full-plan path, resolved fast plan path, or resolved fix plan path when explicitly targeted).
- Config use: resolve full-plan directory via `paths.plans`, fast/fix plans via `paths.plan` and `paths.fix_plan`, git behavior via `git.enabled` and `git.create_branches`, optional legacy/bundled research context from `paths.research` and its derived sibling `research/` directory, and patch fallback via `paths.patches`.
- Read-only context: description, architecture, roadmap, rules, and research artifacts except where the active plan file itself is being updated.

## Important Rules

1. **Don't rewrite from scratch** — improve the existing plan, don't replace it
2. **Preserve completed work** — never modify or remove `- [x]` completed tasks
3. **Traceable improvements** — every change must be justified by codebase analysis or user input
4. **Respect settings** — if testing is "no", don't add test tasks. If logging is "minimal", don't add verbose logging tasks
5. **No gold-plating** — don't propose adding tasks outside the feature scope unless critical. When you find a task already in the plan that drifts outside scope, route it to the "💡 Out of scope" report section, not to "🗑️ Removals" — the user should see useful-but-not-here ideas separately from dead-weight duplicates.
6. **Minimal viable improvements** — suggest only what matters, not every possible enhancement
7. **User approves first** — never apply changes without user confirmation
8. **Keep plan file in sync** — the plan file MUST match the task list after improvements
9. **Original Request is immutable** — when present, use `## Original Request` to judge scope and preserve it verbatim on every plan edit or regeneration
10. **Requirements remain external evidence** — never rewrite requirement,
    research, roadmap, rules, or architecture artifacts to make the plan look
    consistent; refine the plan or report the conflict.

## Examples

Worked examples for the default, prompt-driven, no-plan, explicit-plan-file, and "plan looks solid" flows live in `references/EXAMPLES.md`. The `--list` mode example lives in `references/LIST-MODE.md`; the `+check` mode example lives in `references/CHECK-MODE.md`.
