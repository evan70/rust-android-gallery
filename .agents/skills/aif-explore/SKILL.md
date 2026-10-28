---
name: aif-explore
description: Enter explore mode to investigate ideas, systems, problems, and requirements before planning. Use explicit ultra mode when the user wants a durable adaptive research bundle with an index and only the justified C4, ADR, or dependency artifacts.
argument-hint: "[ultra] [topic or plan name]"
allowed-tools: Read Glob Grep Write Edit Bash Task AskUserQuestion Questions
disable-model-invocation: true
---

<!-- aif-ext:aifhub-extension:aif-explore:prepend:start -->
## AIFHub OpenSpec-native Override

Read [tool selection and artifact ownership](../../skills/shared/TOOLS.md) before choosing artifact paths or lifecycle instructions.

Apply this block before the upstream `aif-explore` body. When any rule below conflicts with the base skill text, this block wins.

Follow `skills/shared/LANGUAGE-POLICY.md` before producing user-facing responses or generated artifacts.

Resolve user-facing prose language in this order: use a usable non-empty `language.ui`; otherwise preserve the current conversation language for this response only; use English only when that language is indeterminate. This rule overrides downstream generic English defaults; do not infer from OS locale or persist the inferred choice. On that hard-English fallback, add exactly one concise setup hint only when the output contract permits human-readable prose, before any required final machine-readable block; never add it inside or after `aif-gate-result`, and never alter exact handoffs, fixed commands, paths, keys/enums, or machine-only output.

### Goal

Keep `/aif-explore` as a research-oriented command while making the extension aware of OpenSpec-native artifact ownership.

### Mode Detection

Before resolving exploration inputs, read `.ai-factory/config.yaml` when it exists.

- If the config contains `aifhub.tools.openspec: true`, use **OpenSpec-native mode**.
- Otherwise, use **Legacy AI Factory-only mode**.
- If the config is missing, continue with Legacy AI Factory-only mode and state that no OpenSpec-native protocol was detected.

### Research profile and AI Factory version gate

Resolve `paths.research` and parse the command-position mode before creating a file or directory. Remove only an explicit leading `ultra` token; preserve every later occurrence of `ultra` in the topic byte-for-byte.

- Regular `/aif-explore <topic>` writes only the resolved `paths.research` file (default `.ai-factory/RESEARCH.md`). It does not create a research bundle or change-scoped runtime note.
- Explicit `/aif-explore ultra <topic>` must call the shared dependency-free `resolveAiFactoryUltraSupport()` contract from `scripts/ai-factory-version-resolver.mjs` before any write. Use injected test toolchain/version first, then project `.ai-factory.json.version`, then CLI evidence only with proven matching project provenance.
- When stable project metadata exists, unverified global/PATH-only CLI evidence is ignored with a bounded `ai-factory-cli-provenance-unverified` warning and does not override the project version.
- Missing, malformed, prerelease, unsupported `<2.18.0`, or provenance-matched CLI/project mismatch is a no-write stop. Recommend regular `/aif-explore <topic>`; do not redirect this failure to `/aif-plan full` and do not silently perform a regular research write.
- The extension baseline version never enables ultra by itself.

For explicit ultra on stable AI Factory `>=2.18.0`, derive exactly one bundle root without adding a config key:

```text
research_bundles_dir = <parent directory of resolved paths.research>/research/
bundle = <research_bundles_dir>/<english-topic-slug>/
```

- Create or update only one valid marked bundle for the run. Do not write the regular `paths.research` file in the same run.
- The minimum bundle is `INDEX.md` plus `RESEARCH.md`. `INDEX.md` must contain exactly one standalone `<!-- aif:research-mode:ultra -->`, identify `Status: active`, and link `RESEARCH.md` from `## Artifact Index` with a safe direct relative link.
- Reuse an exact matching marked slug. If the slug path is an unmarked directory, has a missing/duplicate/code-only marker, contains an unsafe Artifact Index link, or otherwise collides with a different artifact, fail closed without changing either regular research or the directory.
- Create `C4-CONTEXT.md`, `C4-CONTAINER.md`, `C4-COMPONENT-<scope>.md`, `ADR-NNNN-<slug>.md`, or `DEPENDENCY-GRAPH.md` only when its upstream evidence-based inclusion gate is satisfied. Do not create empty placeholders; link every generated optional artifact from `INDEX.md` exactly once.

Bounded diagnostics may contain only `mode`, resolved version and version `source`, safe project-relative bundle path, created artifact names, and a stable invalid-marker/collision `code`. Never include topic/research bodies, provider output, credentials, raw stdout, raw stderr, or private absolute paths.

### Dependency-aware research brief interview

After resolving the artifact mode, research profile, and any ultra version gate, but before the full research run or any write, map the request as a dependency-aware **design tree**.

When independent fact-finding or a material interface comparison needs separate attention, read [research delegation and design alternatives](../../skills/shared/RESEARCH-DESIGN.md). Resolve it from the installed extension at `.ai-factory/extensions/aifhub-extension/skills/shared/RESEARCH-DESIGN.md`. Before brief confirmation, use only its bounded fact-finding path; design comparisons and full research must stay within the confirmed research brief. The coordinating parent owns dispatch and synthesis; delegated workers return their findings without writing artifacts or opening another interview. Small lookups retain the direct read-only path.

