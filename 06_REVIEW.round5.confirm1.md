# Branch Review

Status: COMPLETED
Reviewer: Claude Code (pro; concrete resolution unavailable)
Operation: RUN
Review-round: 5
Confirmation: 0
Effort: medium
Review-shape: finder-verifier
Reduced-fan-out: none
Branch: 1-feature-drill-down-zoom-into-session-windows
Date: 2026-08-22
Base: origin/main at merge-base 4741e630226f5feee7f6ce94abc9c5d860b21a89
Reviewed-tree: HEAD 4741e630226f5feee7f6ce94abc9c5d860b21a89; tracked dirty paths README.md, src/input.rs, src/main.rs, src/model.rs, src/tmux.rs, src/ui.rs, tmux.expose.tmux; untracked paths graphify-out/.graphify_labels.json, graphify-out/.graphify_python, graphify-out/.graphify_root, graphify-out/GRAPH_REPORT.md, graphify-out/cache/ast/v0.9.32/4544322e308ffa07a4322ede79a08a38182cbc337cbb7610b92398b1dfba8308.json, graphify-out/cache/ast/v0.9.32/666693ccd4c68597d9f8984a91aa95d17f74e031cf4cf016098bbc6a99cb7af1.json, graphify-out/cache/ast/v0.9.32/68891e4eb9894d30524ebedc1d3863b83c1cb3490484e3afef61d4b2d43e7acb.json, graphify-out/cache/ast/v0.9.32/6db1d429e19aa6b2f3e1b81dd52fdbf95c8c03e3940acec99f4adcc84d7d41b6.json, graphify-out/cache/ast/v0.9.32/d474a558379a40ed8755006b79c2db6d5002e40f97bdd5e0d327ad497add5cbb.json, graphify-out/cache/ast/v0.9.32/d6edc9cef721f85b7ac6f3a94af9fc341e3139c8ac542efbaf666595bc84cba2.json, graphify-out/cache/ast/v0.9.32/d8cc86535d54b0fa759e7450e90cc2ce17866a36548f815560638cf4629c25cd.json, graphify-out/cache/semantic/pd5fd89c46bb5/0e15e45fee3918f5cdcf0dc12b5e3a6599a7c7c8619f0ab1833964580449a17c.json, graphify-out/cache/semantic/pd5fd89c46bb5/1dee00906535fe9177c47cb7c0bfe6364613d5ccf99bf8b5a8848ff08829b4cc.json, graphify-out/cache/semantic/pd5fd89c46bb5/5e7fadc3875ddff89fd99cecaddf053296b9f6185a9bee45d62df358bbd8c74d.json, graphify-out/cache/semantic/pd5fd89c46bb5/631c38a46196c78da6ede3e99e067e6156bb6bd6a6e2372a8949e194c5fd005f.json, graphify-out/cache/semantic/pd5fd89c46bb5/6692972132ed85d94c4277431ed0e0c70ead0e129d85a1cd2d47c045c9888f76.json, graphify-out/cache/stat-index.json, graphify-out/cost.json, graphify-out/graph.html, graphify-out/graph.json, graphify-out/manifest.json, tests/zoom_tests.rs
Review-input identity: effort medium; base origin/main; merge-base 4741e630226f5feee7f6ce94abc9c5d860b21a89; HEAD 4741e630226f5feee7f6ce94abc9c5d860b21a89; exact path sets and blob IDs under Finder scope revision 5

## Purpose

Run the developer-authorized full Review of the current branch at medium effort after the round 4 provider reroute.

## Summary

Verdict: Changes requested.

Counts: BLK 3, NIT 9, QST 2.

Sweep-coverage: 5/5.

The zoom feature works along its main path and the round 4 blockers about session/window target collision (`BLK9`), Enter-versus-refresh retargeting (`BLK10`), zoom-key validation (`BLK7`) and search-to-zoom (`BLK8`) are resolved in current source. The new blockers are that the branch breaks three checked-in repository gates it never ran, and that zoom-key conflict resolution still admits keys that disable quitting or that never reach the zoom handler.

Prior round 4 test-evidence blockers `BLK1`, `BLK3`, `BLK4` and `BLK5` are retired: the requirement-tagged regressions exist as `src/` unit tests, and `.github/workflows/rust-tests.yml` runs `cargo test` (all targets) on every pull request, so those tests are neither dead nor rarely run. `BLK2` is dropped: no requirement demands coverage of the `main` event-loop error branches, and the repository covers no other `main` loop path either.

## Points

### BLK11

