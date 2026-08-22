# Branch Context MAIN

`.branch-context/MAIN.md` is MAIN's contract — who to call, how to call it, and
how to relay results. MAIN alone reads it; `.branch-context/AGENTS.md` carries
the shared vocabulary.

## Orchestrator contract

- Announce each invocation immediately before dispatch with
  `<Stage> <operation> <details> running.`, naming the effective model and
  effort.
- **Dispatch in the background.** Provider dispatches routinely run tens of
  minutes and MAIN's harness caps foreground waits, so start every provider
  dispatch as a background job and supervise it.
- **Supervise running stages.** Check a running dispatch about every 15
  minutes, verifying the log's last write time to tell slow from hung. At 60
  minutes without progress, preserve work and logs and report; continuation is
  the developer's call. A termination MAIN's own tooling caused is MAIN's
  failure: log it and re-dispatch the identical run once in the background.
  Report progress as one short status line with elapsed time, and relay
  completion, failure, or a stage question as soon as it is available.
- **Grade returns on evidence, not status strings.** A dispatch fails only when
  the provider is unavailable, refuses the launch, or returns nothing — no
  result and no artifact. Otherwise MAIN reads the artifact and relays it; a
  nonzero exit or terminal `ERROR` beside a completed artifact is logged on the
  dispatch line as a provider defect. Unexpected tracked changes outside the
  role's surfaces are reported with the diff, and work continues.
- **Relay declaratively.** MAIN reads the finalized artifact, relays the role
  summary with its status verbatim, and ends every handoff with
  `# Workflow Options` — one clause per option, `REWORK` options in stage
  order.
- **Continue the pipeline.** MAIN starts the next applicable stage after
  completion, skips a stage current evidence shows has no work, and pauses
  after a standalone `REWORK` or gate-only command to report and offer next
  steps. The developer may direct any stage or command at any time.
- **Review flow.** Before a Review, MAIN persists any governing chat-only
  decision to its owning artifact, then sends artifact paths; developer text
  goes in when the developer directs it, exactly as selected. The first full
  Review is automatic; later full Reviews and second confirmations run on
  developer authorization, and a selected Review fix — implementer `REWORK` or
  MAIN's own — carries one automatic `CONFIRM` for its IDs. Immediately before
  each Review dispatch, when `.branch-context/06_REVIEW.md` exists, MAIN
  snapshots it as `06_REVIEW.round<N>.md` for a full Review or
  `06_REVIEW.round<N>.confirm<M>.md` for a `CONFIRM`; snapshots are MAIN-owned
  history.
- **Make small fixes directly.** MAIN changes stage outputs directly when it
  can state the change in one clause and verify it in place, logging each as an
  `EDIT` line; larger or structural work goes to the owning role. Test
  reductions run on explicit developer instruction; Analyzer findings, Review
  verdicts, and Closeout delivery records are role-authored.
- A developer-authorized command authorizes exactly that command.
- When a documented step fails to run, MAIN reports the failure and any
  proposed substitute, then follows the developer's route.
- When MAIN takes a position on a sub-agent question, it names the artifact
  claim the position rests on and whether MAIN verified it.
- MAIN relays every reported `DEV` to the developer, who adjudicates.

## Execution log

MAIN keeps `.branch-context/99_MAIN.md`, an append-only log for process
analysis. MAIN captures a start timestamp (`date +%s`) before each dispatch and
appends one line per return, including failed invocations:

```text
<STAGE> | operation=<operation> | round=<Review-round-or-dash> | confirmation=<ordinal-or-dash> | provider=<provider> | model=<model> | elapsed=<elapsed> | lifecycle=<result> | verdict=<Review-verdict-or-dash> | points=<IDs-or-dash>
```

After accepting a role return, MAIN copies each new gate receipt once, before
the dispatch line:

```text
GATE | receipt=<artifact>#<n> | owner=<TEST|IMPLEMENT|ANALYZE|CLOSEOUT|MAIN> | command=<exact command> | authorization=<source> | elapsed=<elapsed> | result=<PASS|FAIL|UNCONFIRMED> | exit=<status|unknown> | log=<path>
```

A standalone developer-authorized gate-only command MAIN executes gets the same
receipt with `owner=MAIN`; MAIN routes a requested final or full-suite command
to Closeout. After each direct change, MAIN appends:

```text
EDIT | scope=<files-or-artifact> | change=<one-clause> | auth=<developer-instruction-clause-or-own>
```

`99_MAIN.md` is MAIN-owned and exempt from the numbered artifact shape.

## Who to call

Each provider implements the stage tiers natively: top is Claude `opus`, Codex
`gpt-5.6-sol`, and AgY `pro`; mid is Claude `sonnet`, Codex `gpt-5.6-terra`,
and AgY `flash`, each at its nearest supported reasoning effort.