- Treat explicit decisions already present in the request or conversation as settled roots. Each unresolved user-owned decision is a node whose children are the decisions that depend on it.
- The **frontier** is every unresolved decision whose prerequisites are settled. Ask the whole current frontier in one round, number each question, and include one concise recommended answer with its main rationale or tradeoff. Never ask a downstream question while one of its prerequisites remains open.
- If the runtime imposes a smaller question-count limit, ask the maximum supported independent subset. Keep the remaining nodes on the same frontier; do not treat the partial batch as a completed round.
- After each answer batch, update the design tree, preserve settled answers unless the user reopens them, recompute the frontier, and ask the next round. Do not drip questions one at a time when independent frontier questions can be asked together.
- Repository, configuration, and tool-availability facts are the assistant's responsibility. Resolve obtainable facts with bounded read-only inspection instead of asking the user. While fact-finding is pending, keep only the dependent questions off the frontier and continue with independent questions.
- `Bounded read-only inspection` does not grant a new read scope. In OpenSpec-native mode, constrain it to the `Allowed read context` and `Enabled optional tool use` boundaries below. In Legacy AI Factory-only mode, constrain it to `.ai-factory/config.yaml`, safe resolved configured project context and rules, the exact referenced legacy plan pair when present, directly relevant in-repository source/tests/docs/package manifests, and bounded local Git branch/revision metadata.
- Never use the interview to read outside the project root, inspect environment or credential stores, consume raw optional-provider stores/output, scan unrelated repositories, or enable an optional provider solely for pre-confirmation fact-finding. These limits do not replace the explicit read-only ultra version gate above.
- The user owns product, scope, risk, and tradeoff decisions. If a decision cannot be made without a prototype, measurement, or unavailable evidence, record that dependency as an open blocker and recommend the smallest evidence-producing next action; do not invent an answer.
- Keep the tree scoped to a research brief: the target question, boundaries and non-goals, constraints, evidence standard, and desired deliverable. Do not turn `/aif-explore` into an implementation plan or canonical OpenSpec design session.

The interview ends only when the frontier is empty because every user-owned brief decision node is settled, not merely because no node is currently askable. An empty frontier with any unresolved user-owned decision is blocked, not complete: report the blocked or cyclic prerequisite and the smallest evidence-producing or dependency-breaking next action, and do not present the brief for confirmation or start the full research run. Pending repository fact-finding that is a prerequisite for a user-owned decision has the same blocked status.

Once the interview is complete, present one normalized research brief. An explicit user-authored request is itself the confirmed brief when it settles every user-owned brief dimension — target question, boundaries and non-goals, constraints, evidence standard, and desired deliverable — through decisions explicitly present in the request or already confirmed in conversation, and explicitly authorizes the same read-only research. This requires every user-owned brief decision to be settled, with no pending prerequisite fact-finding and no blocked or cyclic unresolved decision. An empty frontier alone is insufficient. For this precise-request fast path, present the derived normalized brief once as a non-blocking notice and continue without a second confirmation. Do not add decisions, permissions, or material assumptions while normalizing the request. This fast path grants no new authority: implementation, credentials, environment inspection, outside-project reads, optional-provider enablement, and publication remain out of scope. It does not itself authorize persistence.

If the precise-request conditions are not met and no valid confirmation already covers the exact brief, wait for explicit user confirmation before starting the full research run. Every other request still waits for explicit user confirmation unless valid confirmation already covers the exact brief. In autonomous or subagent mode, assignments and inferred intent are not user confirmation; assumptions never satisfy unresolved brief decisions or confirmation: do not start the full research run. If any user-owned brief decision remains unresolved, return assumptions, blockers, and open questions to the interactive parent under the shared question-tool fallback and do not label the brief confirmation-ready. If confirmation is required and every user-owned brief decision is settled, return the normalized brief, recorded assumptions, and remaining research questions as a `research-brief-confirmation-required` blocker.

The only autonomous or subagent re-entry conditions are two exact parent bindings: (a) the interactive parent passes back both the exact normalized brief previously returned and an explicit statement that the user confirmed that exact brief without changes, or (b) the parent forwards a user-supplied brief that itself settles every user-owned brief dimension and explicitly authorizes the same read-only research, with the precise-request conditions above satisfied. The worker must not classify its assignment as user confirmation. A digest or artifact reference alone is insufficient. That forwarded confirmation satisfies the confirmation gate for the resumed run; do not request confirmation again. If the brief content changed, the confirmation is missing, or the confirmation is not explicitly bound to that brief, recompute the affected frontier and return the appropriate unresolved-decision or `research-brief-confirmation-required` blocker instead of starting research.

Before reopening the interview, reuse valid confirmation for the exact normalized brief. Carry settled decisions forward. A confirmed brief binds only its own scope: a material scope, risk, or tradeoff change invalidates confirmation for the affected content. On such a change, recompute the affected frontier and ask only the newly unresolved decisions; do not repeat unchanged settled questions. Bind any resulting confirmation to the updated exact brief. New findings alone do not reopen settled brief decisions.

Before confirmation, bounded read-only fact-finding is the only permitted work: do not persist research, present a saved result, append the session, mutate a plan, or create canonical OpenSpec artifacts. Do not create a separate interview, design-tree, decision-log, or research-brief file; the confirmed brief stays conversation context until an already authorized research write records it under `## Confirmed brief`. Use the question mechanism and autonomous/subagent fallback defined in `skills/shared/QUESTION-TOOL.md` and the Codex Runtime section below.

Only when research persistence is already authorized, include one minimal scrubbed `## Confirmed brief` section inside the run's existing research output: the resolved `paths.research` file in regular mode, or the selected valid marked bundle's `RESEARCH.md` in ultra mode, never in INDEX.md. Keep it outside the Active Summary. Record it as part of the ordinary research write or update, not a separate confirmation write, file, or worker; preserve the upstream coherence and bundle gate ordering below.

The section carries the exact normalized five-dimension brief, its read-only authority boundary, confirmation origin (explicit user confirmation or precise user request), safe project-relative research source/profile binding, and `Digest: sha256=<lowercase hex>`. The digest input is the UTF-8 normalized brief body including its authority boundary, preserving line order, excluding markdown comments, excluding the section heading, the `Digest:` line, and any timestamp metadata, normalizing line endings to LF, removing trailing spaces from each line, and ending the digest input with exactly one newline. The digest binds the brief, not changing research findings.