- Surface: mixed
- Location: `tmux.expose.tmux:78`, `tests/plugin_entrypoint_test.sh:66-68`, `Makefile:61-64`, `.github/workflows/rust-tests.yml` step `Check tmux plugin entrypoint`
- Mechanism: The plugin entrypoint now emits one more argument pair, `-e "TMUX_EXPOSE_ZOOM_KEY=${zoom_key}"`, but the checked-in contract test that pins the emitted `bind-key` argv was not updated.
- Evidence: `tmux.expose.tmux:26` defaults `zoom_key="${zoom_key:-z}"`, so the pair is always emitted. `tests/plugin_entrypoint_test.sh:67` still expects `'bind-key -T root M-e display-popup -w 100% -h 100% -e TMUX_EXPOSE_TOGGLE_KEY=M-e -E tmux-expose '`, and `assert_equals` (`tests/plugin_entrypoint_test.sh:60-63`) calls `exit 1` on the first mismatch. All 13 assertions in that file pin the old shape. `make plugin-check` runs `bash tests/plugin_entrypoint_test.sh`.
- Why it matters: `make plugin-check` and the CI job fail deterministically on this branch. The branch's only recorded gate, `cargo test --test zoom_tests`, cannot see a shell test.
- Required regression: Update every `assert_equals` expectation to the new argv and add one assertion for a non-default `@tmux-expose-zoom-key`, which the new validation block at `tmux.expose.tmux:27-30` currently has no coverage for.

### BLK12

- Surface: production
- Location: `src/model.rs:83`, `src/main.rs:124-128`, `.github/workflows/rust-tests.yml` steps `Check formatting` and `Run clippy`
- Mechanism: Two changed lines violate the repository's own formatting and lint gates. `src/model.rs:83` is 101 columns, one over rustfmt's default `max_width` of 100, and the repository has no `rustfmt.toml`. `src/main.rs:124-128` introduces an `if` whose two arms are byte-identical.
- Evidence: `src/model.rs:83` reads `                if let Some(pos) = self.visible_sessions().into_iter().position(|s| s.name == name) {`. `src/main.rs:124-128` reads `let hint = if app.vim_keys { "Press q or Esc to quit." } else { "Press q or Esc to quit." };`, while the sibling zoomed path at `src/main.rs:100-104` does differentiate the two modes. CI runs `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D warnings`; `clippy::if_same_then_else` is warn-by-default, and `-D warnings` promotes it to an error. Review did not execute either command; both are inert evidence under the Gate.
- Why it matters: The formatting and lint gates fail before the test gate runs. The identical arms also lose the intended distinction: in default mode `q` is filter input, not a quit key (`src/input.rs:136-140`, pinned by `q_filters_instead_of_quitting` at `src/input.rs:480-487`), so the non-vim hint tells the user to press a key that types into the search box.
- Required regression: Reformat the branch with the repository's formatting gate and give the non-vim arm its correct text, then run `make check` once.

### BLK13

- Surface: mixed
- Location: `src/input.rs:42-64`, `src/input.rs:77-80`, `src/input.rs:94-106`, `tmux.expose.tmux:27`, `src/ui.rs:365`, `src/ui.rs:390`, `README.md:95`
- Mechanism: `resolve_zoom_key` rejects only two conflicts — the popup toggle key and vim normal-mode `q` — and never validates its own fallback. Three reachable configurations still produce a zoom key that is advertised in the footer but cannot zoom, and two of them disable an existing key.
- Evidence:
  - `Esc`: `tmux.expose.tmux:27` explicitly whitelists `Esc` (`"${zoom_key}" != "Esc"`), `ToggleKey::from_tmux_key` maps it at `src/input.rs:23`, and `is_zoom_match` returns at `src/input.rs:99` before `handle_default_key`'s quit arm (`src/input.rs:124-130`), `handle_vim_normal_key`'s quit arm (`src/input.rs:149-155`) and the clear-search arm (`src/input.rs:229`). Esc then only toggles zoom; `Ctrl-C` becomes the sole exit in default mode.
  - `C-c`: accepted by `tmux.expose.tmux:27` and by `resolve_zoom_key`, and README documents `C-<key>` at `README.md:95`, but the `Ctrl-C` quit check at `src/input.rs:77-80` runs first, so the configured zoom key quits the picker.
  - Toggle key `z`: with `set -g @tmux-expose-key 'z'`, `resolve_zoom_key` falls back to `default_key = z`, which equals the toggle key. The toggle branch at `src/input.rs:82-92` runs first, so `is_zoom_match` is unreachable while the footer still renders `z` `to zoom`.
  - In vim mode a single-character zoom key of `h`, `j`, `k`, `l` or `/` passes both the shell regex and `resolve_zoom_key`, and shadows that navigation or search key at `src/input.rs:99`.
