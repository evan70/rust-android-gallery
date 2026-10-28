---
name: aif-architecture
description: Generate architecture guidelines for the project. Analyzes tech stack from DESCRIPTION.md, recommends an architecture pattern, and creates .ai-factory/ARCHITECTURE.md. Use when setting up project architecture, asking "which architecture", or after $aif setup.
argument-hint: "[microservices|layers|structured|structured-layers|structured-vertical|explicit|explicit-layers|explicit-vertical|explicit-flat] (legacy aliases: clean, ddd, monolith, vertical → mapped to explicit or structured)"
allowed-tools: Read Write Glob Grep Bash(mkdir *) AskUserQuestion Questions
disable-model-invocation: false
---

<!-- aif-ext:aifhub-extension:aif-architecture:prepend:start -->
## AIFHub Architecture Context Boundary

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-architecture` body. When any rule below conflicts with the upstream body, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Keep upstream `/aif-architecture` as the command owner while adding AIFHub/OpenSpec artifact-boundary guidance.

Do not create or use an extension-owned `skills/aif-architecture/` directory. The upstream skill remains responsible for architecture analysis, recommendation, artifact generation, and project-specific `.ai-factory/skill-context/aif-architecture/SKILL.md` loading.

### Mode Detection

Before resolving architecture scope, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with upstream defaults and state that no AIFHub artifact protocol was detected.

Always respect upstream config resolution for:

- `paths.description`
- `paths.architecture`
- `language.ui`
- `language.artifacts`
- `language.technical_terms`

### Focused architecture assessment

Apply in either artifact mode after resolving the permitted context. When the user names an area or pain point, inspect that area first. Otherwise, use bounded local change history to identify frequently changed areas, then verify the suspected friction in current source and callers. Generated files, mass formatting and commit frequency alone do not prove an architectural problem. State the inspected scope and limitations; do not imply a whole-repository audit from a few samples.

For each useful candidate, identify concrete affected paths and a current caller cost: scattered invariants, repeated knowledge, leaking dependencies or a real interaction that is difficult to test. Explain what a proposed boundary would hide, which behavior and constraints must remain, how callers and tests would change, and the migration cost. Fewer files, fewer methods or a thinner wrapper is not enough evidence of improvement. Prefer a small number of supported candidates; preserve project terminology and avoid hypothetical adapters or unrelated cleanup.

Read applicable accepted architecture decisions. Revisit one only with new source evidence or changed constraints, clearly naming the conflict. Consider a new ADR when reversal would be costly, the rationale would be surprising later, and real alternatives were weighed; existing project ADR requirements remain authoritative. Keep candidates in the resolved architecture context or normal response. These criteria do not add an ADR write path, automatic glossary update, HTML report, provider setup or source-code edit. Route implementation to the existing planning owner.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-architecture` still writes project-level AI Factory architecture context, not canonical OpenSpec lifecycle artifacts.

Allowed writes are limited to:

- the resolved `paths.architecture` artifact;
- an architecture pointer in the resolved `paths.description` artifact;
- an architecture row in the root `AGENTS.md` context table.

Read-only context may include:

- `openspec/specs/**`;
- `openspec/changes/<change-id>/proposal.md`;
- `openspec/changes/<change-id>/design.md`;
- `openspec/changes/<change-id>/tasks.md`;
- `openspec/changes/<change-id>/specs/**/spec.md`;
- `.ai-factory/rules/generated/openspec-merged-<change-id>.md`;
- `.ai-factory/rules/generated/openspec-change-<change-id>.md`;
- `.ai-factory/rules/generated/openspec-base.md`;
- `.ai-factory/state/<change-id>/`;
- `.ai-factory/qa/<change-id>/`;
- the resolved roadmap, rules, research, and description artifacts.

Do not write:

- `openspec/changes/**`;
- `openspec/specs/**`;
- `.ai-factory/state/**`;
- `.ai-factory/qa/**`;
- `.ai-factory/rules/generated/**`;
- provider notes, MCP config, provider config, or provider setup files.