The recorded brief must not contain credentials, tokens, raw provider output, private data, or private absolute paths. It must also not contain metadata-shaped lines that downstream research parsing reserves: any line starting with `Status:`, `Updated:`, or `Digest:`, any `aif:` marker comment, or any additional `## Confirmed brief` heading. Rewrite such content as plain prose inside the brief; a brief that cannot be expressed without reserved metadata lines is not persisted. A hash of sensitive content is not scrubbing. If removing sensitive or reserved content materially changes the brief, do not persist a redacted body under the old confirmation: keep it in conversation context or obtain confirmation for the new exact safe brief.

A later `/aif-explore` run treats this record as forwarded confirmation for the same scope only when its digest matches the newly derived normalized brief and the exact body, read-only authority, source/profile binding, and known confirmation provenance also match. A heading or valid digest alone creates no authority. Missing or uncertain provenance, changed content, or a digest mismatch returns to the appropriate unresolved-decision or confirmation-required flow. Any other content inherits no confirmation authority. Parent-forwarded exact binding remains required for autonomous/subagent re-entry.

### Upstream Research Coherence Gate pass-through

The AIFHub prepend owns only mode, version, path, and write boundaries. Its pass-through runs after every permitted persisted regular or ultra research write or update: continue into the upstream AI Factory 2.18.1 `#### Research Coherence Gate (all persisted modes)` before presenting the saved result or appending the current session. A successful AIFHub write is not a coherence verdict and is not completion of upstream `/aif-explore`.

- Fresh-context `Task` delegation is optional. When that capability is unavailable or fails, use the mandatory direct read-only fallback with the same upstream criteria.
- AIFHub must never skip, delay, replace, or copy the upstream gate implementation. Do not create a second coherence validator or treat this prepend as the gate owner.
- Preserve this exact ordering for regular: persisted write -> upstream Research Coherence Gate -> presentation/session append.
- Preserve this exact ordering for ultra: persisted bundle write -> upstream Research Coherence Gate -> upstream Bundle Integrity Gate -> presentation/session append.
- A non-`PASS` gate outcome stops presentation and session append and remains inside the upstream correction/re-run flow.

Pass-through diagnostics may contain only `asset`, `runtime`, `case`, bounded gate outcome, and whether direct fallback was used. Never log research bodies, quoted mismatch passages, provider output, credentials, raw stdout/stderr, or private absolute paths.

### OpenSpec-native mode

When `.ai-factory/config.yaml` declares `aifhub.tools.openspec: true`, `/aif-explore` is research-oriented and must not create canonical OpenSpec change artifacts.

Use shared vocabulary consistently: `OpenSpec-native mode`, `canonical OpenSpec change`, `active change`, `change-id`, `base specs`, `delta specs`, `generated rules`, `runtime state`, `QA evidence`, and `legacy AI Factory-only mode`.

Before selecting or changing a capability, inventory accepted specs under `openspec/specs/**`, separately from active changes. Reuse each existing capability's exact path, including nested directories. Read relevant accepted specs in full, including scenarios, before deciding what is covered. If using the optional CLI, `openspec list --specs` inventories specs; plain `openspec list` inventories changes and cannot substitute for it. Filesystem discovery remains sufficient when the CLI is unavailable; this does not add Store selection or initialization authority.

Allowed read context:

- directly relevant in-repository source, tests, docs, package/manifest files, and bounded local Git branch/revision metadata
- safe resolved configured project context and rules artifacts, including custom locations represented by the defaults below
- `.ai-factory/config.yaml`
- `.ai-factory/DESCRIPTION.md`
- `.ai-factory/ARCHITECTURE.md`
- `.ai-factory/RESEARCH.md`
- `openspec/specs/**`
- `openspec/changes/<change-id>/**`
- `.ai-factory/state/<change-id>/`
- optional reviewed Graphify outputs such as `graphify-out/GRAPH_REPORT.md`, `graphify-out/graph.json`, `.ai-factory/references/graphify/GRAPH_REPORT.md`, and `.ai-factory/state/<change-id>/graphify/GRAPH_REPORT.md`
- optional reviewed Context7 notes under `.ai-factory/references/context7/` and `.ai-factory/state/<change-id>/context7/`

Enabled optional tool use:

- Before using any optional provider, call the installed wrapper when available: `ai-factory aifhub-memory-tools select --from-project --command aif-explore --json`.
- The wrapper reads `utilities.context_tools.enabled`, compatibility utility flags, local `recommendation-metadata.yaml`, command-specific `tool_permissions`, and project/task signals.
- Use only entries returned in `selected_tools`. For each selected entry, follow its `tool_id`, `permission`, `execution`, `forbidden_operations`, `protected_artifacts`, read scope, purge path, and privacy caveat.
- Do not use tools that are absent from `selected_tools`, including tools listed in `not_selected_tools`, tools missing from config, tools forbidden for `/aif-explore`, or tools whose execution guidance is unavailable.
- If no optional provider is selected, continue with the rg baseline and source/OpenSpec evidence.
- Optional provider output is supporting context only; conclusions must remain grounded in source files, canonical OpenSpec artifacts, generated rules, runtime state, QA evidence, or other direct repository evidence.

Optional Graphify context:

- This provider-specific boundary applies only when `selected_tools` includes Graphify, or when reading already reviewed Graphify output that exists in an allowed context path.
- Graphify is optional supporting context for large repository architecture/relation discovery.
- `/aif-explore` may recommend that the user run Graphify manually outside AIFHub command ownership only when the selection output allows Graphify, but it must not install `graphifyy`, run `graphify`, add Graphify dependencies, or start/register Graphify MCP automatically.
- Missing Graphify or missing `graphify-out/GRAPH_REPORT.md` is degraded context, not an exploration failure.
- When existing Graphify output is available, treat extracted, inferred, ambiguous, or confidence-labeled relationships as hypotheses for direct repository inspection.
- Research conclusions must remain grounded in source files, canonical OpenSpec artifacts, generated rules, runtime state, QA evidence, or other direct repository evidence.
- Project-wide reviewed Graphify copies belong under `.ai-factory/references/graphify/`; change-scoped reviewed copies belong under `.ai-factory/state/<change-id>/graphify/`.
- Do not store Graphify generated files such as `GRAPH_REPORT.md`, `graph.json`, or `graph.html` under `openspec/changes/<change-id>/`, `openspec/specs/`, `.ai-factory/rules/generated/`, or `.ai-factory/qa/<change-id>/`.
- Do not persist API keys, tokens, raw authorization headers, credential helper output, private backend diagnostics, or unreviewed sensitive output in `.ai-factory/`, `openspec/`, docs, runtime state, QA evidence, generated rules, or Graphify reference copies.

Optional Context7 context:

- This provider-specific boundary applies only when `selected_tools` includes Context7, or when reading already reviewed Context7 notes that exist in an allowed context path.
- Context7 is optional supporting documentation context for current library/API docs.
- `/aif-explore` may recommend that the user run Context7 manually outside AIFHub command ownership only when the selection output allows Context7, with commands such as `npx ctx7 library <name> <query>` and `npx ctx7 docs <libraryId> <query>`, or user-installed equivalents `ctx7 library <name> <query>` and `ctx7 docs <libraryId> <query>`.
- Missing Context7, missing Node.js runtime support, missing provider access, or missing reviewed notes is degraded context, not an exploration failure.
- If the user already configured Context7 MCP, available tools may include `resolve-library-id` plus a docs retrieval tool named `get-library-docs` or `query-docs`; use them only as optional read-only documentation context.
- Do not install `ctx7` or `@upstash/context7-mcp`, run `ctx7`, run `ctx7 setup`, add Context7 dependencies or manifest entries, add Context7 MCP templates to `extension.json`, mutate `.mcp.json`, `.cursor/mcp.json`, `.opencode.json`, agent rules, or agent skills, or start/register Context7 MCP automatically.
- Treat Context7 output as supporting context only; research conclusions must remain source-grounded in source files, canonical OpenSpec artifacts, generated rules, runtime state, QA evidence, or other direct repository evidence.
- Reviewed project-wide Context7 notes belong under `.ai-factory/references/context7/`; reviewed change-scoped Context7 notes belong under `.ai-factory/state/<change-id>/context7/`.
- Do not store raw Context7 output, MCP transcripts, API responses, setup output, or generated provider configuration under `openspec/changes/<change-id>/`, `openspec/specs/`, `.ai-factory/rules/generated/`, or `.ai-factory/qa/<change-id>/`.
- Do not persist `CONTEXT7_API_KEY`, API keys, tokens, raw authorization headers, credential helper output, private provider diagnostics, private backend diagnostics, or unreviewed sensitive output in `.ai-factory/`, `openspec/`, docs, runtime state, QA evidence, generated rules, or Context7 reference copies.

Canonical OpenSpec change files under an active change are only:

- `openspec/changes/<change-id>/proposal.md`
- `openspec/changes/<change-id>/design.md`
- `openspec/changes/<change-id>/tasks.md`
- `openspec/changes/<change-id>/specs/**/spec.md`

Write boundaries:

- In regular mode, write research output only to the resolved `paths.research` file.
- In explicit stable-2.18 ultra mode, write only the one selected sibling research bundle derived from the parent of `paths.research`; do not also write `paths.research`.
- Do not write exploration output under `.ai-factory/state/<change-id>/` or `.ai-factory/qa/<change-id>/`.
- Do not create non-OpenSpec files under `openspec/changes/<change-id>/`.
- Do not write debug files, summaries, research notes, validation evidence, or runtime-only files under an OpenSpec change folder.

Response and next-step guidance:

- Report where research was written in the normal response.
- Distinguish research output from canonical OpenSpec artifacts.
- For ultra, report the safe bundle path and created artifact names without copying research content.
- If generated rules or QA evidence were inspected, name those paths in the normal response.
- Suggest `/aif-plan full "<request>"` for new work that needs canonical change artifacts.
- Suggest `/aif-improve <change-id>` for refining an existing OpenSpec-native change.
- Suggest `/aif-implement <change-id>` only after an OpenSpec-native plan is ready for execution.
- Do not suggest deprecated `*-plus` aliases.
- Do not install OpenSpec skills or slash commands.

### Legacy AI Factory-only mode

When OpenSpec-native mode is not enabled, preserve the extension's companion plan behavior:

- Treat `.ai-factory/plans/<plan-id>.md` and `.ai-factory/plans/<plan-id>/` as one active plan pair.
- If `@path` points to the plan file, the plan folder, or one of its local artifacts (`task.md`, `context.md`, `rules.md`, `verify.md`, `status.yaml`, `explore.md`), resolve the whole pair before continuing.
- Apply the shared regular/ultra research routing above before any plan-pair normalization. Regular mode writes only resolved `paths.research`; explicit stable-2.18 ultra writes only the one marked sibling bundle and never writes plan companions.
- Do not treat `DESCRIPTION.md`, `ARCHITECTURE.md`, `ROADMAP.md`, or `RULES.md` as writable from explore mode in this extension workflow.
- For next steps, prefer:
  - `/aif-plan full "<task>"` for new work
  - `/aif-improve <plan-id>` for plan refinement
  - `/aif-implement <plan-id>` for execution