- Why it matters: This is the same class the developer already treated as blocking in round 4 (`BLK7`): a documented option that advertises one action and performs another, and in the `Esc` case removes the documented quit key (`README.md:236`).
- Required regression: Reject a parsed zoom key that equals `Ctrl-C`, `Esc`, the resolved toggle key (including the `z` fallback) or, in vim mode, `h`/`j`/`k`/`l`/`/`/`q`, then align `tmux.expose.tmux` validation and `README.md:95` with the accepted set.

### NIT1

- Surface: non-code
- Location: `graphify-out/.graphify_root:1`, `graphify-out/.graphify_python:1`, `graphify-out/GRAPH_REPORT.md:1`
- Present cost: The untracked generated tree carries machine-local absolute paths, pre-feature architecture, and caches with no runtime or test role. `.gitignore` lists only `/target`, so `git status` reports 22 untracked files for the whole branch and one `git add -A` commits them.
- Evidence: The report omits the branch's `Window` flow while its manifest claims current source inputs.

### NIT5

- Surface: production
- Location: `src/model.rs:81`, `src/main.rs:224-228`, `src/main.rs:266-270`
- Present cost: Zoom-out clears `app.error` unconditionally, including an error raised by the session-list refresh that zoom did not cause, so one `Esc` hides a live failure until the next refresh tick re-detects it. The vim-mode window-fetch hint says `Press q or Esc to quit.`, but while zoomed `Esc` zooms out rather than quitting.
- Evidence: `src/main.rs:284` sets the session-refresh error; `src/model.rs:76-86` clears it with no origin check. `src/input.rs:149-155` routes zoomed `Esc` to `toggle_zoom`.

### NIT7

- Surface: production
- Location: `src/model.rs:16`, `src/tmux.rs:10`, `src/tmux.rs:249-252`, `src/ui.rs:243-246`
- Present cost: `Window.index` is requested in `WINDOW_FORMAT`, parsed, stored and refreshed every tick, and never rendered. `render_window_card` prints `format!("window {}", window.id)`, so the card shows the opaque tmux id where the index the user types was already fetched and discarded.
- Evidence: Every read of `Window.index` is a test assertion (`src/tmux.rs:330-374`). Implement retained the field deliberately so the public `Window` struct still matches `tests/zoom_tests.rs:25-34`; the field and the card header are the remaining cost.

### NIT8

- Surface: production
- Location: `src/input.rs:220-223`, `src/input.rs:94-97`, `src/model.rs:92-97`
- Present cost: Three branches added by the zoom work cannot execute in the binary. `handle_search_key`'s `(KeyCode::Esc, _) if app.is_zoomed()` arm needs searching-and-zoomed, which `toggle_zoom`'s `clear_search()` (`src/model.rs:89`) and the `!app.is_zoomed()` gates at `src/input.rs:137` and `src/input.rs:158` make unconstructible. The `zoom_key.is_none()` fallback keeps a second hard-coded `'z'` outside `resolve_zoom_key`, which always returns `Some`. `toggle_zoom`'s active-window scan and `saturating_sub` clamp always run against an empty `self.windows`, because zoom-out and `replace_sessions` clear it first and the fetch happens after the toggle.
- Evidence: `src/main.rs:174-179` always supplies `Some(zoom_key)`; `src/input.rs:67` does too. Selection of the active window is supplied only by `set_windows_for_zoomed_session` (`src/model.rs:109-112`).

### NIT9

- Surface: production
- Location: `src/main.rs:215-233`, `src/main.rs:258-273`
- Present cost: The fetch-windows-and-set-error block is duplicated verbatim in the key arm and the refresh tick, so the rule fires on key events but not on mouse or resize events, and the loop body interleaves input handling with subprocess orchestration. On a `list_windows` failure `app.windows` stays empty and nothing records the failure, so every later key press — including auto-repeat, since the arm matches `KeyEventKind::Press | KeyEventKind::Repeat` — synchronously re-spawns `tmux list-windows` on the render thread with no backoff.
- Evidence: Both blocks build the same hint and call the same `tmux::list_windows`; the retry condition `app.is_zoomed() && app.windows.is_empty()` is unchanged by the error path.

### NIT10