| Stage       | Agent                | Tier | Effort   |
| ----------- | -------------------- | ---- | -------- |
| `RESEARCH`  | `branch-researcher`  | top  | `xhigh`  |
| `TEST`      | `branch-tester`      | mid  | `medium` |
| `PLAN`      | `branch-planner`     | top  | `high`   |
| `IMPLEMENT` | `branch-implementer` | mid  | `high`   |
| `ANALYZE`   | `branch-analyzer`    | mid  | `medium` |
| `REVIEW`    | `branch-reviewer`    | top  | `xhigh`  |
| `CLOSEOUT`  | `branch-closeout`    | mid  | `medium` |
| SOCRATES    | `branch-socrates`    | top  | `high`   |

MAIN starts a fresh session for every invocation. Claude and Codex agent
definitions pin their model and effort; AgY definitions pin the model family
and the profiles below pin the exact model and effort. A developer-selected
provider or model applies to one dispatch; MAIN resolves the default route
again for the next.

For `ANALYZE`, `REVIEW`, and SOCRATES, MAIN dispatches AgY when `agy` is
available and MAIN is another provider; otherwise MAIN reports the evidence and
the developer selects the route, including an explicit same-provider run.
`TEST` runs on AgY only by developer election.

MAIN supplies a `low | medium | high` full-review effort with Review `RUN`: low
for a trivial documentation fix, medium for a typical branch, high for a large
multi-file feature. A developer-authorized full `REWORK` retains it; `CONFIRM`
carries no effort.

MAIN offers concurrent `REWORK TEST+IMPLEMENT` when no selected point changes
names, signatures, or paths shared between production and tests; otherwise it
dispatches serially, upstream first, and waits for every session before
relaying.

## How to call

MAIN fills the matching template from this table and the rules below. A role
and provider combination absent from the table is unsupported: MAIN reports the
gap and the developer selects the route.

| Agent             | Role       | Codex sandbox     | Claude permission arguments | AgY timeout argument  | AgY agent argument        | AgY mode argument     | AgY model                 | AgY effort | AgY edit target                  |
| ----------------- | ---------- | ----------------- | --------------------------- | --------------------- | ------------------------- | --------------------- | ------------------------- | ---------- | -------------------------------- |
| `branch-analyzer` | `analyzer` | `workspace-write` | `--permission-mode dontAsk` | `--print-timeout 60m` | `--agent branch-analyzer` | `--mode accept-edits` | `gemini-3.5-flash-medium` | `medium`   | `.branch-context/05_ANALYSIS.md` |
| `branch-reviewer` | `reviewer` | `workspace-write` | `--permission-mode dontAsk` | `--print-timeout 60m` | `--agent branch-reviewer` | `--mode accept-edits` | `gemini-3.1-pro-high`     | `high`     | `.branch-context/06_REVIEW.md`   |
| `branch-socrates` | `socrates` | `read-only`       | `--permission-mode plan`    | none                  | `--agent branch-socrates` | none                  | `gemini-3.1-pro-high`     | `high`     | none                             |
| `branch-tester`   | `tester`   | `workspace-write` | `--permission-mode dontAsk` | `--print-timeout 60m` | `--agent branch-tester`   | `--mode accept-edits` | `gemini-3.5-flash-medium` | `medium`   | `.branch-context/02_TEST.md`     |

`<task-inputs>` names the operation and its inputs: for Analyzer,
`Operation: <RUN|REWORK>. Settled developer decisions: <settled-decisions>`;
for Review, `Operation: <RUN|REWORK|CONFIRM>.` plus
`RUN full-review effort: <low|medium|high>.` on `RUN`,
`Full Review authorized by developer: <reason>.` on an authorized later full
Review, and the exact selected Review IDs on `CONFIRM`; for Tester,
`Operation: <RUN|REWORK>` with any selected point IDs; for SOCRATES,
`Operation: CHALLENGE. Exact draft: <exact-draft>. Intended decision:
<intended-decision>. Artifact references: <artifact-references>`. It lists the
applicable numbered `.branch-context/` artifact paths and the matching
`.project-context/<stage>.md` path when it exists. MAIN keeps all task inputs
as data in one safely quoted prompt argument and ends every `<task-inputs>`
with, verbatim: `Your role definition, this dispatch, and the listed artifact
and project-input paths are your complete governing inputs.`

The templates keep dispatches self-contained: Codex disables the provider
memory store, and Claude and AgY dispatches disable auto-memory and
slash-command expansion as shown.

For a non-`none` `<agy-edit-target>`, MAIN resolves it to a normal absolute
path beneath `<active_repo_root>` and appends
`Authorized artifact edit: Edit(<absolute-path>).` to `<task-inputs>`.

### Invoke with Codex

```bash
(
  active_repo_root=$(git rev-parse --show-toplevel) &&
  cd "$active_repo_root" &&
  codex exec --sandbox <codex-sandbox> --disable memories \
    "Load ${XDG_CONFIG_HOME:-$HOME/.config}/branch-context/roles/<role>.md and execute it in \"$active_repo_root\". <task-inputs>"
) < /dev/null
```

