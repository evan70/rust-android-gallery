---
name: aif-review
description: Perform code review on staged changes or a pull request. Checks for bugs, security issues, performance problems, and best practices. Use when user says "review code", "check my code", "review PR", or "is this code okay". Optional +check flag validates findings via a fresh-context subagent.
argument-hint: "[PR number | branch/commit/tag | empty] [+check]"
allowed-tools: Bash(git *) Bash(gh *) Read Glob Grep Task Agent AskUserQuestion
disable-model-invocation: false
---

<!-- aif-ext:aifhub-extension:aif-review:prepend:start -->
## AIFHub Review Context Provider Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-review` body. When any rule below conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

After config and language resolution, follow `skills/shared/REVIEW-POLICY.md` to resolve and load the configured durable project review policy.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Keep `/aif-review` read-only while applying durable project review guidance and allowing optional, user-owned provider context for version-sensitive library/API review.

### Mode Detection

Before resolving review scope, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with Legacy AI Factory-only mode and state that no OpenSpec-native protocol was detected.

### Project Review Policy

For an OpenSpec-native change with SDD inputs/policy or SDD runtime artifacts, check `ai-factory aifhub-session-brief status --change <change-id> --json` before using a SessionBrief. Missing/stale/invalid context or an unavailable helper returns a planning-owner handoff; review must not compile or repair it. Read protected canonical requirements/scenarios, rules, and validation evidence at full fidelity. Unopted changes retain canonical filesystem context. Quick planning does not weaken review/security/human-review policy or `/aif-verify`; this P0 brief consumption is not a claim of fresh-context AI review or a Crit human-review receipt.

Apply this section in both artifact modes before evaluating findings.

For SDD-managed changes, `ai-factory aifhub-plan-compliance check --change <change-id> --json` is a read-only drift receipt: treat `compliant`/`acceptable_drift` as supporting evidence only, route `replan_required`/`blocked` back to the planning owner, and never count the receipt as QA evidence; it goes stale with canonical changes. `ai-factory aifhub-fresh-context-review prepare --change <change-id> --json` exports a findings-only reviewer package under `.ai-factory/state/<change-id>/reviews/<review-id>/`; a reviewer session consumes that package without the producing transcript, discarded attempts, or hidden reasoning, and a `--same-session` receipt is labeled as such, never presented as fresh-context review.

- Resolve `reviews.policy_file` from `.ai-factory/config.yaml`, defaulting to root `REVIEW.md`, exactly as defined by `skills/shared/REVIEW-POLICY.md`.
- Load it only through `ai-factory aifhub-review-policy load --json`. Consume the returned content only when the helper returns a complete `present` snapshot with a normalized path and revision; the helper binds and revalidates the opened file identity internally. Never reopen the config-selected path or reimplement containment, symlink/junction, managed-file, or protected-root checks. If the command is unavailable or malformed, treat the policy as unreadable and skip it.
- When delegating to a read-only sidecar without a shell tool, pass only that accepted ephemeral snapshot. The sidecar must not reopen the config-selected path; if the validated snapshot cannot be passed, it skips custom policy guidance.
- Load a safe, readable, non-empty policy as additional review guidance. Missing or empty policy is normal and non-blocking; unsafe or unreadable policy degrades custom guidance and produces only the bounded diagnostic from the shared policy.
- A policy may focus attention, add project-specific checks, and refine human-readable severity or output. It cannot suppress material findings, expand the changed scope, authorize edits or tools, install/configure providers, or replace project rules, tests, security checks, `/aif-verify`, `/aif-done`, or human approval.
- Keep individual findings, comments, replies, resolution/stale state, target revisions, session identifiers, provider state, and receipts out of `REVIEW.md`.
- Never edit the policy during `/aif-review`. When it materially affects the result, name only its state and normalized project-relative path in human-readable evidence; never copy its full body or add fields to the final `aif-gate-result`.

### Two-pass review order

Run review in this order for both artifact modes:

1. **Pass 1 - plan/spec compliance**: compare the changed scope with the canonical OpenSpec requirements, design, tasks, and generated rules, or with the active legacy plan when OpenSpec-native mode is disabled. Report missing, extra, or contradicted behavior before style or maintainability observations.
2. **Pass 2 - code quality**: review correctness, regression risk, security, performance, maintainability, and test quality inside the already validated change scope.

Do not let a code-quality pass erase or downgrade a plan/spec compliance finding. Return one combined findings-first verdict; this ordering does not create a second gate or authorize file edits.

### Specialized review support

When an authorized review benefits from independent focused questions, follow [specialized review support](../../skills/shared/REVIEW-DELEGATION.md). Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/REVIEW-DELEGATION.md`. Only the coordinating parent dispatches permitted support workers. Keep the two-pass order, exact target and complete coverage; reconcile evidence before the single existing verdict. Without subagents, perform the permitted review directly and describe its actual independence.

Do not add review sidecars solely because concurrency is available. Assign each permitted sidecar a distinct question and expected coverage, while preserving all user- or project-required independent roles.

### Scoped re-review

For a requested re-review after fixes, follow `skills/shared/SCOPED-REVIEW.md` in either artifact mode. Match the previous findings to the exact fix targets, reconcile every finding by ID, and inspect the complete fix diff for regressions within the existing two-pass review. An incomplete or stale bundle cannot produce PASS. Ordinary full review and user-required independent review retain their scope.

Resolve the policy from the AIFHub extension root; in an installed project it is `.ai-factory/extensions/aifhub-extension/skills/shared/SCOPED-REVIEW.md`. Return the scoped evidence before the single existing gate result; keep review read-only and do not create a replacement policy in the consumer project.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-review` is an optional read-only gate for one active OpenSpec change.

Read context may include:

- changed files;
- canonical OpenSpec artifacts under `openspec/specs/**` and `openspec/changes/<change-id>/`;
- generated rules under `.ai-factory/rules/generated/` when present;
- the configured `reviews.policy_file` when safe, readable, and non-empty;
- runtime state under `.ai-factory/state/<change-id>/` when relevant;
- QA evidence under `.ai-factory/qa/<change-id>/` when relevant;
- reviewed Context7 notes under `.ai-factory/references/context7/` and `.ai-factory/state/<change-id>/context7/`;
- reviewed Graphify outputs under `.ai-factory/references/graphify/` and `.ai-factory/state/<change-id>/graphify/` when dependency or impact review benefits from them.

Context7 guidance:

- Context7 is optional supporting documentation context for current library/API docs.
- `/aif-review` may recommend that the user run Context7 manually outside AIFHub command ownership with commands such as `npx ctx7 library <name> <query>` and `npx ctx7 docs <libraryId> <query>`, or user-installed equivalents `ctx7 library <name> <query>` and `ctx7 docs <libraryId> <query>`.
- Missing Context7, missing Node.js runtime support, missing provider access, or missing reviewed notes is degraded context, not a review failure.
- If the user already configured Context7 MCP, available tools may include `resolve-library-id` plus a docs retrieval tool named `get-library-docs` or `query-docs`; use them only as optional read-only documentation context.
- Do not install `ctx7` or `@upstash/context7-mcp`, run `ctx7`, run `ctx7 setup`, add Context7 dependencies or manifest entries, add Context7 MCP templates to `extension.json`, mutate `.mcp.json`, `.cursor/mcp.json`, `.opencode.json`, agent rules, or agent skills, or start/register Context7 MCP automatically.
- Treat Context7 output as supporting context only. Findings must be source-grounded in changed files, canonical OpenSpec artifacts, generated rules, runtime state, QA evidence, package files in the repository, or other direct repository evidence.
- Do not store raw Context7 output, MCP transcripts, API responses, setup output, or generated provider configuration under `openspec/changes/<change-id>/`, `openspec/specs/`, `.ai-factory/rules/generated/`, or `.ai-factory/qa/<change-id>/`.
- Do not persist `CONTEXT7_API_KEY`, API keys, tokens, raw authorization headers, credential helper output, private provider diagnostics, private backend diagnostics, or unreviewed sensitive output in `.ai-factory/`, `openspec/`, docs, runtime state, QA evidence, generated rules, or Context7 reference copies.