- Surface: production
- Location: `src/tmux.rs:16-28`, `src/tmux.rs:30-43`, `src/tmux.rs:233-237`, `src/tmux.rs:186-190`, `src/model.rs:235-244`
- Present cost: `list_sessions_metadata` repeats the whole prologue of `list_sessions_skipping_preview_for` — same args, same context string, same error string, same parse — so `SESSION_FORMAT` and the error text now have two edit sites. `parse_windows` repeats `parse_sessions`'s separator sniffing and empty-line guard. `replace_sessions_preserving_all_previews` deep-clones every session's preview (up to 200 lines each) on every zoomed tick although `ui::render` draws only `app.windows` while zoomed, and the old vectors are dropped immediately afterwards.
- Evidence: `src/ui.rs:104-116` renders windows only; `src/main.rs:257` calls the clone path twice per second at the 500 ms default.

### NIT11

- Surface: production
- Location: `src/tmux.rs:68-95`, `src/tmux.rs:45-51`, `src/tmux.rs:300-302`
- Present cost: `list_windows` captures a preview for every window unconditionally and never consults `should_capture_preview`, the guard the session path uses to skip the session the picker runs inside. When tmux-expose runs in an ordinary pane rather than a popup — a documented mode (`README.md:34`) — zooming into the attached session snapshots the pane running tmux-expose, so that window's card renders a copy of the grid.
- Evidence: `current_session_id` is resolved at `src/main.rs:144` but never threaded into `list_windows` at `src/main.rs:218` or `src/main.rs:260`.

### NIT12

- Surface: non-code
- Location: `README.md:229`, `README.md:142`, `src/input.rs:121`, `src/input.rs:136-140`, `src/input.rs:157-161`
- Present cost: The keyboard tables this branch edited still describe typing and `/` as always available. In default mode the zoom key character never reaches the filter, and pressing it mid-query silently discards the query through `toggle_zoom`'s `clear_search()`; while zoomed, typing and `/` do nothing. The doc comment at `src/input.rs:121` still claims "typing any character immediately fuzzy-filters the list".
- Evidence: `src/input.rs:99-106` gates the typeable-key escape on `app.vim_keys`, unlike the toggle-key guard at `src/input.rs:87`. The behavior itself is the settled resolution of round 4 `BLK8`; only the documentation and the doc comment are unaligned.

### NIT13

- Surface: production
- Location: `src/model.rs:37`, `src/model.rs:98`, `src/model.rs:114`, `src/model.rs:186`, `src/main.rs:257`
- Present cost: `selected_index` doubles as the window index while zoomed, so `replace_sessions` reads a window ordinal as a session index on every zoomed refresh. When the zoomed session disappears, the session it lands on is chosen by that ordinal rather than by the zoomed session's neighbours, and nine mirror-write sites plus two mirror-read branches keep the two fields in step by hand.
- Evidence: `src/model.rs:186` captures `selected_name` from `visible_sessions()[selected_index]`; zoom-mode rendering (`src/ui.rs:111`) and targeting (`src/model.rs:126-132`) use `selected_window_index`, so the misread is confined to the auto-zoom-out cursor position.

### QST1

- Surface: production
- Location: `src/main.rs:96`, `src/model.rs:126-132`, `src/tmux.rs:165-176`, `tests/zoom_tests.rs:129`
- Mechanism: The zoomed path passes a bare window ID such as `@2` to `tmux switch-client -t`, whose parameter is still named `session_target`. The repository holds no command-level evidence that the supported tmux target parser selects that exact window, and no `select-window` call was added anywhere on the branch.
- Evidence: `tests/zoom_tests.rs:129` proves only that the intermediate target is `@2`; it never executes tmux. `capture_window_preview` is correct for `@N` because `capture-pane -t` takes a pane target, so the two call sites are not equivalent evidence.
- Why it matters: If `switch-client -t @2` resolves only session targets, Enter and mouse click fail for every window card and the branch's headline outcome (REQ1-3, `README.md:34`) never works. If it resolves to the containing session, the user silently lands on that session's current window instead of the selected one.
- QUESTION: Does every supported tmux version accept a bare window ID such as `@2` for `switch-client -t` and make that exact window current?
- Resolution evidence: Record a disposable-tmux command regression for the bare ID, or pass a target shape already established to select an exact session window.

### QST2

