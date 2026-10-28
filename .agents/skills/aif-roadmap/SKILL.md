---
name: aif-roadmap
description: Create or update a project roadmap with major milestones. Generates the configured roadmap artifact (default .ai-factory/ROADMAP.md) — a strategic checklist of high-level goals. Use when user says "roadmap", "project plan", "milestones", or "what to build next".
argument-hint: "[check | project vision or requirements]"
allowed-tools: Read Write Edit Glob Grep Bash(git *) AskUserQuestion Questions
disable-model-invocation: true
---

<!-- aif-ext:aifhub-extension:aif-roadmap:prepend:start -->
## AIFHub Roadmap Audit Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this extension guidance before the base `aif-roadmap` instructions. When any rule below conflicts with the upstream body, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Treat `/aif-roadmap` as an evidence-based maturity audit for this extension workflow, not as a generic milestone brainstorm.

### Required Context

1. Read `.ai-factory/config.yaml` first.
2. Read `.ai-factory/RULES.md` and `.ai-factory/rules/base.md` when present.
3. Read `.ai-factory/DESCRIPTION.md` and `.ai-factory/ARCHITECTURE.md` when present.
4. Read the current `.ai-factory/ROADMAP.md` before editing it.
5. Treat only explicit localization markers from `config.yaml` as saved memory for this skill.
6. If localization markers are missing or incomplete in config, ask before roadmap generation.

### OpenSpec-native evidence

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, roadmap audit may read OpenSpec-native evidence:

- `openspec/specs/**`
- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`
- `openspec/changes/archive/**`
- `.ai-factory/rules/generated/openspec-merged-<change-id>.md`
- `.ai-factory/rules/generated/openspec-change-<change-id>.md`
- `.ai-factory/rules/generated/openspec-base.md`
- `.ai-factory/state/<change-id>/`
- `.ai-factory/qa/<change-id>/`

Use these as evidence only. `/aif-roadmap` must not write runtime state, QA evidence, generated rules, or canonical OpenSpec change artifacts.

When OpenSpec-native evidence is used, summarize the artifact source set in the normal response.

### Managed local lifecycle reconciliation

In check mode, `/aif-roadmap check` owns reconciliation of the configured roadmap's marker-bounded `OpenSpec Change Lifecycle` block. Treat local lifecycle evidence and external GitHub evidence as independent clocks. For local lifecycle reconciliation, mutate only the managed block; the normal roadmap owner may still refresh phase and slice audit content under the existing rules. Preserve evidence-backed state, and do not rewrite canonical OpenSpec, runtime, QA, generated-rules, implementation, or GitHub artifacts.

Apply these local lifecycle rules:

- When an active canonical OpenSpec proposal contains `## Roadmap Linkage` with valid non-`none` roadmap linkage, register one local `planned` row for that change. The row must not claim implementation, verification, finalization, merge, or issue closure.
- For an archived change with durable local done/archive evidence, register a missing `finalized` row or preserve its evidence-backed `finalized` row and its project-relative finalization evidence. Reconciliation must never downgrade `finalized` to `planned`, even if the same linkage is also visible in an active or historical source.
- If an archived change has no durable local done/archive evidence and no current evidence-backed `finalized` row, report bounded local drift instead of fabricating finalization.
- An explicitly unlinked change is one whose standardized section exists and all linkage values are `none`. `/aif-roadmap check` must not create a managed lifecycle row for it and must not infer linkage from branch names, GitHub state, labels, or roadmap text.
- If an existing managed row conflicts with canonical linkage or durable local evidence, report the contradiction and preserve the higher evidence-backed local state rather than guessing or deleting history.
- Reject duplicate, reversed, nested, incomplete, or otherwise malformed lifecycle markers without rewriting unrelated roadmap bytes.

### Independent external reconciliation

Use current GitHub evidence for phase audit and post-merge reconciliation only. When a locally finalized change's GitHub state changes after the final local commit, refresh the current issue, PR, and milestone state in the phase or unphased audit while the managed local lifecycle row remains `finalized`. A remote closure or merge MUST NOT be rewritten as local finalization evidence, must not replace the row's project-relative evidence path, and must not promote `planned` to `finalized`.

When GitHub evidence is unavailable, unauthenticated, rate-limited, offline, or partial, local lifecycle reconciliation continues from canonical and durable local evidence. The GitHub limitation is non-blocking by itself; preserve existing evidence-backed `planned` or `finalized` state and do not invent external status.

In normal output and roadmap audit prose, report lifecycle and GitHub evidence sources separately using bounded summaries:

- `Lifecycle evidence:` project-relative canonical proposal, archive, done, final-summary, or managed-row sources used for `planned`/`finalized` decisions.
- `GitHub evidence:` `used`, `unavailable`, or `partial`, plus public issue/PR/milestone identifiers when available.

Never copy proposal bodies, finalization contents, raw provider output, credentials, or private authentication diagnostics into either source summary.

### GitHub-aware evidence

When GitHub context is available, `/aif-roadmap` may read GitHub-aware evidence as supporting evidence only:

- milestones
- open and closed issues
- open, merged, and closed PRs
- labels
- linked branches
- current git tree, changed files, tags, and recent commits

GitHub context may come from `gh`, a GitHub connector, explicit issue/PR URLs, or caller-provided metadata. GitHub access is optional and non-blocking. If GitHub data is missing, unauthenticated, rate-limited, offline, or only partially available, continue from local repository evidence and summarize whether GitHub evidence was used, unavailable, or partially available.

GitHub state must never replace local proof. A closed issue, completed milestone, or merged PR must never be the sole reason to mark a slice or roadmap item `done`; require local evidence from OpenSpec artifacts, source files, tests, CI, runtime state, QA evidence, generated rules, or other repository artifacts.

Treat GitHub milestones as roadmap phases when milestone evidence is available. Read both open and closed milestones when possible, and state in normal roadmap output whether GitHub milestone evidence was used, unavailable, or partial.

Milestone phase audit rules:

- Closed milestones must produce a phase audit section instead of only disconnected checklist points. Include the milestone title, milestone number when available, closed date when available, linked issues/PRs, and local evidence status.
- Open milestones with `open_issues = 0` are not closed phases. Report `phase-completion drift` with the milestone title, number when available, open/closed issue counts, and the exact reason the phase still needs closure or local evidence.
- Issues and PRs assigned to a milestone attach to that roadmap phase.
- Issues and PRs without a milestone remain in an explicit `unphased backlog/drift` section. For issue #88, keep it unphased unless GitHub assigns it a milestone.
- local artifact evidence remains required before marking a phase, slice, or roadmap item `done`.
- Closed milestone phase audits still require local artifact evidence before any phase or linked roadmap item is marked `done`.

Detect and report drift when material:

- GitHub says done, but local evidence is missing
- local implementation exists, but GitHub is stale
- OpenSpec change exists, but no linked roadmap/milestone/issue is visible
- closed milestone is missing a phase audit
- open milestone has `open_issues = 0`, but the roadmap does not report `phase-completion drift`
- issue or PR milestone linkage is missing from the corresponding phase
- merged PR exists, but current git tree does not contain the expected local evidence

GitHub-aware roadmap output must be credential-safe. It may include public or user-provided identifiers such as issue numbers, PR numbers, milestone names, titles, states, and URLs. It must not write tokens, authorization headers, raw credential helper output, or private authentication diagnostics into `.ai-factory/ROADMAP.md` or normal responses.

`/aif-roadmap` is read-only with respect to GitHub: it must not mutate GitHub issues, milestones, PRs, labels, or linked branches. It must also not write runtime state, QA evidence, generated rules, canonical OpenSpec artifacts, or implementation files.

### Legacy AI Factory-only evidence

When OpenSpec-native mode is not enabled, legacy `.ai-factory/plans/<plan-id>/` and `.ai-factory/specs/<plan-id>/` records may be used as historical roadmap evidence.

### Modes

- Default mode: create or refresh the roadmap audit.
- Check mode: when the user asks to verify, re-audit, or compare the roadmap against the current repository.

### Audit Model

Analyze the repository across these slices:

- Launch / Runtime
- Architecture / Structure
- Core Business Logic
- API / Contracts
- Data / Database / Migrations
- Security / Auth / Secrets
- Integrations / External Services
- Quality / Tests / Validation
- CI/CD / Delivery
- Observability / Logs / Metrics
- Documentation / DX

Status definitions are strict:

- `done`: comprehensive evidence exists in the repository for that slice.
- `partial`: meaningful evidence exists, but important pieces are missing or incomplete.
- `missing`: no meaningful implementation evidence exists, or only aspirational notes exist without working artifacts.

Evidence priority is strict:

- Primary evidence: source code, config files, schemas, tests, pipelines, automation definitions.
- Secondary evidence: project documentation only when it matches the repository state.
- Git history and GitHub state are supporting context only and must never be the sole reason to mark a slice `done`.

### Output Rules

- Prefer `partial` over optimistic grading.
- Preserve manual notes that still match the codebase.
- Treat the roadmap as an audit artifact, not as a generic task list.
- In check mode, call out regressions explicitly.
- Link to GitHub milestones, issues, or PRs where useful, but keep local artifact evidence as the basis for status decisions.
- When GitHub milestones are present, include milestone phase audit output before or alongside slice status output.
- Keep unmilestoned GitHub issues and PRs visible in `unphased backlog/drift` instead of silently folding them into a phase.
- Summarize strongest areas, critical gaps, and any status changes.

### Reference Assets

When available, follow these extension-owned reference files:

- `.ai-factory/extensions/aifhub-extension/injections/references/aif-roadmap/slice-checklist.md`
- `.ai-factory/extensions/aifhub-extension/injections/references/aif-roadmap/roadmap-template.md`

If the extension copy is not installed yet, continue with the rules in this injected block.
<!-- aif-ext:aifhub-extension:aif-roadmap:prepend:end -->

# Roadmap - Strategic Project Planning

Create and maintain a high-level project roadmap with major milestones.

## Workflow

### Step 0: Load Project Context

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.description`, `paths.architecture`, `paths.rules_file`, `paths.roadmap`, `paths.research`, and `paths.rules`; derive `research_bundles_dir = <parent directory of paths.research>/research/`
- **Language:** `language.ui` for prompts, `language.artifacts` for generated content

If config.yaml doesn't exist, use defaults:
- Paths: `.ai-factory/` for all artifacts
- Language: `en` (English)

**Read `.ai-factory/DESCRIPTION.md`** (use path from config) if it exists to understand:
- Tech stack (language, framework, database, ORM)
- Project architecture and conventions
- Non-functional requirements

**Read the resolved architecture artifact** if it exists (`paths.architecture`, default: `.ai-factory/ARCHITECTURE.md`) to understand:
- Chosen architecture pattern and folder structure
- Module boundaries and communication patterns

**Optionally read one relevant research source:** the configured `paths.research`
file or a clearly topic-matching active direct child bundle. Recognize one as
active only when its `INDEX.md` contains `<!-- aif:research-mode:ultra -->` exactly once,
declares `Status: active`, and links its `RESEARCH.md` from `## Artifact Index`.
Use its Active Summary for
strategic context; read optional C4/ADR/dependency artifacts only for rationale.
Do not merge unrelated topics or treat implementation-level notes as milestones.

**Read `.ai-factory/skill-context/aif-roadmap/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the ROADMAP.md
  template. The template in this SKILL.md is a **base structure**. If a skill-context rule says
  "roadmap MUST include X" or "milestones MUST have Y" — you MUST augment the template accordingly.
  Generating a roadmap that violates skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

### Step 1: Determine Mode

If argument is `check` → Mode 3: Check Progress (requires the resolved roadmap path)

Otherwise check if the resolved roadmap path exists (`paths.roadmap`, default: `.ai-factory/ROADMAP.md`):
- **Does NOT exist** → Mode 1: Create Roadmap
- **Exists** → Mode 2: Update Roadmap

---

### Mode 1: Create Roadmap (First Run)

**1.1: Gather Input**

If user provided arguments (vision/description):
- Use as primary input for milestones

If no arguments:
- Ask interactively:

```
AskUserQuestion: What are the major goals for this project?

Options:
1. Let me describe the vision
2. Analyze codebase and suggest milestones
3. Both — I'll describe, you'll add what's missing
```

**Based on choice:**
- "Analyze codebase and suggest milestones" → proceed to Step 1.2
- "Let me describe the vision" or "Both" → collect user description (if "Both", also add codebase analysis in Step 1.2), then ask follow-up:

```
AskUserQuestion: Any priorities or deadlines?

Options:
1. Yes, let me specify
2. No, just order by logical sequence
3. Skip — I'll reprioritize later
```

**1.2: Explore Codebase**

Scan the project to understand what's already built:
- `Glob` for project structure (key directories, modules)
- `Grep` for implemented features (routes, models, services)
- Check git log for completed work: `git log --oneline -20`

**1.3: Generate ROADMAP.md**

Create the resolved roadmap artifact (default: `.ai-factory/ROADMAP.md`) with this format:

```markdown
# Project Roadmap

> <project vision — one-liner from DESCRIPTION.md or user input>

## Milestones

- [ ] **Milestone Name** — short description of what this achieves
- [ ] **Milestone Name** — short description of what this achieves
- [x] **Milestone Name** — short description (already done based on codebase analysis)

## Completed

| Milestone | Date |
|-----------|------|
| Milestone Name | YYYY-MM-DD |
```

**Rules for milestones:**
- Each milestone is a **high-level goal**, not a granular task (that's `$aif-plan`)
- 5-15 milestones is the sweet spot — fewer means too vague, more means too granular
- Order by logical sequence (dependencies first)
- Mark already-completed milestones as `[x]` and add them to the Completed table
- Use today's date for milestones detected as already done

**1.4: Confirm with user**

Show the generated roadmap and ask:

```
AskUserQuestion: Here's the proposed roadmap. What would you like to do?

Options:
1. Looks good — save it
2. Add more milestones
3. Remove/modify some milestones
4. Rewrite — let me give better input
```

Apply changes if requested, then save to the resolved roadmap path.

---

### Mode 2: Update Roadmap (Subsequent Run)

**2.1: Read Current State**

- Read the resolved roadmap path
- Read `.ai-factory/DESCRIPTION.md` (use path from config) for context
- Explore codebase briefly to check what's changed since last update

**2.2: Determine Action**

If user provided arguments (new milestones/changes):
- Apply the requested changes directly

If no arguments:
- Analyze current state and present options:

```
AskUserQuestion: What would you like to do with the roadmap?

Options:
1. Review progress — check what's done, mark completed milestones
2. Add new milestones
3. Reprioritize — reorder existing milestones
4. Rewrite — major revision of the roadmap
```

**2.3: Review Progress (if chosen)**

- Scan codebase for evidence of completed milestones
- For each unchecked milestone, check if the work appears done
- Propose marking completed milestones:

```
These milestones appear to be done:
- **Milestone Name** — [evidence: files exist, routes implemented, etc.]

Mark them as completed?
```

If confirmed:
- Change `- [ ]` to `- [x]` in the Milestones section
- Add entry to Completed table with today's date
- Move completed milestones below unchecked ones (or keep in place — user preference)

**2.4: Add New Milestones (if chosen)**

- Ask user to describe new milestones
- Insert them in logical order among existing milestones
- Update the resolved roadmap path

**2.5: Reprioritize (if chosen)**

- Show current order
- Ask user for new order or let them describe priority changes
- Reorder milestones in the resolved roadmap path

**2.6: Save Changes**

Update the resolved roadmap path with all modifications.

Show summary:
```
## Roadmap Updated

Total milestones: N
Completed: X/N
Next up: **Milestone Name**

To start working on the next milestone:
$aif-plan <milestone description>  → creates a plan and optional branch/worktree flow
$aif-implement                     → executes the plan
```

---

### Mode 3: Check Progress (`$aif-roadmap check`)

Automated scan — analyze the codebase and mark completed milestones without interactive questions.

**Requires** the resolved roadmap path to exist. If it doesn't — tell the user to run `$aif-roadmap` first.

**3.1: Read roadmap and project context**

- Read the resolved roadmap path
- Read `.ai-factory/DESCRIPTION.md` (use path from config) for tech stack context

**3.2: Analyze each unchecked milestone**

For every `- [ ]` milestone:
- Determine what evidence would prove it's done (files, routes, models, configs, tests)
- Use `Glob` and `Grep` to search for that evidence
- Check `git log --oneline --all -30` for related commits
- Score: **done** (strong evidence), **partial** (some work started), **not started**

**3.3: Report findings**

```
## Roadmap Progress Check

✅ Done (ready to mark):
- **User Authentication** — found: src/auth/, JWT middleware, login/register routes
- **Database Setup** — found: migrations/, models/, seed scripts

🔨 In Progress:
- **Payment Integration** — found: src/payments/ exists but Stripe webhook handler missing

⏳ Not Started:
- **Admin Dashboard**
- **Email Notifications**

Mark completed milestones? (2 milestones)
```

**3.4: Apply changes (if confirmed)**

- Mark done milestones `[x]`
- Add entries to Completed table with today's date
- Leave partial and not-started milestones unchanged

Show updated summary:
```
Completed: X/N milestones
Next up: **Milestone Name**
```

---

## ROADMAP.md Format

```markdown
# Project Roadmap

> <project vision — one-liner>

## Milestones

- [ ] **Name** — short description
- [ ] **Name** — short description
- [x] **Name** — short description

## Completed

| Milestone | Date |
|-----------|------|
| Name | YYYY-MM-DD |
```

## Critical Rules

1. **Milestones are high-level** — each represents a major feature or capability, not a task
2. **ROADMAP.md is the source of truth** — always read before modifying
3. **Never remove milestones silently** — always confirm with user before removing
4. **Completed table tracks history** — every checked milestone gets a date entry
5. **NO implementation** — this skill only plans, use `$aif-plan` to start a feature and `$aif-implement` to execute
6. **Ownership boundary** — this command owns roadmap structure/content; `$aif-implement` may only mark milestones completed when implementation evidence is clear