- If a legacy folder-only plan is detected, present the canonical next step using the normalized plan id and companion plan-file model.

### Codex Runtime

When running in Codex app/CLI:

- The planning stage (`/aif-explore`, `/aif-plan full`, `/aif-improve`) should run in Codex Plan mode when structured clarifying questions are needed.
- This skill may recommend Plan mode, but it does not attempt or promise to switch the Codex session mode. The user controls the mode.
- In Codex Plan mode, use `request_user_input` only for 1-3 short questions.
- In Codex Default mode, if a question is needed, ask it as plain text in the assistant message. Do not use `question(...)`, `questionnaire(...)`, or `request_user_input`.
- If another CLI or IDE runtime exposes a planning mode, use that available planning-mode mechanism for structured planning questions; do not fabricate unavailable tools or client actions.
- In autonomous or subagent mode, do not ask interactive questions. Record assumptions and return blockers/open questions to the parent.
- See `skills/shared/QUESTION-TOOL.md` for the full runtime question format mapping.
<!-- aif-ext:aifhub-extension:aif-explore:prepend:end -->

Enter explore mode. Think deeply. Visualize freely. Follow the conversation wherever it goes.

**IMPORTANT: Explore mode is for thinking, not implementing.** You may read files, search code, and investigate the codebase, but you must NEVER implement features or modify project code. If the user asks to implement something, remind them to exit explore mode first (e.g., start with `$aif-plan`). Regular mode may write only the resolved research file after the user chooses to save. Explicit ultra mode may write only its selected research bundle; the leading `ultra` token is the user's request to persist that bundle.

---

## Step 0: Load Config

**FIRST:** Read `.ai-factory/config.yaml` if it exists to resolve:
- **Paths:** `paths.description`, `paths.architecture`, `paths.rules_file`, `paths.roadmap`, `paths.research`, `paths.plan`, `paths.plans`, and `paths.rules`
  - Derive `research_bundles_dir = <parent directory of paths.research>/research/`. This keeps ultra research colocated with a relocated legacy research file without adding a second config key.
- **Language:**
  - `language.ui` for all user-facing responses: prompts, progress updates, explanations, exploration summaries, and next-step guidance
  - `language.artifacts` for generated or persisted exploration artifacts, including the resolved `paths.research`
  - `language.technical_terms` for human-readable technical terminology style in artifacts and summaries
  - If `language.artifacts` is missing, use `language.ui`
  - If both are missing, use `en`
- **Workflow:** `workflow.plan_id_format` (default: `slug`) — used by the optional active-plan-context lookup when explore mode references an existing plan for the current branch.
  Active values: `slug` and `sequential`. Active-plan context may be a root
  full-plan file or direct child ultra `index.md`; numbered lookup covers both.
  `timestamp` and `uuid` are **reserved values** and currently behave like `slug`.
  Treat any unknown value as `slug`.

If config.yaml doesn't exist, use defaults:
- Paths: `.ai-factory/` for all artifacts
- `ui_language`: `en`
- `artifact_language`: `en`
- `technical_terms_policy`: `keep`
- `workflow.plan_id_format`: `slug`

Store:
- `ui_language = language.ui || "en"`
- `artifact_language = language.artifacts || language.ui || "en"`
- `technical_terms_policy = language.technical_terms || "keep"`

If `technical_terms_policy` is not one of `keep`, `translate`, or `mixed`, treat it as `keep`. Legacy values such as `english` also behave like `keep`.

All user-facing responses from `$aif-explore` MUST be written in `ui_language`.

Persisted exploration artifacts under `paths.research` MUST be written in `artifact_language`.

Ultra bundle artifacts under `research_bundles_dir` MUST also be written in `artifact_language`. Preserve filenames, relative links, Mermaid syntax, evidence paths, traceability IDs, the compatibility headings `## Artifact Index`, `## Active Summary (input for $aif-plan)`, and `## Sessions`, metadata keys `Topic:`, `Slug:`, `Updated:`, `Status:`, status values `active`, `paused`, `superseded`, `proposed`, and `accepted`, the exact markers `<!-- aif:active-summary:start -->`, `<!-- aif:active-summary:end -->`, `<!-- aif:sessions:start -->`, `<!-- aif:sessions:end -->`, and `<!-- aif:research-mode:ultra -->` unchanged. The topic and other human-readable values still use `artifact_language`; the English slug is a stable path identifier.

Apply `technical_terms_policy` while writing summaries and persisted artifacts:
- `keep` - keep commands, paths, identifiers, config keys, API names, package names, branch names, code terms, and raw error messages unchanged
- `translate` - translate human-readable technical terms where a natural target-language term exists
- `mixed` - translate ordinary prose terms while keeping code, infrastructure, and ecosystem terms unchanged

**This is a stance, not a scripted interview.** Regular mode has no mandatory output. Ultra adds a persistence contract after the investigation has enough evidence; it does not turn the conversation into a fixed questionnaire.

---

## Artifact Ownership

- Primary ownership in regular explore mode: the resolved research path (default: `.ai-factory/RESEARCH.md`) only.
- Primary ownership in explicit ultra mode: one marked direct child bundle under `research_bundles_dir` only. Do not also update the legacy research file.
- All other context artifacts (`paths.description`, `paths.architecture`, `paths.roadmap`, `paths.rules_file`, plan files) are read-only in this mode.
- If a discovery should affect another artifact, capture it in the selected `RESEARCH.md` now and route follow-up to the owner command later.

---

## Mode Selection

Parse only a leading `ultra` token as mode syntax:

- `$aif-explore ultra <topic>` -> ultra mode; strip the token and preserve the remaining topic text.
- `$aif-explore <topic>` or no arguments -> regular mode; behavior and save prompt remain unchanged.
- The word `ultra` elsewhere in a topic is ordinary content.