Write boundaries:

- Do not edit files.
- Do not write OpenSpec artifacts, runtime state, QA evidence, generated rules, provider notes, MCP config, provider config, or provider setup files.
- Return the review result in the response only unless the user explicitly pipes or saves it with another command.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve upstream review behavior and keep the command read-only.

- Review only the changed scope and legacy plan context that already exists.
- Apply the configured review policy under the same shared resolution, authority, and read-only boundaries.
- Context7 remains optional supporting documentation context; missing Context7 is degraded context, not a review failure.
- The same user-owned Context7 boundaries apply: no install, no `ctx7`, no `ctx7 setup`, no MCP mutation, no automatic MCP registration, no provider templates, and no file edits.
- Findings must still be grounded in changed files, existing project artifacts, tests, or other direct repository evidence.

### Output

- Start with findings ordered by severity.
- Include provider context in evidence only when it materially influenced a finding.
- Include review policy state and its normalized project-relative path in human-readable evidence when it materially influenced the review; do not copy policy contents.
- If Context7 or Graphify was unavailable or not used, mention it only when relevant to residual risk.
- End with the upstream `aif-gate-result` contract for `/aif-review`.
<!-- aif-ext:aifhub-extension:aif-review:prepend:end -->

# Code Review Assistant

Perform thorough code reviews focusing on correctness, security, performance, and maintainability.

## Step 0: Load Config

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.description`, `paths.architecture`, `paths.rules_file`, `paths.roadmap`, and `paths.rules`
- **Language:** `language.ui` for review summary language
- **Git:** `git.base_branch` for branch comparison guidance

If config.yaml doesn't exist, use defaults:
- Paths: `.ai-factory/` for all artifacts
- Language: `en` (English)
- Git: `base_branch: main`

## Behavior

### Argument flags

Before routing the argument string into one of the modes below, extract any standalone tokens that flag optional behavior. Strip them from the argument string and route the remainder normally.

- `+check` — runs the optional findings validator after the review is produced. The full procedure (when to run, failure modes, output additions, gate-result recomputation) lives in `references/CHECK-MODE.md`. Default is OFF; the validator runs only when this token is present. The token may appear before or after the main argument (e.g. `$aif-review +check`, `$aif-review 123 +check`, `$aif-review main +check`).

If the leftover argument string is empty, fall back to the empty-argument mode (staged review). Unknown `+`-prefixed tokens are passed through as part of the main argument so they are not silently consumed.

> Edge case: a git ref literally named `+check` will be consumed by the flag stripper — acceptable compromise.

### Without Arguments (Review Staged Changes)

1. Run `git diff --cached` to get staged changes
2. If nothing staged, run `git diff` for unstaged changes
3. Analyze each file's changes

### With PR Number/URL

1. Use `gh pr view <number> --json` to get PR details
2. Use `gh pr diff <number>` to get the diff
3. Review all changes in the PR

### With Git Ref (Commits Mode)

Argument routing chain:
1. **Empty** → staged review (see above)
2. **Digits or `#N`** → PR mode (see above)
3. **Everything else** → validate via `git rev-parse --verify` → commits mode or ask user

Validation:
```bash
git rev-parse --verify <argument> 2>/dev/null
```

- **Valid ref** → enter commits mode (steps below)
- **Invalid ref** → do NOT fall back to staged review silently. Ask the user to clarify:

  ```
  AskUserQuestion: `<argument>` is not a valid git ref. What did you mean?

  Options:
  1. Review staged changes instead
  2. Cancel
  ```

  **Based on choice:**
  - "Review staged changes" → run staged review (default mode)
  - "Cancel" → inform the user that review was cancelled → **STOP**
  - "Other" → user provides corrected ref → re-validate via `rev-parse`

> Edge case: a branch with a purely numeric name (e.g. `123`) will be interpreted as a PR number — acceptable compromise.

**Steps:**

1. **Get commit list** between the ref and HEAD:
   ```bash
   git log --oneline --reverse <ref>..HEAD
   ```
   If no commits found (HEAD is at or behind `<ref>`), inform the user and **stop**.

