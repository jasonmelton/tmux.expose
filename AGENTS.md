# Branch Context

`.branch-context/` holds the workflow and durable artifacts for this branch.

## Actors and ownership

### Developer

The developer supplies direction and authorization.
The developer writes `00_DEVNOTE.md` and `> NOTE:` annotations in artifacts.

### MAIN

MAIN sequences stages, dispatches sub-agents, relays decisions, and reports gate results live.
After each stage return, MAIN runs a bounded acceptance check on the returned
artifact. MAIN may make mechanical between-stage edits to a numbered artifact
(structure, metadata, `Status`, ID format); substantive stage content originates
with the owning role. After a stage is accepted, MAIN's orchestration contract
grants bounded direct-change authority over that stage's outputs: each change is
logged in MAIN's execution log, a change that reduces what a recognized test
checks requires an explicit developer instruction, and Analyzer, Review, and
Closeout outputs remain role-authored. Where this file routes a change through an
owning role, that post-acceptance authority may apply instead.

### sub-agent

The sub-agent named for each stage loads its role definition, which is authoritative
for everything in it. Every role definition uses the same section structure: `Task`
(mission, ownership, operation semantics), `Inputs`, `Procedure`, `Points`, `Gate`
(only roles with gate duties), `Artifact` (all but `socrates`), and `Summary`. MAIN
defers to the role definition for all content under those sections.

Each stage sub-agent writes its workflow table output, including its `Status:` and `> RESOLVED:` / `> BLOCKER:` replies.

A sub-agent is not MAIN. It executes one role for the provider session that
dispatched it, does not invoke another provider or agent CLI, and does not dispatch
another role; the only spawning it may use is its role's documented native subagent
interface. Its return is the role summary alone: a sub-agent does not emit
`# Workflow Options`, workflow adjudication, or other MAIN-voiced output. The role
definition, the dispatch inputs, and the listed artifacts govern its run over
provider memory, skills, and prior-session directives.

`branch-socrates` owns the read-only, provider-native challenge return defined by
the `socrates` role.

## The Workflow

The workflow has seven stages. Each stage is executed by the named agent, which loads
the named role and owns the listed artifact.

RESEARCH → TEST → PLAN → IMPLEMENT → ANALYZE → REVIEW → CLOSEOUT

| Stage       | Agent                | Role          | Artifact          | Tier | Effort   |
| ----------- | -------------------- | ------------- | ----------------- | ---- | -------- |
| `RESEARCH`  | `branch-researcher`  | `researcher`  | `01_RESEARCH.md`  | top  | `xhigh`  |
| `TEST`      | `branch-tester`      | `tester`      | `02_TEST.md`      | mid  | `medium` |
| `PLAN`      | `branch-planner`     | `planner`     | `03_PLAN.md`      | top  | `high`   |
| `IMPLEMENT` | `branch-implementer` | `implementer` | `04_IMPLEMENT.md` | mid  | `high`   |
| `ANALYZE`   | `branch-analyzer`    | `analyzer`    | `05_ANALYSIS.md`  | mid  | `medium` |
| `REVIEW`    | `branch-reviewer`    | `reviewer`    | `06_REVIEW.md`    | top  | `xhigh`  |
| `CLOSEOUT`  | `branch-closeout`    | `closeout`    | `07_CLOSEOUT.md`  | mid  | `medium` |

`Tier` and `Effort` are the provider-neutral model-selection contract. A top-tier
stage mints requirements, carries design, or judges others' work; a mid-tier stage
executes bounded or procedural work. The non-stage `branch-socrates` runs top tier
at `high` effort.

`.project-context/` (optional) contains project-specific input. It's good
practice for each stage to independently verify if its corresponding
`.project-context/<STAGE>.md` file exists (e.g., `05_ANALYSIS.md` for Analyzer)
and read it, even if omitted from the dispatch prompt.
`.branch-context/` contains durable output.

Project inputs specialize the general Branch Context workflow and take precedence
over role defaults. If project input conflicts with other active project instructions,
the stage owner records a blocking `QST` so the project input can be corrected. Do not
execute the conflicting instruction or bypass the project input.

## Workflow operations

The normal workflow proceeds sequentially from RESEARCH through CLOSEOUT.

`RUN <stage>` starts or continues a stage. `REWORK <stage>` applies developer
direction. Review also accepts `CONFIRM <selected Review IDs>` for its bounded
post-fix check.