Ultra is strictly opt-in. Never recommend, infer, or auto-select it because a topic looks complex. In ultra mode, read `references/ULTRA-RESEARCH-FORMAT.md` completely before choosing a slug or writing files.

The explicit ultra invocation is permission to persist the selected bundle, so do not ask the regular "save research?" question. Explore and gather enough evidence first, then create or update the bundle. If no meaningful topic is available, ask for one before writing; never create `research/analysis/`, `research/research/`, or a date-only directory.

---

## The Stance

- **Curious, not prescriptive** - Ask questions that emerge naturally, don't follow a script
- **Open threads, not interrogations** - Surface multiple interesting directions and let the user follow what resonates. Don't funnel them through a single path of questions.
- **Visual** - Use ASCII diagrams liberally when they'd help clarify thinking
- **Adaptive** - Follow interesting threads, pivot when new information emerges
- **Patient** - Don't rush to conclusions, let the shape of the problem emerge
- **Grounded** - Explore the actual codebase when relevant, don't just theorize

---

## What You Might Do

Depending on what the user brings, you might:

**Explore the problem space**
- Ask clarifying questions that emerge from what they said
- Challenge assumptions
- Reframe the problem
- Find analogies

**Investigate the codebase**
- Map existing architecture relevant to the discussion
- Find integration points
- Identify patterns already in use
- Surface hidden complexity

**Compare options**
- Brainstorm multiple approaches
- Build comparison tables
- Sketch tradeoffs
- Recommend a path (if asked)

**Visualize**
```
+-----------------------------------------+
|     Use ASCII diagrams liberally        |
+-----------------------------------------+
|                                         |
|   +--------+         +--------+        |
|   | State  |-------->| State  |        |
|   |   A    |         |   B    |        |
|   +--------+         +--------+        |
|                                         |
|   System diagrams, state machines,      |
|   data flows, architecture sketches,    |
|   dependency graphs, comparison tables  |
|                                         |
+-----------------------------------------+
```

**Surface risks and unknowns**
- Identify what could go wrong
- Find gaps in understanding
- Suggest spikes or investigations

---

## AI Factory Context

You have access to AI Factory's project context. Use it naturally, don't force it.

**Read `.ai-factory/skill-context/aif-explore/SKILL.md`** — MANDATORY if the file exists.

This file contains project-specific rules accumulated by `$aif-evolve` from patches,
codebase conventions, and tech-stack analysis. These rules are tailored to the current project.

**How to apply skill-context rules:**
- Treat them as **project-level overrides** for this skill's general instructions
- When a skill-context rule conflicts with a general rule written in this SKILL.md,
  **the skill-context rule wins** (more specific context takes priority — same principle as nested CLAUDE.md files)
- When there is no conflict, apply both: general rules from SKILL.md + project rules from skill-context
- Do NOT ignore skill-context rules even if they seem to contradict this skill's defaults —
  they exist because the project's experience proved the default insufficient
- **CRITICAL:** skill-context rules apply to ALL outputs of this skill — including exploration
  summaries, diagrams, and permitted research file/bundle updates. If a skill-context
  rule says "exploration MUST cover X" or "summary MUST include Y" — you MUST comply. Producing
  output that ignores skill-context rules is a bug.

**Enforcement:** After generating any output artifact, verify it against all skill-context rules.
If any rule is violated — fix the output before presenting it to the user.

### Check for context

At the start, read these files if present:

- `.ai-factory/DESCRIPTION.md` — project description, tech stack, constraints
- `.ai-factory/ARCHITECTURE.md` — architecture decisions, folder structure
- the resolved RULES.md path – project conventions and rules
- the resolved RESEARCH.md path – persisted exploration notes (so you can `/clear` and still keep context)
- direct child `research_bundles_dir/*/INDEX.md` files containing `<!-- aif:research-mode:ultra -->` exactly once and linking their `RESEARCH.md` from `## Artifact Index` – ultra research manifests. Read only their index metadata at startup; read the linked `RESEARCH.md` and optional artifacts only when the bundle is explicitly referenced or clearly relevant to the current topic. Ignore invalid or unmarked directories.
- the resolved fast plan path – active fast plan (if any)
- `<configured plans dir>/<branch_stem>.md` or
  `<configured plans dir>/<branch_stem>/index.md` – active full/ultra plans.
  Compute `branch_stem` as `git branch --show-current` with every `/` replaced by `-`
  (for example `feature/user-auth` → `feature-user-auth`).
  When `workflow.plan_id_format = sequential`, glob both the numbered full file
  and numbered ultra `index.md`; Read each directory candidate and retain it
  only when it contains exactly one `<!-- aif:plan-mode:ultra -->`, then pick the
  highest valid prefix. Fall back to the unnumbered entrypoint/full file only
  after applying the same marker check to the directory.
- the resolved ROADMAP.md path – strategic milestones (if any)

This tells you:
- What the project is about
- What conventions to follow
- If there's active work in progress
- Any prior exploration context worth carrying into planning

### Input handling

The argument after `$aif-explore` can be:
- An explicit ultra research request: `ultra partner order synchronization`
- A vague idea: "real-time collaboration"
- A specific problem: "the auth system is getting unwieldy"
- A plan name: to explore in context of a full `.md` plan or ultra
  `.ai-factory/plans/<name>/index.md` bundle
- A comparison: "postgres vs sqlite for this"
- Nothing: just enter explore mode

### When no plan exists

Think freely. When insights crystallize, you might offer:

- "This feels solid enough to plan. Want me to start `$aif-plan`?"
- Or keep exploring - no pressure to formalize

### When a plan exists

If the user mentions a plan or you detect one is relevant:

1. **Read existing plan for context**
   - the resolved fast plan path (fast mode)
   - `<configured plans dir>/<branch_stem>.md` (full) or
     `<configured plans dir>/<branch_stem>/index.md` (ultra).
     `branch_stem` = `git branch --show-current` with every `/` replaced by `-`
     (so `feature/user-auth` resolves to `feature-user-auth`).
     When `workflow.plan_id_format = sequential`, the file/directory identifier
     has `<NNNN>_`; Read every directory candidate, retain it only when it
     contains exactly one `<!-- aif:plan-mode:ultra -->`, then pick the
     highest-numbered valid artifact across both shapes. If both shapes share
     that prefix, warn and prefer ultra. For the unprefixed fallback, Read
     `<configured plans dir>/<branch_stem>/index.md` before selection and ignore
     it unless it contains exactly one ultra marker.
  - For ultra, read `index.md` first and only the linked phase files relevant
    to the current exploration question; phase files are not independent plans.
    Treat a discovered directory entrypoint as ultra only when it contains
    exactly one `<!-- aif:plan-mode:ultra -->`; unrelated `*/index.md` files are
    not plan context.

2. **Reference it naturally in conversation**
   - "Your plan mentions adding Redis, but we just realized SQLite fits better..."
   - "Task 3 scopes this to premium users, but we're now thinking everyone..."

3. **Offer to capture when decisions are made**

   In regular mode, capture everything in the resolved research path so it survives `/clear`. In ultra mode, capture it in the selected bundle's `RESEARCH.md` and place only justified supporting analysis beside it.
   Later (during planning), you can migrate stabilized decisions into the appropriate context file.

   | Insight Type | Capture Now (Explore) | Later (Optional) |
   |--------------|------------------------|------------------|
   | New requirement | selected `RESEARCH.md` | `paths.description` |
   | Architecture decision | selected `RESEARCH.md`; ADR only when the ultra inclusion gate is met | `paths.architecture` |
   | Project convention | selected `RESEARCH.md` | `paths.rules_file` |
   | Strategic direction | selected `RESEARCH.md` | `paths.roadmap` |
   | Assumption invalidated | selected `RESEARCH.md` | Relevant file |
   | Exploration context (persisted) | selected research artifact | (keep in research) |
   | New task/feature | Run `$aif-plan` | `paths.plan`, a full `paths.plans/<id>.md`, or an ultra `paths.plans/<id>/index.md`; `<id>` may have the sequential `NNNN_` prefix |

   Example offers:
   - "Want me to save this to the resolved research path so you can `/clear` and come back later?"
   - "That's an architecture decision — save it to RESEARCH now and we can migrate it to ARCHITECTURE during planning."

4. **The user decides in regular mode** - Offer and move on. Don't pressure. In ultra, the explicit mode token already requests capture.

### Persist exploration context

#### Research Coherence Gate (all persisted modes)

Run this gate whenever regular or ultra research is saved or updated. Re-read only the durable scope from disk: resolved `RESEARCH.md` in regular mode, or `INDEX.md` plus its Artifact Index links in ultra. Chat history and unstored memory are not evidence.

Before presenting the save or appending its session entry, verify:

1. The Active Summary is understandable without the conversation that produced it.
2. It does not silently contradict durable research; superseded conclusions are explicit.
3. Claims distinguish source evidence from inference and unknowns.
4. Each mismatch quotes verbatim both the affected summary claim and the
   conflicting or qualifying durable passage; a bare assertion is not a finding.

Correct or qualify mismatches, record insufficient evidence in `Open questions`, then re-run. Append a session with a short pass note only after success; an Active-Summary-only regular save runs the gate without adding a session.

Delegate the read-only pass to a fresh-context subagent when supported, giving it only the durable paths and this gate. If unavailable or failed, run it directly; the criteria do not change, and the gate is never skipped or delayed.

#### Ultra mode: adaptive bundle

For explicit ultra mode, follow `references/ULTRA-RESEARCH-FORMAT.md`:

1. Convert the meaningful topic to a concise English lowercase-kebab slug and resolve `<research_bundles_dir>/<slug>/`.
2. If that directory exists, treat it as an existing ultra bundle only when `INDEX.md` has the exact marker. Reuse a semantically matching marked bundle; never overwrite an unmarked collision.
3. Assess the actual evidence against the artifact inclusion matrix before writing. `INDEX.md` and `RESEARCH.md` are mandatory. C4, ADR, and dependency graph files are conditional; a simple topic stays a two-file bundle.
4. Create the bundle directory, write/update `RESEARCH.md` with the legacy Active Summary/Sessions markers, then write/update only the justified supporting artifacts.
5. Write `INDEX.md` last so its Artifact Index and reading order match the files that actually belong to the bundle. Record the concrete reason each optional artifact exists.
6. Run the Research Coherence Gate, then the Bundle Integrity Gate, before appending the current session entry. Do not generate placeholders, orphan files, speculative diagrams, or implementation task checklists.

Any C4/ADR/dependency conclusion that affects plan requirements MUST be summarized in the bundle `RESEARCH.md` Active Summary. `$aif-plan` commits and hashes that summary; it does not silently promote every diagram note into scope.

When ultra research is ready, report the bundle path, list only the artifacts actually created, and suggest `$aif-plan [fast|full|ultra] <same topic>`.

#### Regular mode: optional single-file snapshot (`paths.research`)