2. **Check commit count:**
   If more than 20 commits, ask the user before proceeding:

   ```
   AskUserQuestion: Found <N> commits to review. Reviewing all of them will be slow and consume significant context. How to proceed?

   Options:
   1. Review all <N> commits
   2. Review only the last 20
   3. Cancel
   ```

   **Based on choice:**
   - "Review all" → continue with the full commit list
   - "Review only the last 20" → truncate the list to the 20 most recent commits (keep chronological order)
   - "Cancel" → inform the user that review was cancelled → **STOP**

3. **Review each commit:**
   ```bash
   git show <commit-hash> --stat
   git show <commit-hash>
   ```
   For each commit check:
   - Does the commit message match the actual changes?
   - Are changes atomic (single logical unit per commit)?
   - Are there any issues introduced in this specific commit?

4. **Provide combined summary** with per-commit notes

## Context Gates (Read-Only)

Before finalizing review findings, run read-only context gates:

- Check the resolved architecture artifact (if present) for boundary/dependency alignment issues.
- Check the resolved RULES.md artifact (if present) for explicit convention violations.
- Check the resolved roadmap artifact (if present) for milestone alignment and mention missing linkage for likely `feat`/`fix`/`perf` work.

Human gate result severity:
- `WARN` for non-blocking inconsistencies or missing optional files.
- `ERROR` only for explicit blocking criteria requested by the user/review policy.

If the user wants a standalone rules-only pass, suggest `$aif-rules-check`. Keep human `$aif-review` gate labels at `WARN` / `ERROR`, then append the standard machine-readable gate result with `pass|warn|fail` status.

### Machine-readable gate result

This section is the single owner of `aif-gate-result` computation:
- Append one final fenced `aif-gate-result` JSON block after the human-readable review.
- Use `"gate": "review"`.
- `"status": "pass|warn|fail"` — the more severe (`fail` > `warn` > `pass`) of two independent inputs:
  - **findings input** — `fail` when any "Critical Issues" item remains (critical correctness, security, data-loss, performance, downstream regression — see `references/SEVERITY.md` for the authoritative critical/suggestion definitions); `warn` when only "Suggestions", missing optional context, or review uncertainty remain; `pass` when nothing material remains.
  - **context-gate input** — `fail` for a blocking (`ERROR`) gate finding; `warn` for a non-blocking (`WARN`) one; `pass` when none.
  - A failing context gate keeps `"status"` at `fail` even with zero Critical Issues — a clean findings list must never mask a failed gate.
- `"blocking": true|false` — `true` only when `"status"` is `fail`.
- `"blockers"` — merge-blocking findings only: every "Critical Issues" item and every blocking context-gate finding, nothing else.
- `"affected_files"` — reviewed or implicated paths.
- `"suggested_next.command"` follows `"status"`: `fail` → `$aif-fix` by default, but if every blocker came from a single context gate point at that gate's command instead (rules gate → `$aif-rules`, architecture gate → `$aif-architecture`, roadmap gate → `$aif-roadmap`); `warn`/`pass` → `$aif-commit`; `null` only when no command fits.

`$aif-review` is read-only for context artifacts by default. Do not modify context files unless user explicitly asks.

### Project Context

**Read `.ai-factory/skill-context/aif-review/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the review
  summary format and the checklist criteria. If a skill-context rule says "review MUST check X"
  or "summary MUST include section Y" — you MUST augment the output accordingly. Producing a
  review that ignores skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

## Review Checklist

### Correctness
- [ ] Logic errors or bugs
- [ ] Edge cases handling
- [ ] Null/undefined checks
- [ ] Error handling completeness
- [ ] Type safety (if applicable)

### Security
- [ ] SQL injection vulnerabilities
- [ ] XSS vulnerabilities
- [ ] Command injection
- [ ] Sensitive data exposure
- [ ] Authentication/authorization issues
- [ ] CSRF protection
- [ ] Input validation

### Performance
- [ ] N+1 query problems
- [ ] Unnecessary re-renders (React)
- [ ] Memory leaks
- [ ] Inefficient algorithms
- [ ] Missing indexes (database)
- [ ] Large payload sizes

### Best Practices
- [ ] Code duplication
- [ ] Dead code
- [ ] Magic numbers/strings
- [ ] Proper naming conventions
- [ ] SOLID principles
- [ ] DRY principle

### Testing
- [ ] Test coverage for new code
- [ ] Edge cases tested
- [ ] Mocking appropriateness

## Output Format

```markdown
## Code Review Summary