`DRAFT ISSUE` loads the `draft-issue` skill independently of the seven stages. MAIN
may provide role-owned artifacts as read-only evidence and creates no numbered
artifact. The existing issue authority rules still apply.

Each stage begins `DRAFT`, may receive zero or more `REWORK`s, and becomes `COMPLETED`
when finished. An invoked stage with no applicable work completes and records that
result in its artifact. `COMPLETED` means the role finished its work; reported
findings still govern the next Workflow Options.

MAIN normally starts the next applicable stage after completion. When current
evidence establishes that a stage has no work, MAIN skips its invocation and creates
no artifact. If only the role can determine applicability, its initial RUN completes
the no-work artifact without a no-op REWORK. The developer may direct another stage
or command at any time.

Automatic continuation stops after a standalone REWORK or a developer-authorized
gate-only command; a `REWORK` that MAIN's stage-acceptance check triggers is
exempt and continuation resumes after it. MAIN reports the completed artifact and offers the next operation.
Only an Implement REWORK that was explicitly selected from current Review findings
authorizes one automatic, effort-free `CONFIRM` for those IDs. It does not authorize
a full Review or a second confirmation.

Stage gates are scoped: at most a compile of touched targets or the suites the
branch added or modified. Project inputs supply scoped gate commands to Implement.
The branch has at most one full-suite run: the final gate. Closeout runs it on the
delivered tree when optional `.project-context/07_CLOSEOUT.md` or explicit developer
direction supplies the command. Without either source, Closeout records the final
gate as `NOT APPLICABLE` and continues; absence of the optional project input is not
a blocker. A later-stage change does not make a scoped stage gate stale and triggers
no refresh.

Rework stays scoped. A `NIT` or `BLK` fix takes at most a compile or a
scoped-suite run. The final gate runs inside Closeout after all fixes, so it is
generally fresh. When the final gate fails, Closeout records the result and stops
before delivery. MAIN routes the fix through Implement, then reports the result and
offers Closeout without dispatching it. The developer can select `PROCEED` to deliver
without a green final gate.

Each assigned gate authorizes one execution. Implement runs scoped gates and
Closeout runs the final gate, each under its role's `Gate` rules. When a role
records an `UNCONFIRMED` execution, MAIN reports that record and waits for
express developer authorization before assigning another execution.

Every role-executed gate records one numbered `### Gate receipt <n>` under the
owning artifact's `Evidence` with these exact fields:

- `Owner: TEST | IMPLEMENT | ANALYZE | CLOSEOUT`
- `Command: <exact command>`
- `Authorization: <project-input path or quoted developer instruction>`
- `Elapsed: <NmNNs>`
- `Result: PASS | FAIL | UNCONFIRMED`
- `Exit: <integer | unknown>`
- `Log: <unique complete-log path>`

The role captures elapsed time. If the log output or wrapper stdout clearly
demonstrates success, record `PASS` even if the numeric exit status is missing
or unknown. Include the log path when possible to help with debugging. Only
Closeout may own a final or full-suite gate receipt. MAIN copies each new
receipt once into `99_MAIN.md`.

### Developer operations

- `BREAKDOWN <selected current point IDs>` is read-only live
  discussion. MAIN verifies every selected ID is current, reads the owning records
  and relevant evidence, explains each presented point in plain English before its
  options, and drafts viable options with pros and cons and a recommendation. The
  lead states what is broken or uncertain, the concrete consequence or outcome at
  stake, and why developer judgment is needed. MAIN invokes one `CHALLENGE` before
  relaying that draft. An invalid or stale ID stops before `CHALLENGE`. A multi-ID
  BREAKDOWN challenges the combined draft once. With `1-by-1`, MAIN challenges and
  presents one point, then waits. Existing points, authorizations, and gate state
  remain unchanged. MAIN persists decisions through the owning sub-agent.
- `REWORK <stage> <point ID>` directs the named stage to address that point.
- `REWORK <stage>+<stage> <point IDs>` dispatches the named stages concurrently
  for those points.
- `PROCEED` authorizes MAIN to continue to the next stage despite the reported
  workflow-evidence condition. During Closeout, if unexecuted delivery actions remain,
  MAIN dispatches `REWORK closeout`.
- `REBASE` authorizes MAIN to invoke `branch-closeout`. After success, MAIN reports
  affected evidence and offers narrow refresh routes; it requires re-entry only for
  a concrete `BLK`.
- `RESTORE REBASE` authorizes MAIN to invoke `branch-closeout` only after
  `branch-closeout` offers that recovery for a failed rebase.

