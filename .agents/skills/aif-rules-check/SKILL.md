---
name: aif-rules-check
description: Run a standalone read-only rules compliance gate against changed files or a git ref. Use when you need a dedicated project-rules check without a full review or verify pass.
argument-hint: "[git ref | empty]"
allowed-tools: Read Glob Grep Bash(git *) AskUserQuestion
disable-model-invocation: false
metadata:
  author: AI Factory
  version: "1.0"
  category: quality
---

<!-- aif-ext:aifhub-extension:aif-rules-check:prepend:start -->
## AIFHub Rules Check OpenSpec-native Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-rules-check` body. When this guidance conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Use the upstream `/aif-rules-check` read-only gate as the command owner, while adding AIFHub OpenSpec-native generated rules and artifact ownership rules.

### Mode Detection

Before resolving rule sources, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- If the explicit scope is under `openspec/changes/<change-id>/`, use **OpenSpec-native mode**.
- Otherwise use **Legacy AI Factory-only mode** and follow the upstream rules hierarchy.

### OpenSpec-native mode

When OpenSpec-native mode is active, `/aif-rules-check` is read-only and checks changed files against canonical OpenSpec context plus generated rules.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Read canonical OpenSpec artifacts only as context:

- `openspec/specs/**`
- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`

Load rules in this priority order:

1. `.ai-factory/rules/generated/openspec-merged-<change-id>.md`
2. `.ai-factory/rules/generated/openspec-change-<change-id>.md`
3. `.ai-factory/rules/generated/openspec-base.md`
4. The resolved `paths.rules_file`, default `.ai-factory/RULES.md`
5. The resolved `rules.base`, default `.ai-factory/rules/base.md`
6. Relevant named `rules.<area>` files from config, only when they clearly match the checked scope

OpenSpec-native mode does not require plan-local `rules.md`. Ignore plan-local `rules.md` unless the run is explicitly in Legacy AI Factory-only mode.

Load generated trace metadata when present:

- `.ai-factory/rules/generated/openspec-rules-trace-<change-id>.json`
- `.ai-factory/rules/generated/index.json`

Use trace metadata only as provenance for generated rules. Generated markdown remains the readable rule guidance; canonical OpenSpec artifacts remain the source of truth.

Generated-rule `FAIL` findings require trace-backed source evidence:

- cite `source.path`
- cite `source.requirement`
- keep the cited source aligned with `.ai-factory/rules/generated/openspec-rules-trace-<change-id>.json`

If generated rules or generated trace metadata are missing, stale, or invalid, return `WARN`, report which generated rules and trace files are present, missing, stale, or invalid, and ask the caller to regenerate rules through the compiler-owning workflow: `/aif-mode sync --change <change-id>`, then rerun `/aif-rules-check`. This gate must not regenerate or edit generated rules.

When generated trace metadata is missing or invalid, possible generated-rule findings are capped at `WARN`. Do not return final `status: "fail"` solely for a generated-rule finding unless that finding includes trace-backed `source.path` and `source.requirement`.

Runtime state and QA evidence are external context only:

- Name `.ai-factory/state/<change-id>/` as the runtime state path when useful.
- Name `.ai-factory/qa/<change-id>/` as the QA evidence path when useful.
- When strict done policy needs durable rules evidence, name `.ai-factory/qa/<change-id>/rules.md` as the expected storage location for the final rules `aif-gate-result`, but keep this gate read-only.
- Do not write runtime state, QA evidence, generated rules, rule artifacts, source files, or canonical OpenSpec artifacts.

The final response must still follow the upstream rules-check output contract and end with exactly one final machine-readable `aif-gate-result` fenced JSON block. Use `"gate": "rules"` and lowercase JSON `status`: `pass`, `warn`, or `fail`.

The final `aif-gate-result` block keeps `suggested_next` `null` when `status` is `pass`: the next-step routing below is prose guidance and MUST NOT be encoded into the machine-readable `suggested_next` field.

Next-step routing is one-way with terminal states and wins over any upstream next-step suggestion that would loop this gate with `/aif-verify`:

- When this gate passes and `/aif-verify <change-id>` has not already passed for the current change state, the suggested next step is `/aif-verify <change-id>`.
- When this gate passes and `/aif-verify <change-id>` has already passed for the current change state, the suggested next step is `/aif-done <change-id>`; do not suggest another `/aif-verify` run.
- When this gate fails or generated rules are missing, stale, or invalid, follow the regeneration route above (`/aif-mode sync --change <change-id>`, then rerun `/aif-rules-check`); do not suggest `/aif-verify` as remediation for a failing rules gate.
- Do not suggest rerunning this gate after it has already passed for the current change state.

Determine whether `/aif-verify <change-id>` has already passed by reading the final `aif-gate-result` block in `.ai-factory/qa/<change-id>/verify.md`: treat verification as already passed only when that receipt records `"status": "pass"` and has not been invalidated by later canonical artifact changes for the current change state; with no such current passing receipt, verification has not already passed.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not active, do not add OpenSpec generated-rule requirements. Follow the upstream `/aif-rules-check` behavior for `.ai-factory/RULES.md`, `rules.base`, named `rules.<area>`, optional plan context, changed files, and the final `aif-gate-result` block.
<!-- aif-ext:aifhub-extension:aif-rules-check:prepend:end -->

# Rules Compliance Gate

Run a standalone read-only rules gate for project rules. This command checks rule compliance only; it does not replace `$aif-review` or `$aif-verify`.

## Step 0: Load Contract

- Read `references/RULES-CHECK-CONTRACT.md` first.
- Treat it as the canonical source for verdict semantics and report structure.
- If examples in this file drift from the reference, follow the reference.

## Step 1: Load Config

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- `paths.rules_file`
- `paths.rules`
- `paths.plan`
- `paths.plans`
- `language.ui`
- `git.enabled`
- `git.base_branch`
- `rules.base`
- named `rules.<area>` entries
- `workflow.plan_id_format` (default: `slug`) — used by the optional branch-based plan-context lookup in Step 2.3.
  Active values: `slug` and `sequential`. Plan context may be a root full-plan
  file or direct child ultra `index.md`; numbered lookup covers both shapes.
  `timestamp` and `uuid` are **reserved values** and currently behave like `slug`.
  Treat any unknown value as `slug`.

If config is missing or partial, use defaults:
- `paths.rules_file`: `.ai-factory/RULES.md`
- `paths.rules`: `.ai-factory/rules/`
- `paths.plan`: `.ai-factory/PLAN.md`
- `paths.plans`: `.ai-factory/plans/`
- `git.enabled`: `true`
- `git.base_branch`: detect the repo default branch from git metadata; fall back to `main` only when detection is unavailable
- `rules.base`: `.ai-factory/rules/base.md`
- `workflow.plan_id_format`: `slug`

If `paths.rules_file` is missing from config, default to `.ai-factory/RULES.md` instead of treating config as incomplete.
If `git.base_branch` is missing from config, resolve the repository default branch from git metadata when possible; use `main` only as the final fallback.

### Step 1.1: Load Skill Context

**Read `.ai-factory/skill-context/aif-rules-check/SKILL.md`** - MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as project-level overrides for this skill's general instructions.
- When a skill-context rule conflicts with a general rule in this file, the skill-context rule wins.
- When there is no conflict, apply both.
- Skill-context rules apply to all outputs of this skill, including verdict wording and report structure.

**Enforcement:** Before presenting the final report, verify it against all skill-context rules and fix any drift.

## Step 2: Resolve Inputs

Resolve two inputs before checking any rule:

1. **Changed scope** - the diff and file list you are evaluating
2. **Resolved rule sources** - the rule artifacts that may apply to that scope

### Step 2.1: Resolve Changed Scope

**If the user provided a git ref:**

1. Validate it first:
   ```bash
   git rev-parse --verify <argument>
   ```
2. If valid, use:
   ```bash
   git diff --name-only <argument>...HEAD
   git diff <argument>...HEAD
   ```
3. If invalid, ask:

   ```
   AskUserQuestion: `<argument>` is not a valid git ref. What should I check instead?

   Options:
   1. Check staged / working-tree changes
   2. Cancel
   ```

**Without arguments:**

1. Prefer staged work:
   ```bash
   git diff --cached --name-only
   git diff --cached
   ```
2. If nothing is staged, fall back to working tree:
   ```bash
   git diff --name-only
   git diff
   ```
3. If there is still no local diff and `git.enabled = true`, fall back to branch diff:
   ```bash
   git diff --name-only <resolved-base-branch>...HEAD
   git diff <resolved-base-branch>...HEAD
   ```

If there are still no changed files, return `WARN` rather than a hard failure.

### Step 2.2: Resolve Rule Sources

Load rule sources in this order:

1. The resolved `paths.rules_file` artifact
2. The resolved `rules.base` file
3. Any named `rules.<area>` files from config that clearly match the changed scope

Area rules are optional and scoped:
- Use changed file paths, folder names, and optional plan context to judge relevance.
- If relevance is ambiguous, mention the rule source as uncertain and keep the outcome at `WARN`, not `FAIL`.

If no rules sources resolve, return `WARN` rather than a hard failure.

### Step 2.3: Optional Plan Context

Optional plan context: use the active plan file only when it helps interpret scope or area relevance; absence of a plan is never a failure.

Plan resolution order:
1. Compute the **canonical branch stem** the same way as `$aif-plan`,
   `$aif-implement`, and `$aif-improve`:
   - get current branch via `git branch --show-current` (git mode only);
   - `branch_stem` = current branch with every `/` replaced by `-`
     (for example `feature/user-auth` → `feature-user-auth`).
2. Branch-based lookup using `<branch_stem>`:
   - when `workflow.plan_id_format = sequential`, glob both
     `paths.plans/[0-9][0-9][0-9][0-9]_<branch_stem>.md` and
     `paths.plans/[0-9][0-9][0-9][0-9]_<branch_stem>/index.md`; Read every
     directory candidate and retain it only when it contains exactly one
     `<!-- aif:plan-mode:ultra -->`, then pick the highest-numbered valid
     artifact and warn when multiple valid candidates exist; prefer ultra if
     both shapes share the highest prefix;
   - otherwise/fallback check `paths.plans/<branch_stem>/index.md` and
     `paths.plans/<branch_stem>.md`; Read the directory entrypoint first, ignore
     it unless it contains exactly one ultra marker, and warn/prefer ultra if
     both valid shapes exist.
3. A single named artifact in `paths.plans`: count root `*.md` full plans and
   direct child `*/index.md` entrypoints containing
   `<!-- aif:plan-mode:ultra -->`; exclude
   the resolved fast-plan path and never count phase files.
4. The fast plan at `paths.plan`.

For ultra, read `index.md` first and only the linked phase files relevant to the
changed area when extra scope detail is needed. Do not fail the rules check
because a plan artifact is missing or ambiguous.
An automatically discovered directory entrypoint counts only when it contains
exactly one `<!-- aif:plan-mode:ultra -->`; ignore unrelated `*/index.md` files.

## Step 3: Evaluate Rules

Read the changed files from the resolved scope and compare them against the resolved rules.

Classification rules:
- `PASS` when at least one applicable rule was checked and no clear violations were found.
- `WARN` when no applicable rules were resolved, the evidence is ambiguous, or there are no changed files to evaluate.
- `FAIL` when an explicit hard rule is clearly violated by the inspected diff or changed files.

Only return `FAIL` when an explicit hard rule is clearly violated by the inspected diff or changed files.

Evidence rules:
- Tie every blocking violation to specific rule text and at least one concrete file/path or diff hunk.
- If a rule sounds like a preference, is too vague, or cannot be verified confidently from the diff, do not escalate it past `WARN`.
- Missing optional files or partially configured rules hierarchy are `WARN`, not `FAIL`.

## Step 4: Read-Only Boundary

This command is read-only: do not edit `RULES.md`, `rules/base.md`, `rules.<area>`, plan files, or source code.

If rules are missing, stale, or need refinement:
- Suggest `$aif-rules <rule text>` for axioms
- Suggest `$aif-rules area:<name>` for area-specific rules

## Step 5: Output

Use the exact verdict semantics and section order from `references/RULES-CHECK-CONTRACT.md`.

Required content:
- overall verdict
- files checked
- gate results
- blocking violations
- suggested fixes
- suggested rule updates
- final machine-readable `aif-gate-result` fenced JSON block

When useful, suggest the next best workflow:
- `$aif-review` for broader code review
- `$aif-verify` for full plan-completeness verification
- `$aif-rules` when the underlying rules need to be captured or corrected

Machine-readable gate result:
- Append one final fenced `aif-gate-result` JSON block after the human-readable rules report.
- Use `"gate": "rules"`.
- Map the human rules verdict exactly: `PASS` -> `pass`, `WARN` -> `warn`, and `FAIL` -> `fail`.
- Use `"blocking": true|false`; set it to `true` only for explicit hard-rule violations that produce a human `FAIL`.
- Include only hard-rule violations in `"blockers": [`.
- Include changed or inspected paths in `"affected_files": [`.
- Set `"suggested_next": {` to `$aif-rules` when rules should be added or clarified, `$aif-fix` when code must change, or `null` when no allowed next command fits.
- Do not use `$aif-review` in the JSON `suggested_next.command`; it may appear only in human-readable workflow suggestions.

```aif-gate-result
{
  "schema_version": 1,
  "gate": "rules",
  "status": "warn",
  "blocking": false,
  "blockers": [],
  "affected_files": [],
  "suggested_next": {
    "command": "$aif-rules",
    "reason": "Rules are missing or ambiguous for the changed scope."
  }
}
```

Schema reminder: `"status": "pass|warn|fail"`, `"blocking": true|false`, `"blockers": [`, `"affected_files": [`, `"suggested_next": {`.