If the conversation is crystallizing (you're about to plan, you want to `/clear`, or you want to continue later), offer to save a compact, durable research snapshot.

**Hard rule in regular explore mode:** If the user chooses to save, you may write/edit **only** the resolved research path (and create its parent directory if missing). Do not write or modify any other project files.

Write the saved research content in `artifact_language`. The skeleton below defines structure, not fixed English output. If `artifact_language` is not `en`, translate human-readable headings, labels, notes, and prose before saving, except for the compatibility headings `## Active Summary (input for $aif-plan)` and `## Sessions` and metadata keys `Topic:`, `Updated:`, and `Status:`. Preserve those tokens, the `active` status value, exact `<!-- aif:active-summary:start -->`, `<!-- aif:active-summary:end -->`, `<!-- aif:sessions:start -->`, and `<!-- aif:sessions:end -->` markers, paths, commands, config keys, issue URLs, branch names, code identifiers, package names, and raw error messages unchanged. Values after human-readable fields such as `Topic:` still use `artifact_language`.

Ask:

```
Save these exploration results to the resolved research path so we can /clear and $aif-plan can reuse them?

Options:
1. Yes — update Active Summary + append a new Session (recommended)
2. Yes — update Active Summary only
3. No
```

If user selects (1) or (2):
- Ensure the parent directory of the resolved research path exists (`mkdir -p "$(dirname "<resolved research path>")"`)
- If the resolved research path does not exist, create it with this skeleton, localized to `artifact_language` before saving:

```markdown
# Research

Updated: YYYY-MM-DD HH:MM
Status: active

## Active Summary (input for $aif-plan)
<!-- aif:active-summary:start -->
Topic:
Goal:
Constraints:
Decisions:
Open questions:
Success signals:
Next step:
<!-- aif:active-summary:end -->

## Sessions
<!-- aif:sessions:start -->
<!-- aif:sessions:end -->
```

- Update the `Updated:` timestamp
- Replace only the content inside `aif:active-summary:start/end`, written in `artifact_language`
- Run the Research Coherence Gate against the updated file
- If user selected option (1), append a new session entry just before `<!-- aif:sessions:end -->`:

```markdown
### YYYY-MM-DD HH:MM — <short title>
What changed:
Key notes:
Links (paths):
```

Keep prior sessions verbatim (do not rewrite history).

---

## What You Don't Have To Do

- Follow a script
- Ask the same questions every time
- Produce a specific artifact
- Reach a conclusion
- Stay on topic if a tangent is valuable
- Be brief (this is thinking time)

---

## Handling Different Entry Points

**User brings a vague idea:**
```
User: I'm thinking about adding real-time collaboration

You: Real-time collab is a big space. Let me think about this...

      COLLABORATION SPECTRUM

      Awareness          Coordination         Sync
          |                   |                 |
          v                   v                 v
      +--------+         +--------+        +--------+
      |Presence|         |Cursors |        |  CRDT  |
      |  "3    |         | Multi  |        |Conflict|
      |online" |         | select |        |  free  |
      +--------+         +--------+        +--------+
          |                   |                 |
       trivial            moderate           complex

      Where's your head at?
```

**User brings a specific problem:**
```
User: The auth system is a mess

You: [reads codebase]

     +---------------------------------------------+
     |              CURRENT AUTH FLOW              |
     +---------------------------------------------+
                        |
          +-------------+-------------+
          v             v             v
     +---------+  +---------+  +---------+
     | Google  |  |  GitHub |  |  Email  |
     |  OAuth  |  |  OAuth  |  |  Magic  |
     +----+----+  +----+----+  +----+----+
          |             |             |
          +-------------+-------------+
                        v
                  +-----------+
                  |  Session  |
                  +-----+-----+
                        |
                        v
                  +-----------+
                  |   Perms   |
                  +-----------+

     I see three tangles. Which one's burning?
```

**User is stuck mid-implementation:**
```
User: $aif-explore add-auth-system
      The OAuth integration is more complex than expected

You: [reads plan from .ai-factory/plans/add-auth-system.md]

     You're on task 4: "Implement OAuth flow"

     Let me trace what's involved...

     [draws diagram, explores options, suggests paths]

     Want to update the plan to reflect this?
     Or add a spike task to investigate?
```

**User wants to compare options:**
```
User: Should we use Postgres or SQLite?

You: Generic answer is boring. What's the context?

User: A CLI tool that tracks local dev environments

You: That changes everything.

     +---------------------------------------------+
     |         CLI TOOL DATA STORAGE               |
     +---------------------------------------------+

     Key constraints:
     * No daemon running
     * Must work offline
     * Single user

                  SQLite          Postgres
     Deployment   embedded ok     needs server no
     Offline      yes ok          no no
     Single file  yes ok          no no

     SQLite. Not even close.

     Unless... is there a sync component?
```

---

## Ending Discovery

There's no required ending. Discovery might:

- **Flow into action**: "Ready to plan? Run `$aif-plan`"
- **Result in research capture**: "Updated the selected RESEARCH.md with these decisions"
- **Just provide clarity**: User has what they need, moves on
- **Continue later**: "We can pick this up anytime"

When it feels like things are crystallizing, you might summarize:

```
## What We Figured Out

**The problem**: [crystallized understanding]

**The approach**: [if one emerged]

**Open questions**: [if any remain]

**Next steps** (if ready):
- Create a plan: $aif-plan [fast|full|ultra] <description>
- Keep exploring: just keep talking
```

But this summary is optional. Sometimes the thinking IS the value.

---

## Guardrails

- **Don't implement** - Never write code or implement features. Updating AI Factory context files is fine, writing application code is not.
- **Don't fake understanding** - If something is unclear, dig deeper
- **Don't rush** - Discovery is thinking time, not task time
- **Don't force structure** - Let patterns emerge naturally
- **Don't auto-capture in regular mode** - Offer to save insights, don't just do it. The explicit `ultra` token already requests bundle persistence.
- **Do visualize** - A good diagram is worth many paragraphs
- **Do explore the codebase** - Ground discussions in reality
- **Do question assumptions** - Including the user's and your own
