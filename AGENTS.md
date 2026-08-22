# Branch Context

`.branch-context/` holds this branch's durable workflow artifacts.
`.project-context/` (optional) holds project-specific per-stage inputs; they
specialize the workflow and take precedence over role defaults. When project
input conflicts with another active project instruction, the stage owner
follows the project input and records a `QST` naming the conflict for
correction.

The developer supplies direction and authorization, writes `00_DEVNOTE.md`,
and annotates artifacts with `> NOTE:`. Each stage's sub-agent loads and
executes its role definition; the role definition, the dispatch, and the
listed artifacts are its complete governing inputs.

## The workflow

RESEARCH → TEST → PLAN → IMPLEMENT → ANALYZE → REVIEW → CLOSEOUT

| Stage       | Agent                | Role          | Artifact          |
| ----------- | -------------------- | ------------- | ----------------- |
| `RESEARCH`  | `branch-researcher`  | `researcher`  | `01_RESEARCH.md`  |
| `TEST`      | `branch-tester`      | `tester`      | `02_TEST.md`      |
| `PLAN`      | `branch-planner`     | `planner`     | `03_PLAN.md`      |
| `IMPLEMENT` | `branch-implementer` | `implementer` | `04_IMPLEMENT.md` |
| `ANALYZE`   | `branch-analyzer`    | `analyzer`    | `05_ANALYSIS.md`  |
| `REVIEW`    | `branch-reviewer`    | `reviewer`    | `06_REVIEW.md`    |
| `CLOSEOUT`  | `branch-closeout`    | `closeout`    | `07_CLOSEOUT.md`  |

The non-stage `branch-socrates` runs MAIN's `CHALLENGE`.

Each stage begins `DRAFT` and becomes `COMPLETED` when the role finishes its
work; a stage with no applicable work completes recording that result.
Reported findings still govern next steps.

## Operations

- `RUN <stage>` — start or continue a stage.
- `REWORK <stage> <point IDs>` — the named stage addresses those points;
  `<stage>+<stage>` runs stages concurrently.
- `CONFIRM <Review IDs>` — reviewer re-checks fixed findings without a new
  review round.
- `BREAKDOWN <point IDs>` — read-only discussion: MAIN explains each selected
  point and drafts options with a recommendation.
- `PROCEED` — continue past the reported condition.
- `REBASE` / `RESTORE REBASE` — Closeout rebase and its recovery, each on the
  developer's explicit call.
- `DRAFT ISSUE` — loads the `draft-issue` skill; creates no numbered artifact.

Scoped gates belong to Implement; the branch's one full-suite final gate
belongs to Closeout. Delivery after a failed final gate proceeds on developer
`PROCEED`.

## Shared shapes

Numbered artifacts start with role-required identity metadata and `Status`,
then `Purpose`, `Summary`, `Points`, and `Evidence`; write `none` for an empty
required section. Write numbered artifacts with your native file-write tool;
keep scratch files outside the repository. MAIN moves any stray file out and
continues.

Each role finalizes and reads back its artifact, then returns one compact
summary: `# <role> summary`, `Artifact:`, `Status:`, then populated
artifact-derived groups, questions and findings last. Each point:
`- <ID> — Surface: <production|test|mixed|non-code> — <summary>`. Preserve
exact commands, IDs, and `QUESTION:` text; detail stays in the artifact.

Every gate execution records one numbered `### Gate receipt <n>` under the
owning artifact's `Evidence`:

- `Owner: TEST | IMPLEMENT | ANALYZE | CLOSEOUT`
- `Command: <exact command>`
- `Authorization: <project-input path or quoted developer instruction>`
- `Elapsed: <NmNNs>`
- `Result: PASS | FAIL | UNCONFIRMED`
- `Exit: <integer | unknown>`
- `Log: <unique complete-log path>`

Clear success in the log or stdout records `PASS` even when the numeric exit
status is missing; the log path aids debugging.

### Points

`BLK` blockers, `NIT` non-blocking findings, `QST` questions, plus role-owned
series: `REQ`/`HAZ`/`BUG`/`COS` (researcher), `TST`/`NOT` (tester), `PHS`/`TSK`
(planner), `DEV` (implementer). Research mints `REQ<issue>-<n>`, the one
issue-qualified series; other series are local to their artifact. Cite local
IDs bare and cross-artifact IDs owner-qualified (`RESEARCH:QST3`). Each ID
keeps its point for the life of its artifact; new IDs take the next unused
number.

A directly affected checked-in documentation or guidance inconsistency is a
Review `BLK`; optional polish is a `NIT` the developer selects. Review `Status`
reports execution; `Verdict` carries judgment.

## Language

- Match response length to the work; start with the answer.
- Use short common words, active voice, one term per concept.
- Ask only questions needed to continue, in prose, one at a time.
- Follow a documented default and report the action; state agreed next steps
  directly.
- Put copyable text in a code block; write prose as continuous lines.
- Preserve exact code, commands, paths, IDs, quotations, and output.
- Use the code's name for every named concept, verbatim; copy declared
  framework names verbatim.
- No credit, taglines, or attribution in any output.