BREAKDOWN decision persistence is idempotent. An option that preserves an
already-settled decision is labeled `KEEP`, quotes the durable decision, and states
that selecting it writes nothing; a material change is a distinct `AMEND` option
naming its exact delta. After selection, MAIN compares the selection's meaning with
the durable decision: a semantically equivalent selection reports
`UNCHANGED — decision already durable` with no owner dispatch, while a change to
scope, ordering, authority, acceptance criteria, required evidence, or non-goals
dispatches the owning role with that exact delta. When reaffirmation versus
amendment is unclear, MAIN asks one direct clarification before dispatch.
Workflow-derived consequences and explanatory wording are explanation outside the
decision and persist nothing.

A required directly affected checked-in documentation or workflow-guidance
inconsistency is a Review `BLK`; MAIN automatically fixes it directly when bounded,
or invokes `branch-implementer` with `REWORK` and that exact Review ID. Optional
documentation polish is a developer-selected `NIT`. After the selected rework or
MAIN's logged direct fix completes, MAIN invokes `branch-reviewer` once with
`CONFIRM` for the selected Review IDs. `CONFIRM`
is direct, has no effort, cannot create a `NIT` or `QST`, and can add only a concrete
fix-induced `BLK`. An invasive delta stops for explicit developer authorization
before any full Review.

MAIN preserves targeted recovery and finding scope. A completed Review normally
proceeds to Closeout from `Approve`; `Changes requested` exposes applicable discussion
and rework routes. Explicit `PROCEED` remains available. After Review, MAIN exposes
every useful `REWORK Implement` selection.

Review artifact `Status` reports execution lifecycle, not judgment. MAIN
reports it separately from the Review `Verdict`; `COMPLETED` does not imply `Approve`.

## Numbered artifact shape

Each numbered artifact starts with role-required identity metadata and `Status`, then
uses these top-level sections:

- `Purpose` — the artifact's objective and scope.
- `Summary` — its current outcome and lifecycle result.
- `Points` — one subsection per current role-owned point, with its stable ID,
  supporting evidence, disposition when applicable, and role-defined hierarchy.
- `Evidence` — role-required execution records, gates, or analysis that
  does not belong to one point.

Write `none` when a required section has no entries. Role definitions specify their
metadata, point content, evidence, and `DRAFT` or `COMPLETED` conditions.
`branch-socrates` owns no numbered artifact and does not use this shape.

Write a numbered artifact with your native file-write tool. Do not edit it through a
shell command, a redirection, or an in-place editor. Do not create a patch, backup,
or other scratch file in the repository; use a path outside the repository when you
need scratch space. A stray file is a containment breach that MAIN must clear before
the workflow continues, even when the artifact itself is correct.

## Artifact-first post-phase summaries

Each invoked sub-agent finishes its authorized work and returns one compact summary.
`branch-researcher`, `branch-tester`, `branch-planner`, `branch-implementer`,
`branch-analyzer`, `branch-reviewer`, and `branch-closeout` finalize and read back their
numbered artifact before deriving the summary. `branch-socrates` returns its ephemeral
challenge result directly. The invoked sub-agent returns an explicit failure when a
required write, read-back, or record fails.

Each summary starts with `# <role> summary`, followed by `Artifact:` and `Status:`.
Artifact status is `DRAFT` or `COMPLETED`. Roles then return populated artifact-derived
groups, with questions and actionable finding groups last. For each group with
points, return every current point as
`- <ID> — Surface: <production|test|mixed|non-code> — <artifact-derived summary>`.
Classify the affected surface, not the location of supporting evidence:
`production` covers production source or runtime behavior; `test` covers test source,
fixtures, or test-only configuration; `mixed` covers both; and `non-code` covers
documentation, workflow guidance, process, or no concrete code surface. Counts
accompany point summaries when useful. Preserve exact commands, results, IDs,
status, and unresolved `QUESTION:` text; detailed evidence remains in the artifact.
A role summary contains no `# Workflow Options` block and no adjudication addressed
to MAIN.

MAIN reads the finalized artifact to confirm the role return, then relays the summary
and presents next steps. Show the artifact lifecycle status verbatim. When MAIN knows
the artifact no longer covers current work, preserve the displayed status and include
an applicable refresh operation with its reason under `# Workflow Options`.

MAIN ends every phase handoff with `# Workflow Options`. Include currently useful
Developer Operations. `BREAKDOWN` adds MAIN's detailed analysis, options, pros and
cons, and recommendation for selected current points.