**Files Reviewed:** [count]
**Risk Level:** 🟢 Low / 🟡 Medium / 🔴 High

### Context Gates
[Architecture / Rules / Roadmap gate results with WARN/ERROR labels]

### Critical Issues
[Each item is a short paragraph in prose, not a labeled record. Order inside the paragraph:
1. Behavioral impact — what breaks for the user or downstream code.
2. Optional note — a code citation, a consequence, or extra context. Include only if it adds signal; skip otherwise.
3. Path — file:line of the affected location (or the closest anchor).
4. Suggested fix — concrete edit that addresses the behavior above.

Example:
> Two clients buying the last item both get a confirmation and stock goes negative — the order creation and stock reservation run in separate transactions. `src/services/order.ts:42`. Wrap `OrderService.create` and `InventoryService.reserve` in a shared transaction so the second buyer fails fast with "out of stock".]

### Suggestions
[Same item shape as Critical Issues. The behavioral impact describes a non-blocking improvement (clarity, performance budget, missing log), not a bug.]

### Questions
[Free-form clarifications. Path optional, fix optional — these are open questions for the author, not findings.]

### Positive Notes
[Free-form acknowledgements of good patterns. No path/fix required.]
```

When `+check` reclassifies an item, a short ` [+check: …]` suffix is appended to the item text; see `references/CHECK-MODE.md` for the exact wording.

Append the final machine-readable result after the markdown summary:

```aif-gate-result
{
  "schema_version": 1,
  "gate": "review",
  "status": "pass",
  "blocking": false,
  "blockers": [],
  "affected_files": [],
  "suggested_next": {
    "command": "$aif-commit",
    "reason": "Review found no blocking issues."
  }
}
```

When the `+check` flag is set, the `aif-gate-result` block is assembled **after** validator filtering — `status`, `blockers`, `affected_files`, and `suggested_next` are recomputed accordingly. Exception: the whole-dispatch failure path keeps the unfiltered original list and does NOT recompute these fields. See `references/CHECK-MODE.md` for the full procedure.

## Review Style

- Be constructive, not critical
- Explain the "why" behind suggestions
- Provide code examples when helpful
- Acknowledge good code
- Prioritize feedback by importance
- Ask questions instead of making assumptions

## Examples

**User:** `$aif-review`
Review staged changes in current repository.

**User:** `$aif-review 123`
Review PR #123 using GitHub CLI.

**User:** `$aif-review https://github.com/org/repo/pull/123`
Review PR from URL.

**User:** `$aif-review 2.x`
Review all commits on the current branch compared to branch `2.x`.

**User:** `$aif-review main`
Review all commits on the current branch compared to `main` (or to whatever branch is configured as `git.base_branch` in this repository).

**User:** `$aif-review v1.0.0`
Review all commits on the current branch compared to tag `v1.0.0`.

**User:** `$aif-review +check`
Review staged changes, then run the `+check` validator over Critical Issues and Suggestions before rendering. The validator can drop invented items, rewrite partially-correct ones, and reclassify items between the two severity levels (promote a suggestion to critical or demote a critical to suggestion — see `references/SEVERITY.md` for the rules). It adds a filtering-summary line and rebuilds the gate result from the surviving findings; see `references/CHECK-MODE.md` for the exact line format.

**User:** `$aif-review 123 +check`
Review PR #123 with `+check` validation enabled.

## Integration

If GitHub MCP is configured, can:
- Post review comments directly to PR
- Request changes or approve
- Add labels based on review outcome

> **Tip:** Context is heavy after code review. Consider `/clear` or `/compact` before continuing with other tasks.