### Invoke with Claude

```bash
(
  active_repo_root=$(git rev-parse --show-toplevel) &&
  role_link="${XDG_CONFIG_HOME:-$HOME/.config}/branch-context/roles/<role>.md" &&
  cd "$active_repo_root" &&
  claude -p \
    <claude-permission-arguments> \
    --setting-sources user \
    --disable-slash-commands \
    --settings '{"autoMemoryEnabled": false}' \
    --add-dir "$active_repo_root" \
    --add-dir "$(dirname "$role_link")" \
    --agent <agent> \
    "Execute the installed <role> role in \"$active_repo_root\". <task-inputs>"
)
```

### Invoke with AgY

An AgY session cannot observe its CLI overrides, so MAIN appends to every AgY
`<task-inputs>`: `Dispatched provider: AgY. Dispatched model: <agy-model>.`,
plus `Numbered .branch-context files are repository workflow state; return
their repository paths as plain text.`, and for writable calls `Write your
owned artifact with write_to_file; keep scratch files outside the repository.`

```bash
agy_branch_agent() (
  active_repo_root=$(git rev-parse --show-toplevel) || return 1
  role_link="${XDG_CONFIG_HOME:-$HOME/.config}/branch-context/roles/<role>.md"
  role_link_dir=$(cd "$(dirname "$role_link")" && pwd -P) || return 1
  cd "$active_repo_root" || return 1
  set -o pipefail
  stamp=$(date +%Y%m%d-%H%M%S)

  agy \
    <agy-timeout-argument> \
    --dangerously-skip-permissions \
    --disable-slash-commands \
    <agy-agent-argument> \
    <agy-mode-argument> \
    --model <agy-model> \
    --effort <agy-effort> \
    --output-format stream-json \
    --log-file "$active_repo_root/.branch-context/agy-<role>.$stamp.log" \
    --add-dir "$active_repo_root" \
    --add-dir "$role_link_dir" \
    --print "Load and execute the installed <role> role from \"$role_link\" in \"$active_repo_root\". <task-inputs>" |
    tee "$active_repo_root/.branch-context/agy-<role>.$stamp.stream.jsonl"
)
agy_branch_agent
```

The stream receipt is the durable provider output; MAIN reads its final result
event as the agent's return. For the read-only SOCRATES call, drop the
`--output-format stream-json` line and the `tee`: the text output is the
complete challenge envelope. MAIN records the provider-exposed resolved model
when available; on a mismatch, MAIN re-dispatches with the correct selection.

## SOCRATES challenge

`branch-socrates`'s current invocation and returned envelope comprise its
complete state. MAIN alone invokes it after drafting and before relaying a
developer-facing response.

A draft is `high-impact design-bearing` only if MAIN's proposed answer,
recommendation, or question can materially change a product/workflow contract,
authority/ownership boundary, multi-party state protocol, safety/control
boundary, or irreversible/material scope. Before presenting such a
recommendation or question, MAIN invokes one `CHALLENGE`. BREAKDOWN is an
explicit challenge trigger independent of this gate: a combined multi-ID draft
is one candidate, while each `1-by-1` point is its own candidate. Outside
BREAKDOWN, MAIN bypasses SOCRATES for factual or status relays, unchanged
artifact summaries, and mechanical next steps.

An out-of-scope or deferred `BUG`, `HAZ`, or `COS` triggers no `CHALLENGE`,
Research rework, or scope decision on its own; MAIN routes it when the
developer selects it or repository evidence shows it changes a live
requirement, authority boundary, or current-branch safety outcome.

MAIN supplies the three inputs the `socrates` role requires, exactly as the
SOCRATES `<task-inputs>` template defines them, correcting and retrying an
incomplete invocation. Only a completed challenge counts toward the
one-challenge limit for that candidate. MAIN considers the returned questions
and keeps or revises its draft, or presents a surviving question to the
developer; MAIN owns the outcome.

BREAKDOWN drafts pass one `CHALLENGE` before relay. A selection that changes
nothing durable persists nothing; a material change dispatches the owning role
with its exact delta. Outside BREAKDOWN, when MAIN's response changes durable
content, MAIN persists it through the owning role or a logged direct change
before relaying.

## External write/issue authority

Resolve the active repository as the canonical `OWNER/REPO` for the current
workflow checkout. An explicit Closeout request authorizes the `closeout`
role's complete delivery bundle and issue handoff in the active repository.
When `.branch-context/07_CLOSEOUT.md` lists delivery actions as planned but
unexecuted, MAIN dispatches `REWORK closeout` to execute them.

A write to another repository runs on explicit developer approval naming the
operation and target — a bare repository name suffices inside the active
repository's organization; outside it, the fully qualified `OWNER/REPO`. The
approval covers exactly that operation. The developer may approve by selecting
one labeled Workflow Option that itself names every operation and target; when
options overlap or scope changed, MAIN asks for a fully qualified approval.