## Points

The developer, MAIN, and the stage sub-agents use stable sequential prefixed IDs for
cross-session discussion. `BLK` identifies blockers, `NIT` identifies non-blocking
findings, and `QST` identifies questions.

| Role         | Point type                  | Prefix |
| ------------ | --------------------------- | ------ |
| `researcher` | Requirement                 | `REQ`  |
| `researcher` | Hazard                      | `HAZ`  |
| `researcher` | Bug / defect found          | `BUG`  |
| `researcher` | Cosmetic / design-only item | `COS`  |
| `tester`     | Written logical test        | `TST`  |
| `tester`     | Rejected test consideration | `NOT`  |
| `planner`    | Phase                       | `PHS`  |
| `planner`    | Task                        | `TSK`  |

`REQ` is the only issue-qualified series and only `branch-researcher` mints it. Other
series are local to their artifact. Cite local IDs bare and qualify cross-artifact IDs
with their owner, for example `RESEARCH:QST3`, `TEST:TST2`, `IMPLEMENT:BLK1`, or
`REVIEW:NIT2`; complete `REQ768-17` IDs need no qualifier. Each ID retains its
original point for the life of its artifact. MAIN accepts `BREAKDOWN` and rework
selections only for current IDs.

## Language discipline

The developer, MAIN, and the sub-agents apply these rules to Branch Context
responses and artifacts.

- Match the response length to the work.
- Keep caveats and disclaimers short. Give most of the response to the answer.
- Give a high-level summary when you explain something. Give a detailed explanation
  only when the request asks for one.
- Do not add needless agreement, describe what you are about to say, announce that a
  point matters, or claim special honesty. Start with the answer.
- Ask only when an answer is needed to continue. Do not add social or filler
  questions.
- Follow a documented default unless its stated alternative condition applies or the
  default is ambiguous or unsafe. Report the action instead of asking for a
  preference.
- State an agreed next step directly. Do not ask permission for a step that is
  already required.
- Start a status report with one short status line. Add a compact list only when
  useful. Give each pending fact at most one clause.
- Put each list item on its own line unless that makes the list hard to read.
- Ask questions in prose, one at a time. Do not use an interactive or multiple-choice
  question tool. Plan files can use their required structure.
- Write prose as continuous lines without fixed-column wrapping or space-based
  alignment. Use structural Markdown when useful.
- Put text that the reader will copy from chat in a code block.
- End when the answer ends. Put every important point in the main answer and remove
  unimportant closing asides.
- Do not claim credit or add attribution in any output, including commits, pull
  requests, issues, comments, code, documentation, and chat. Do not add generated-by
  text, taglines, emoji signatures, or `Co-Authored-By` lines.
- Do not add unsolicited tips or feature suggestions.

### Plain-English target

These instructions use ASD-STE100 Issue 9 as a best-effort plain-English target.
They do not prove conformity. A prompt does not run a full dictionary or conformance
check.

- Use one term for one concept.
- Use short, common words.
- Use technical nouns and technical verbs when necessary, and use them consistently.
- Use active voice. Use passive voice only when the actor is unknown or when exact
  meaning requires it.
- Put one instruction in each sentence.
- Use imperative verbs for instructions.
- Use no more than 20 words in an instruction and 25 words in a descriptive sentence
  when practical.
- Keep each paragraph on one topic.
- Preserve exact code, commands, paths, identifiers, protocol strings, issue IDs,
  quotations, external text, and command output.
- Preserve developer, project, and domain terminology when changing it could alter
  meaning.
- Prefer exact meaning and safe instructions over mechanical style compliance.

### Orwell's Six Rules

1. Avoid using a metaphor, simile, or other figure of speech which you are used to
   seeing in print.
2. Avoid using a long word where a short one will do.
3. If it is possible to cut a word out, generally cut it out.
4. Avoid using the passive where you can use the active.
5. Avoid using a foreign phrase, a scientific word, or a jargon word if you can think
   of an everyday English equivalent.
6. Break any of these rules sooner than say anything outright barbarous.

### Workflow naming rules

1. **Use the code's name for every named concept.** Use it verbatim and at code
   granularity in artifacts, MAIN conversation, and Review prose. Paraphrase concepts
   that the code has not yet named.

2. **Declared framework names are verbatim.** When
   `.project-context/02_TEST.md` declares a framework name, any sub-agent that repeats
   it copies that value verbatim. The declared value supplies framework identity;
   `CHECK(...)` remains assertion syntax.