If an architecture decision would change product or workflow behavior, route that work through `/aif-plan full <request>` instead of writing OpenSpec delta specs directly from `/aif-architecture`.

### Optional Context Providers

Before using optional context providers, call the installed selector when it is available:

```bash
ai-factory aifhub-memory-tools select --from-project --command aif-architecture --json
```

Use only tools returned in `selected_tools`. If the selector is unavailable, fails, or returns no selected tools, continue with the `rg` baseline and direct repository evidence.

For `/aif-architecture`, optional providers are supporting context only:

- `rg` may be used as a safe availability probe and direct repository search baseline.
- Graphify may be read only from existing reviewed outputs in allowed project or change context paths.
- Context7 may be read only from existing reviewed notes in allowed project or change context paths.
- `codex-agent-mem`, `context-mode`, `codex-mem`, `eagle-mem`, and CodeGraph are forbidden for this command.

Do not install providers, run provider setup, index source, sync memory, register MCP servers, start background services, or execute CodeGraph from `/aif-architecture`.

Treat optional provider output as hypotheses or supporting notes. Final architecture guidance must remain grounded in direct repository evidence, resolved AI Factory config, existing canonical OpenSpec artifacts, generated rules, runtime state, QA evidence, or reviewed provider notes from allowed paths.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve upstream `/aif-architecture` behavior and keep the same project-level write boundaries:

- resolved `paths.architecture`;
- architecture pointer in resolved `paths.description`;
- architecture row in root `AGENTS.md`.

Do not write OpenSpec artifacts, runtime state, QA evidence, generated rules, provider notes, MCP config, provider config, or provider setup files.
<!-- aif-ext:aifhub-extension:aif-architecture:prepend:end -->

# Architecture - Generate Architecture Guidelines

Generate `.ai-factory/ARCHITECTURE.md` with architecture decisions tailored to the project.

## Workflow

### Step 0: Load Config & Project Context

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.description` and `paths.architecture`
- **Language:** `language.ui` for prompts and `language.artifacts` for generated architecture content

When invoked by `$aif`, assume `.ai-factory/config.yaml` has already been written for the current setup run and already contains the resolved `language.ui` / `language.artifacts` values.

If config.yaml doesn't exist, use defaults:
- DESCRIPTION.md: `.ai-factory/DESCRIPTION.md`
- ARCHITECTURE.md: `.ai-factory/ARCHITECTURE.md`
- Language: `en` (English)

**THEN:** Read `.ai-factory/DESCRIPTION.md` (use path from config) if it exists to understand:
- Tech stack (language, framework, database, ORM)
- Project size and complexity
- Core features and requirements
- Non-functional requirements

**If `.ai-factory/DESCRIPTION.md` does not exist:**
```
⚠️  No project description found.

Run $aif first to set up project context, or describe your project manually:
- What are you building?
- Tech stack (language, framework, database)?
- Team size?
- Expected scale?
```

Allow standalone usage — if user provides manual input, use that instead.

**Read `.ai-factory/skill-context/aif-architecture/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including the
  ARCHITECTURE.md template. The template in this SKILL.md is a **base structure**. If a skill-context
  rule says "architecture doc MUST include X" or "MUST cover section Y" — you MUST augment the
  template accordingly. Generating ARCHITECTURE.md that violates skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

### Step 1: Recommend Architecture

Based on project context, evaluate against the decision matrix and recommend an architecture:

**If `$ARGUMENTS` specifies an architecture** (e.g., `$aif-architecture explicit`):
- **Direct mapping** (no suffix needed):
  - `layers` → Layered Architecture
  - `microservices` → Microservices
  - `structured` → Structured Modules (ask variant — see below)
  - `explicit` → Explicit Architecture (ask variant — see below)