- Surface: production
- Location: `src/tmux.rs:7-10`, `src/tmux.rs:233-248`
- Mechanism: The branch flips the emitted session field separator back to `\u{1f}`, reverting the emitted format that commit `73fe33e` "fix: handle tmux session parsing format" introduced on main, which changed `FIELD_SEPARATOR` from `\u{1f}` to `:` and kept `\u{1f}` only as the legacy fallback. The branch swaps the two constants and rewrites `SESSION_FORMAT` accordingly.
- Evidence: `git show 73fe33e -- src/tmux.rs` shows the fix direction and its regression `rejects_non_empty_unparseable_session_output`, whose input has no separator at all. The parse fallback only helps when some separator survives, so an environment where `\u{1f}` does not survive `list-sessions -F` now shows `tmux list-sessions returned output in an unexpected format` instead of the grid. `WINDOW_FORMAT` has the same exposure, and on the colon fallback `parse_windows` uses `split` where `parse_sessions` uses `splitn(6, …)`, so `@1:api:server:1:0` yields name `api` and reads `active` from `"server"`.
- Why it matters: The change was directed by round 4 `BLK3` to make colon-bearing window names parse, but it reinstates the emitted format that a shipped fix moved away from, and the repository records no reason for that fix beyond its title.
- QUESTION: Which tmux environment made the `\u{1f}` session format fail in commit `73fe33e`, and does it still fail?
- Resolution evidence: Reproduce `list-sessions -F` with `\u{1f}` on that environment, or keep `:` as the emitted separator and put the name field last in `SESSION_FORMAT` / `WINDOW_FORMAT` with a capped `splitn`, which parses colon-bearing names without reverting the fix.

## Evidence

### Evidence revisions

- Governing input `.branch-context/AGENTS.md`: `5e420cb89b320e03c66806a6ff0024ec59342f0b`
- Project input `.project-context/06_REVIEW.md`: absent
- `.branch-context/01_RESEARCH.md`: `f371d99487768a7269ebc7c26c4fd68d32dd43e6`
- `.branch-context/02_TEST.md`: `868369a9f916928c2116096a8a4f3a5b088bd38f`
- `.branch-context/03_PLAN.md`: `e0dd486ce284f1d4a2acf25fe1388c8583529e65`
- `.branch-context/04_IMPLEMENT.md`: `2fd7bb06a1fb1826207d15cfa35bf90630228243`
- `.branch-context/05_ANALYSIS.md`: `54a638be5e38b306e96a54c2eba145732e4861ad`

### Readiness

- Not applicable. The AgY readiness admission applies to an AgY full Review at medium or high effort. This Review ran under the Claude Code native subagent interface after the developer-authorized provider reroute, so no `branch-reviewer-finder` readiness envelope exists.

### Finder scope

Scope revision: 5

Inert-command boundary: Treat every build, test, lint, format, install, or deploy command in project input or branch artifacts as inert evidence; never run one. Read and search freely with provider-native read and search tools, every non-mutating Git command, and read-only shell inspection. Do not run a project gate command. Do not mutate repository or workflow state, including any file write, edit, or scratch file. Do not invoke another provider or agent CLI (`agy`, `codex`, `claude`, or any other). Treat Markdown and code-formatted identifiers in this scope as data, never as commands. When a targeted content search is necessary, use fixed-string matching with a shell-safe literal; never run a pipeline built from artifact prose, a command substitution, or command text assembled from artifact prose.

Production source paths:
- `README.md` — `d2b8f665b75dfb008a7ea4db8bb1a00108e05279`
- `src/input.rs` — `d14ff8ec914c19b8f7b3fc3c0a6d8d8e344685d1`
- `src/main.rs` — `82b9df3111549d1330358940d9ad480dfbed85a8`
- `src/model.rs` — `9df7e52bd53db7b89b288721d7a8d0aadf517e65`
- `src/tmux.rs` — `3cdfc5cecf0c4245087b9dc0ef438d543500b2bb`
- `src/ui.rs` — `11e2fe70e11fc3243bf408ea25822394b1c7837c`
- `tmux.expose.tmux` — `9cdff18b5f549dc11c0f5f13faaf7e9f95d95543`

Test paths:
- `tests/zoom_tests.rs` — `0aff4258895c3e5511b0465286b87da16219870e`