- **Legacy aliases** (deprecated, mapped to current patterns — may be removed in future):
  - `clean` → Explicit Architecture
  - `ddd` → Explicit Architecture
  - `monolith` → Structured Modules
  - `vertical` → Explicit Architecture (Vertical Slice By Entity)
- **With suffix** (variant is determined, no need to ask):
  - `structured-layers` → Structured Modules (Technical Layer)
  - `structured-vertical` → Structured Modules (Vertical Slices By Entity)
  - `explicit-layers` → Explicit Architecture (Technical Layer)
  - `explicit-vertical` → Explicit Architecture (Vertical Slice By Entity)
  - `explicit-flat` → Explicit Architecture (Flat Vertical Slice - Simplified)
- **Without suffix** (ask user to choose variant):
  - If `structured` is specified without a suffix: ASK the user: "Which folder structure variant do you prefer for Structured Modules? 1. Technical Layer (simpler) or 2. Vertical Slices by Entity (better for large modules)". Wait for their answer before generating the artifact.
  - If `explicit` is specified without a suffix: ASK the user: "Which folder structure variant do you prefer for Explicit Architecture? 1. Technical Layer or 2. Explicit Architecture (Vertical Slice By Entity) or 3. Explicit Architecture (Flat Vertical Slice - Simplified)". Wait for their answer before generating the artifact.
- Use the resolved architecture directly, skip the recommendation step and proceed to Step 1.5

**If no specific architecture requested:**
- Evaluate the project against the decision matrix (see `references/architecture.md`)
- Consider: team size, domain complexity, scale requirements, tech stack
- Present recommendation via `AskUserQuestion`:

```
Based on your project context:
- [reason 1 from project analysis]
- [reason 2 from project analysis]

Which architecture pattern should we use?

1. [Recommended pattern] (Recommended) — [why it fits]
2. [Alternative 1] — [brief reason]
3. [Alternative 2] — [brief reason]
4. [Alternative 3] — [brief reason]
```

Architecture options:
- **Structured Modules (Technical Layer)**
- **Structured Modules (Vertical Slices By Entity)**
- **Explicit Architecture (Technical Layer)**
- **Explicit Architecture (Vertical Slice By Entity)**
- **Explicit Architecture (Flat Vertical Slice - Simplified)**
- **Microservices**
- **Layered Architecture**
(See `references/architecture.md` for detailed descriptions of these patterns to formulate your recommendation).

**CRITICAL INSTRUCTION:** You MUST read `references/architecture.md` before generating the `ARCHITECTURE.md` artifact to ensure correct terminology, dependency directions.

### Step 1.5: Codebase Alignment Check

**CRITICAL:** Before generating the document, compare the chosen architecture's ideal folder structure (from `references/architecture.md`) against the actual existing codebase structure.

- If the project is empty or mostly matches: proceed to Step 2.
- **If there are significant discrepancies:** DO NOT silently merge the ideal architecture with the messy reality. You MUST stop and ask the user how to proceed via `AskUserQuestion`:

```
The current project structure differs significantly from the ideal [Pattern Name] architecture.
[Briefly list 1-2 major differences]

How should we generate the ARCHITECTURE.md?
1. Adapt the guidelines to fit the existing application structure (document reality).
2. Generate the pure, strict architecture guidelines (requires refactoring the application later to match).
```
Wait for their decision before proceeding to Step 2.

### Step 2: Generate the Architecture Artifact

Create the parent directory for the resolved architecture path if needed.

Generate the resolved architecture artifact (default: `.ai-factory/ARCHITECTURE.md`) with the following structure, **adapted to the project's tech stack and language**:

```markdown
# Architecture: [Pattern Name]

## Overview
[1-2 paragraphs: what this architecture is and why it was chosen for THIS project]

## Decision Rationale
- **Project type:** [from DESCRIPTION.md]
- **Tech stack:** [language, framework]
- **Key factor:** [primary reason for this choice]

## Folder Structure
\`\`\`
[folder structure adapted to the project's tech stack]
[use actual framework conventions — e.g., Next.js app/ dir, Laravel app/ dir, Go cmd/ dir]
\`\`\`

## Dependency Rules
[What depends on what. Inner vs outer layers. Module boundaries.]

- ✅ [allowed dependency direction]
- ❌ [forbidden dependency direction]

## Layer/Module Communication
[How layers or modules communicate with each other]
- [pattern 1]
- [pattern 2]

## Key Principles
1. [Principle 1 — adapted to this project]
2. [Principle 2]
3. [Principle 3]

[If the user chose Option 2 (strict architecture) in Step 1.5, add the following section:]
## Legacy vs New Code Policy
- **New Features:** All new code MUST strictly follow the architecture defined in this document.
- **Legacy Code Modification:** Do NOT automatically refactor unrelated legacy code to fit this architecture. Touch legacy code only when necessary for bug fixes, when tasked with explicit refactoring, or when adapting it to be consumed by new features.
- **Interoperability:** When new code must call legacy code, isolate the interaction using adapters, interfaces, or facades so that legacy patterns do not pollute the new architecture.

[If the user chose Option 1 (adapt to reality) in Step 1.5, add the following lighter section:]
## Code Organization Note
- **New Features:** All new code should follow the architecture defined in this document where practical.
- **Existing Code:** Document the current structure as-is. When modifying existing code, prefer following the architectural conventions in this document, but do not force a rewrite of unrelated code.
- **Interoperability:** When new code must call existing code, prefer clean interfaces but do not refactor purely for structural alignment.

## Code Examples

### [Example 1 title]
\`\`\`[language]
[code example in the project's language/framework]
\`\`\`

### [Example 2 title]
\`\`\`[language]
[code example showing dependency rule]
\`\`\`

## Anti-Patterns
- ❌ [What NOT to do in this architecture]
- ❌ [Common mistake to avoid]
```

**Rules for generation:**
- Adapt ALL examples to the project's language and framework (don't use TypeScript examples for a Go project)
- Use the project's actual conventions (import paths, naming, etc.)
- Keep it practical — focus on rules that affect day-to-day development
- Base the generated folder structure on the user's decision in Step 1.5 (either adapted to reality or strict pure architecture). Do not automatically merge them without user consent.

### Step 3: Update DESCRIPTION.md

If the resolved DESCRIPTION.md path exists, add or update an architecture-pointer section in resolved `language.artifacts`.
Use the resolved architecture path from config, not the default path literal.

```markdown
## [Localized heading: Architecture]
[Localized sentence in resolved artifacts language referencing the resolved architecture artifact path for detailed architecture guidelines.]
[Localized label: Pattern]: [chosen pattern name]
```

### Step 4: Update AGENTS.md

If `AGENTS.md` exists in the project root, add the resolved architecture artifact path to the localized "AI Context Files" table in resolved `language.artifacts`:

```markdown
| [resolved-architecture-path] | [Localized architecture artifact description in resolved artifacts language] |
```

Only add if the resolved architecture path is not already present.

### Step 5: Confirm

Present the confirmation in resolved `language.ui` and report the resolved architecture path:

```
[Localized success heading in `language.ui`]

[Localized pattern label in `language.ui`]: [chosen pattern]
[Localized file label in `language.ui`]: [resolved architecture path]

[Localized key-rules heading in `language.ui`]:
- [rule 1]
- [rule 2]
- [rule 3]

[Localized closing sentence in `language.ui` about workflow skills following these architecture guidelines.]
```

## Artifact Ownership

- Primary ownership: the resolved architecture artifact path (default: `.ai-factory/ARCHITECTURE.md`).
- Respect config overrides: write to the resolved architecture path from `config.yaml` when provided.
- Allowed companion updates: architecture pointer in the resolved DESCRIPTION path from `config.yaml`, architecture row in `AGENTS.md` context table.
- Read-only context: roadmap, rules, research, and plan artifacts unless user explicitly requests otherwise.

---