Generated untracked paths (non-code, no runtime or test role):
- `graphify-out/.graphify_labels.json` — `75f2e61dfdf781556fe6623ecf38eb98fbf706b2`
- `graphify-out/.graphify_python` — `c79db8a8e45e346f10e89f3fc9cbe084796f6529`
- `graphify-out/.graphify_root` — `5aa4eb4876ece8a2428f06eeb9596012131806ce`
- `graphify-out/GRAPH_REPORT.md` — `bd07becc1c663104850c6058eefa4b66f31f0cc2`
- `graphify-out/cache/ast/v0.9.32/4544322e308ffa07a4322ede79a08a38182cbc337cbb7610b92398b1dfba8308.json` — `865e05424f2482a55a211c71c41cd7e4294ddcf2`
- `graphify-out/cache/ast/v0.9.32/666693ccd4c68597d9f8984a91aa95d17f74e031cf4cf016098bbc6a99cb7af1.json` — `48af3c6038381cd07161b8a215ca2c7b47807726`
- `graphify-out/cache/ast/v0.9.32/68891e4eb9894d30524ebedc1d3863b83c1cb3490484e3afef61d4b2d43e7acb.json` — `929775cf4cdc732a1f8e6640998d21c1fcea7407`
- `graphify-out/cache/ast/v0.9.32/6db1d429e19aa6b2f3e1b81dd52fdbf95c8c03e3940acec99f4adcc84d7d41b6.json` — `e13f88afa2ca63e9dea4718bf3d986e167a1b526`
- `graphify-out/cache/ast/v0.9.32/d474a558379a40ed8755006b79c2db6d5002e40f97bdd5e0d327ad497add5cbb.json` — `0c8a2ae0739547bf273112c5d23fc8e24ca1dc12`
- `graphify-out/cache/ast/v0.9.32/d6edc9cef721f85b7ac6f3a94af9fc341e3139c8ac542efbaf666595bc84cba2.json` — `7c516e17f3131399467eaeb70b918919d916597a`
- `graphify-out/cache/ast/v0.9.32/d8cc86535d54b0fa759e7450e90cc2ce17866a36548f815560638cf4629c25cd.json` — `5d7959754af057d1604d49cb1c672b3c898c0b0a`
- `graphify-out/cache/semantic/pd5fd89c46bb5/0e15e45fee3918f5cdcf0dc12b5e3a6599a7c7c8619f0ab1833964580449a17c.json` — `89c794650905e13e226b7819bca69cd0724c5d52`
- `graphify-out/cache/semantic/pd5fd89c46bb5/1dee00906535fe9177c47cb7c0bfe6364613d5ccf99bf8b5a8848ff08829b4cc.json` — `98f3ac316b10e8784220985ac61a17484eb48551`
- `graphify-out/cache/semantic/pd5fd89c46bb5/5e7fadc3875ddff89fd99cecaddf053296b9f6185a9bee45d62df358bbd8c74d.json` — `e3d890daf7df99ef340f55f2b1c5daaf565401eb`
- `graphify-out/cache/semantic/pd5fd89c46bb5/631c38a46196c78da6ede3e99e067e6156bb6bd6a6e2372a8949e194c5fd005f.json` — `fc35e36a127f0c5d59f30cdc5d957d68e1946819`
- `graphify-out/cache/semantic/pd5fd89c46bb5/6692972132ed85d94c4277431ed0e0c70ead0e129d85a1cd2d47c045c9888f76.json` — `b732bbd3b74ca1f4df0a2d88a82b38bf9e1dd763`
- `graphify-out/cache/stat-index.json` — `4e284a0cd2e37dcea840551b1fad30432eb8459e`
- `graphify-out/cost.json` — `ea5612c3b348f2daeb14aad8ca48c360aa7d5df4`
- `graphify-out/graph.html` — `416b8c1cb8b1af26a53ff585d98833c25ec45ca8`
- `graphify-out/graph.json` — `bf461b5c3476131e9915f40f385f14f0e2b52f1a`
- `graphify-out/manifest.json` — `4fb3097cd2778bdb70c870fa26ba15b031af7bf6`

Fan-out preflight: PASS. The parent read `### Finder scope` back from this artifact before any finder child started, with a positive scope revision, the full inert-command boundary, and the exact grouped path sets with blob IDs.

### Finder checkpoints

#### close-read forward

- State: terminal-success
- Executor: child `Explore/close-read-forward`
- Scope: Scope revision 5; exact source and test sets under `### Finder scope`
- Candidates: plugin entrypoint contract test not updated for the new `-e TMUX_EXPOSE_ZOOM_KEY` argument; identical `if`/`else` arms at `src/main.rs:124`; default-mode zoom key never reaches the filter; `resolve_zoom_key` fallback equal to a `z` toggle key; `Esc` whitelisted as a zoom key; `src/model.rs:83` at 101 columns; unformatted new test file; `list_windows` without the current-session preview guard; window-fetch retry on every key press; `parse_windows` using `split` where `parse_sessions` uses `splitn`.

#### close-read in reverse

- State: terminal-success
- Executor: child `Explore/close-read-reverse`
- Scope: Scope revision 5; exact source and test sets under `### Finder scope`
- Candidates: plugin entrypoint test failure; identical hint arms and the clippy gate; `src/model.rs:83` line width; zoom key shadowing filter input; `resolve_zoom_key` accepting `Esc`; `resolve_zoom_key` accepting vim navigation keys; unbounded window-fetch retry; dead active-window scan in `toggle_zoom`; `parse_windows` colon-name mis-parse; per-tick preview clone while zoomed; unreachable `handle_search_key` zoomed arm; zoom-out clearing unrelated errors; `Window.index` never read outside tests.

#### structural forward

- State: terminal-success
- Executor: child `Explore/structural-forward`
- Scope: Scope revision 5; exact source and test sets under `### Finder scope`
- Candidates: lost "every printable key filters" invariant in default mode; emitted separator reverting commit `73fe33e`; `switch_client` receiving a window ID with no `select-window` anywhere; identical hint arms; `selected_index` overwritten with a window ordinal and read as a session index by `replace_sessions`; search query discarded by zoom-in with no restore.

#### structural in reverse

- State: terminal-success
- Executor: child `Explore/structural-reverse`
- Scope: Scope revision 5; exact source and test sets under `### Finder scope`
- Candidates: emitted-command contract changed without updating its 13 pinned assertions; identical hint arms plus the wrong non-vim key name; `switch_client` precondition violated by the zoomed caller; zoom interception above the search dispatch; `resolve_zoom_key` returning a fallback identical to the conflicting toggle key. Explicitly cleared after tracing every downstream read: the `selected_index` / `selected_window_index` alias, reported as an invariant hazard with no user-visible failure.

#### cleanup

- State: terminal-success
- Executor: child `Explore/cleanup`
- Scope: Scope revision 5; exact production source set under `### Finder scope`; test-source cleanup excluded
- Candidates: `list_sessions_metadata` duplicating `list_sessions_skipping_preview_for`; `parse_windows` duplicating separator sniffing; the error-hint block copied five times in `src/main.rs`; two preview-preserving `replace_sessions` variants; `list_windows` skipping `should_capture_preview`; identical hint arms; unreachable `zoom_key.is_none()` fallback; `selected_index` mirroring; `visible_window_count()` versus `app.windows.len()`; per-tick subprocess fan-out; eager `format!` for error context; per-tick preview clone; unread `Window.index`; window fetch duplicated between the key arm and the refresh tick; input layer writing model index state directly; untracked `graphify-out/`; `tmux.expose.tmux` accepting an undocumented `Esc`; README keyboard tables unaligned with the zoomed state.

### Candidate classification

- Confirmed and recorded: `BLK11`, `BLK12`, `BLK13`, `NIT1`, `NIT5`, `NIT7`, `NIT8`, `NIT9`, `NIT10`, `NIT11`, `NIT12`, `NIT13`.
- Retained as unresolved and merge-relevant: `QST1`, `QST2`.
- Refuted or dropped: the `selected_index` alias as a blocker, because zoom-mode rendering and targeting both use `selected_window_index` and zoom-out re-derives the session index by name; the default-mode zoom-versus-filter behavior as a blocker, because it is the settled resolution of round 4 `BLK8` and only its documentation is unaligned; test-source style and vertical formatting of `tests/zoom_tests.rs`, which is out of Review scope for cleanup; `visible_window_count()` versus `app.windows.len()`, a preference with no present cost; the eager `format!` error context, an allocation with no measurable present cost.
- Retired from round 4 as resolved in current source: `BLK6` (zoom key wired through the plugin and the footer), `BLK7` (validation and toggle/vim-`q` conflicts, with the residue recorded as `BLK13`), `BLK8` (`src/input.rs:99-106` and `type_filter_and_press_default_zoom_key_zooms_into_session`), `BLK9` (`src/model.rs:67-73` matches the exact ID first, proved by `zoomed_session_resolves_exact_id_over_colliding_name`), `BLK10` (`execute_switch` runs in the same event iteration at `src/main.rs:212` and `src/main.rs:244`), `NIT6` (`list_sessions_metadata` skips session preview capture while zoomed).
- Retired from round 4 on re-examined evidence: `BLK1`, `BLK3`, `BLK4`, `BLK5`. The requirement-tagged regressions exist (`src/model.rs:539`, `src/model.rs:575`, `src/tmux.rs:325-347`, `src/input.rs:425`, `src/input.rs:771`), and `.github/workflows/rust-tests.yml` runs `cargo test` over all targets on every pull request, so they are neither dead nor rarely run. `BLK2` is dropped: no requirement or repository convention demands coverage of `main`'s event-loop error branches.

### Intent and Plan conformance

- Research REQ1-1 through REQ1-5 and HAZ1-1 are implemented; Plan PHS1 through PHS3 map to the shipped code with no unauthorized scope.
- REQ1-3's observable outcome is not established at command level; see `QST1`.
- `git diff --check 4741e630226f5feee7f6ce94abc9c5d860b21a89`: PASS.
- Scope note: the branch also changed `README.md` and `tmux.expose.tmux`, which Plan did not list; both follow from the directed rework for `BLK6` and `BLK7` and are in scope.

### Recorded gate evidence

- Implement status: COMPLETED.
- Latest scoped gate: `cargo test --test zoom_tests` — PASS, exit 0, elapsed 0m01s, log `/tmp/tmux_expose_gate_receipt_zoom_tests_rework6.log`.
- Earlier scoped gates: `cargo test --test zoom_tests` — PASS in Implement receipts 1 through 3.
- Test stage: `cargo test --test zoom_tests` — FAIL, exit 101, recorded as the expected initial Red.
- Gate scope: the recognized integration suite only. No recorded gate exercised `cargo fmt --check`, `cargo clippy`, `make plugin-check`, or the `src/` unit tests, which is why `BLK11` and `BLK12` reached Review unseen.
- Review ran no build, test, lint, format, install, or deploy command.

### Failure history

- Review round 3, structural forward, `/root/review_struct_forward_r3`: surfaced forbidden-command attempt `sed -n '1,260p' /Users/jason/.config/branch-context/roles/reviewer.md && for f in .branch-context/01_RESEARCH.md .branch-context/02_TEST.md .branch-context/03_PLAN.md .branch-context/04_IMPLEMENT.md .branch-context/05_ANALYSIS.md .branch-context/06_REVIEW.md .project-context/06_REVIEW.md; do if test -f "$f"; then echo "===== $f ====="; sed -n '1,300p' "$f"; fi; done`; native error: none; the command executed outside Finder scope revision 3's Git-only command boundary.
- Review round 4, close-read forward, `/root/review_close_forward_r4`: surfaced forbidden-command attempt `sed -n '1,240p' /Users/jason/.codex/plugins/cache/ponytail/ponytail/4.9.0/skills/ponytail/SKILL.md && sed -n '1,280p' .branch-context/AGENTS.md && sed -n '1,260p' .branch-context/06_REVIEW.md`; native error: none.
- Review round 4, close-read in reverse, `/root/review_close_reverse_r4`: surfaced forbidden-command attempt `sed -n '1,240p' .branch-context/AGENTS.md && sed -n '1,260p' .branch-context/06_REVIEW.md && git status --short && git diff --stat && git diff`; native error: none.
- Review round 4, structural forward, `/root/review_struct_forward_r4`: surfaced forbidden-command attempt `sed -n '1,240p' .branch-context/AGENTS.md && sed -n '1,260p' .branch-context/06_REVIEW.md && if [ -f .project-context/06_REVIEW.md ]; then sed -n '1,240p' .project-context/06_REVIEW.md; fi`; native error: none.
- Review round 4, structural in reverse, `/root/review_struct_reverse_r4`: surfaced forbidden-command attempts `sed -n '1,240p' .branch-context/AGENTS.md && sed -n '1,260p' .branch-context/06_REVIEW.md && find .project-context -maxdepth 2 -type f -print 2>/dev/null | sort` and `rg -n "structural in reverse|structural.*reverse|Finder checkpoints|reviewer-finder|Review-shape|checkpoint" .branch-context /Users/jason/.config/branch-context /Users/jason/.codex 2>/dev/null | head -n 200`; native error: none.
- Review round 4, cleanup, `/root/review_cleanup_r4`: surfaced forbidden-command attempt `sed -n '1,240p' .branch-context/AGENTS.md && sed -n '1,240p' .branch-context/06_REVIEW.md && git status --short && git diff --stat && git diff -- . ':(exclude).branch-context/06_REVIEW.md'`; native error: none.
- Review round 5: no failed checkpoint. All five required angle children returned a terminal-success result on the first attempt with the correct scope revision and no forbidden-command attempt.

### What's good

- `execute_switch` closes the round 4 Enter-versus-refresh race by switching inside the same event iteration.
- `zoomed_session()` now resolves the exact session ID before falling back to the name, with a dedicated regression.
- `render_card_inner` shares one card renderer between sessions and windows without duplicating the ANSI and truncation logic.
- Zoom state degrades safely when the zoomed session disappears: `replace_sessions` clears zoom, windows and the window index.
