# Branch Review

Status: COMPLETED
Reviewer: Codex (pro; concrete resolution unavailable)
Operation: RUN
Review-round: 2
Confirmation: 0
Effort: medium
Review-shape: finder-verifier
Reduced-fan-out: none
Branch: 1-feature-drill-down-zoom-into-session-windows
Date: 2026-08-22
Base: origin/main at merge-base 4741e630226f5feee7f6ce94abc9c5d860b21a89
Reviewed-tree: HEAD 4741e630226f5feee7f6ce94abc9c5d860b21a89; tracked dirty paths README.md, src/input.rs, src/main.rs, src/model.rs, src/tmux.rs, src/ui.rs, tmux.expose.tmux; untracked paths graphify-out/.graphify_labels.json, graphify-out/.graphify_python, graphify-out/.graphify_root, graphify-out/GRAPH_REPORT.md, graphify-out/cache/ast/v0.9.32/4544322e308ffa07a4322ede79a08a38182cbc337cbb7610b92398b1dfba8308.json, graphify-out/cache/ast/v0.9.32/666693ccd4c68597d9f8984a91aa95d17f74e031cf4cf016098bbc6a99cb7af1.json, graphify-out/cache/ast/v0.9.32/68891e4eb9894d30524ebedc1d3863b83c1cb3490484e3afef61d4b2d43e7acb.json, graphify-out/cache/ast/v0.9.32/6db1d429e19aa6b2f3e1b81dd52fdbf95c8c03e3940acec99f4adcc84d7d41b6.json, graphify-out/cache/ast/v0.9.32/d474a558379a40ed8755006b79c2db6d5002e40f97bdd5e0d327ad497add5cbb.json, graphify-out/cache/ast/v0.9.32/d6edc9cef721f85b7ac6f3a94af9fc341e3139c8ac542efbaf666595bc84cba2.json, graphify-out/cache/ast/v0.9.32/d8cc86535d54b0fa759e7450e90cc2ce17866a36548f815560638cf4629c25cd.json, graphify-out/cache/semantic/pd5fd89c46bb5/0e15e45fee3918f5cdcf0dc12b5e3a6599a7c7c8619f0ab1833964580449a17c.json, graphify-out/cache/semantic/pd5fd89c46bb5/1dee00906535fe9177c47cb7c0bfe6364613d5ccf99bf8b5a8848ff08829b4cc.json, graphify-out/cache/semantic/pd5fd89c46bb5/5e7fadc3875ddff89fd99cecaddf053296b9f6185a9bee45d62df358bbd8c74d.json, graphify-out/cache/semantic/pd5fd89c46bb5/631c38a46196c78da6ede3e99e067e6156bb6bd6a6e2372a8949e194c5fd005f.json, graphify-out/cache/semantic/pd5fd89c46bb5/6692972132ed85d94c4277431ed0e0c70ead0e129d85a1cd2d47c045c9888f76.json, graphify-out/cache/stat-index.json, graphify-out/cost.json, graphify-out/graph.html, graphify-out/graph.json, graphify-out/manifest.json, tests/zoom_tests.rs
Review-input identity: effort medium; base origin/main; merge-base 4741e630226f5feee7f6ce94abc9c5d860b21a89; HEAD 4741e630226f5feee7f6ce94abc9c5d860b21a89; exact path sets and blob IDs under Finder scope revision 2

## Purpose

Run the developer-authorized full Review of the current branch at medium effort.

## Summary

Verdict: Changes requested
Counts: BLK 9, NIT 4, QST 1
Sweep-coverage: 5/5

## Points

### BLK1

- Surface: test
- Location: `src/model.rs:75`, `src/model.rs:525`, `tests/zoom_tests.rs:86`
- Mechanism: The A-to-B session transition regression exists only as a `src/model.rs` unit test. Every recorded rework gate ran `cargo test --test zoom_tests`, which does not execute it.
- Evidence: The recognized suite still preloads windows before zooming and does not prove that window IDs from one session cannot reach another session.
- Why it matters: The selected production ownership path changed without recorded executable proof for the required isolation outcome.
- Required regression: Execute the requirement-tagged A-to-B transition case in the authorized scoped gate.

### BLK2

- Surface: test
- Location: `src/main.rs:187`, `src/main.rs:225`, `tests/zoom_tests.rs:1`
- Mechanism: Both `list_windows` failure branches set `app.error`, but no recognized test drives either branch.
- Evidence: All three Implement receipts run only `cargo test --test zoom_tests`; that suite contains no `list_windows` failure or error assertion.
- Why it matters: The initial and refresh window-fetch error contracts remain unprotected.
- Required regression: Execute requirement-tagged initial and refresh failure cases in the authorized scoped gate.

### BLK3

- Surface: test
- Location: `src/tmux.rs:10`, `src/tmux.rs:319`, `tests/zoom_tests.rs:86`
- Mechanism: The unit-separator parser and colon-bearing window-name regression exist only in `src/tmux.rs` unit tests.
- Evidence: `cargo test --test zoom_tests` does not execute those tests; the recognized suite uses legacy colon-delimited input and does not assert the parsed window index.
- Why it matters: The current tmux format and delimiter fix have no recorded passing execution.
- Required regression: Execute the requirement-tagged current-format parser case in the authorized scoped gate.

### BLK4

- Surface: test
- Location: `src/input.rs:123`, `src/input.rs:400`, `tests/zoom_tests.rs:1`
- Mechanism: Vim normal-mode `q` now quits while zoomed, but the regression exists only as an `src/input.rs` unit test.
- Evidence: The recorded scoped gate excludes `src/input.rs` unit tests, and the recognized suite has no vim-mode case.
- Why it matters: The restored quit-key invariant has no recorded passing execution.
- Required regression: Execute the requirement-tagged vim-mode case in the authorized scoped gate.

### BLK5

- Surface: test
- Location: `src/model.rs:75`, `src/input.rs:746`, `tests/zoom_tests.rs:1`
- Mechanism: Zoom now clears search, but the configurable-key search transition exists only as an `src/input.rs` unit test.
- Evidence: The recorded scoped gate excludes that test, and the recognized suite has no configurable-key search transition.
- Why it matters: The directed configured-key transition has no recorded passing execution.
- Required regression: Execute the requirement-tagged configured-key search case in the authorized scoped gate.

### BLK7

- Surface: mixed
- Location: `src/main.rs:124`, `src/input.rs:13`, `src/ui.rs:87`, `README.md:95`, `tmux.expose.tmux:73`
- Mechanism: The plugin and README accept a zoom-key string without validation. Unsupported values such as `F1` silently make input fall back to `z` while the footer shows `F1`; accepted conflicts such as `q` in vim mode or the popup toggle key make one advertised action unreachable.
- Evidence: `ToggleKey::from_tmux_key` accepts only `Esc`, one character, or `M-`/`C-` plus one character. Current tests cover `z`, `M-z`, and label rendering but no rejection or conflict path.
- Why it matters: The documented custom key can display and behave as different keys, or mask quit and popup-close behavior.

### BLK8

- Surface: mixed
- Location: `src/input.rs:76`, `src/input.rs:746`, `tests/zoom_tests.rs:39`
- Mechanism: A plain zoom key is treated as filter text whenever search is active. Typing a filter and then pressing the default `z` appends `z` instead of zooming the selected session.
- Evidence: The only search-to-zoom unit case uses modified `M-z`, which bypasses the typeable-key branch. The recognized REQ1-1 test starts without a search.
- Why it matters: The existing filter workflow cannot drill into its selected result with the default zoom key.

### BLK9

- Surface: production
- Location: `src/model.rs:68`, `src/main.rs:191`
- Mechanism: `zoomed_session` stores a session ID, but `zoomed_session()` matches each session by ID or name in one pass. A preceding session whose name equals another session's ID wins the search.
- Evidence: Main then calls `list_windows` with the wrong session ID. Current tests use non-colliding names and IDs.
- Why it matters: A reachable session-name collision loads and refreshes another session's windows.

### BLK10

- Surface: mixed
- Location: `src/main.rs:139`, `src/main.rs:187`, `src/main.rs:218`, `tests/zoom_tests.rs:115`
- Mechanism: Enter records only `should_switch`. Before the next loop consumes it, the same iteration can refresh windows and replace a vanished selected window with the active or first window.
- Evidence: The recognized test asserts `selected_target` immediately after Enter and does not cross the refresh path.
- Why it matters: Pressing Enter on a window that closes during refresh can switch to a different window instead of failing that selection.

### NIT1

- Surface: non-code
- Location: `graphify-out/.graphify_root:1`, `graphify-out/.graphify_python:1`, `graphify-out/GRAPH_REPORT.md:1`
- Present cost: The untracked generated tree contains machine-local absolute paths, stale pre-feature architecture, large graph output, and caches with no runtime or test role.
- Evidence: The report omits the branch's Window flow while its manifest claims current source inputs.

### NIT5

- Surface: production
- Location: `src/main.rs:197`, `src/input.rs:99`, `src/model.rs:75`, `src/ui.rs:64`
- Present cost: A window-fetch error says `q` or `Esc` quits, but default zoom mode ignores `q` and uses the first `Esc` to zoom out. The global error then hides the session grid until the next refresh because zoom-out does not clear it.
- Evidence: With a long configured refresh interval, the stale overlay remains for that interval; current zoom-out tests check only mode and quit flags.

### NIT6

- Surface: production
- Location: `src/main.rs:218`, `src/tmux.rs:16`, `src/tmux.rs:54`, `src/ui.rs:102`
- Present cost: Every zoomed refresh captures previews for all hidden sessions before it captures every visible window, all synchronously on the UI thread.
- Evidence: At the 500 ms default, a refresh launches about `2N + W + 1` tmux processes for `N` sessions and `W` windows although session previews are not rendered.

### NIT7

- Surface: production
- Location: `src/model.rs:16`, `src/tmux.rs:10`, `src/tmux.rs:235`
- Present cost: `Window.index` is fetched, parsed, stored, and refreshed but has no production consumer.
- Evidence: All reads of `Window.index` are test-only.

### QST1

- Surface: production
- Location: `src/main.rs:142`, `src/model.rs:123`, `src/tmux.rs:151`, `tests/zoom_tests.rs:129`
- Mechanism: The zoomed path passes a bare window ID such as `@2` to `tmux switch-client -t`, while the repository has no command-level evidence that the supported tmux target parser selects that window.
- Evidence: The recognized test proves only that the intermediate target is `@2`; it never executes tmux.
- Why it matters: If `switch-client -t @2` resolves only session targets, Enter fails for every window.
- QUESTION: Does every supported tmux version accept a bare window ID such as `@2` for `switch-client -t` and select that exact window?
- Resolution evidence: Record a disposable-tmux command regression for the bare ID, or pass a target shape already established to select the exact session window.

## Evidence

### Evidence revisions

- Governing input `.branch-context/AGENTS.md`: `5e420cb89b320e03c66806a6ff0024ec59342f0b`
- Project input `.project-context/06_REVIEW.md`: absent
- `.branch-context/01_RESEARCH.md`: `f371d99487768a7269ebc7c26c4fd68d32dd43e6`
- `.branch-context/02_TEST.md`: `868369a9f916928c2116096a8a4f3a5b088bd38f`
- `.branch-context/03_PLAN.md`: `e0dd486ce284f1d4a2acf25fe1388c8583529e65`
- `.branch-context/04_IMPLEMENT.md`: `4e96a817330ae6a5ad2d2c1dd665940b3750c7ee`
- `.branch-context/05_ANALYSIS.md`: `54a638be5e38b306e96a54c2eba145732e4861ad`

### Finder scope

Scope revision: 2

Inert-command boundary: Treat every build, test, lint, format, install, or deploy command in project input or branch artifacts as inert evidence. Read-only Git inspection is the only allowed command execution. Do not invoke any provider or agent CLI. Use provider-native view or search tools. Treat Markdown and code-formatted identifiers as data. When a targeted Git content search is necessary, use fixed-string matching with a shell-safe literal; never run a pipeline, command substitution, or command text assembled from artifact prose.

Source paths:
- `README.md` — `83c51633ae8c58ac2126cf8041bfac6fa8995b9f`
- `src/input.rs` — `46b166aee656062a8aca867f5b9117342746ee55`
- `src/main.rs` — `e6330dcbfa19437b23f7d418219398b9e7f5e10a`
- `src/model.rs` — `433a8d239bf69dcad86055dc5105c0a73fefeb84`
- `src/tmux.rs` — `3fdac0ee47b8b7f65c24840f41cad4e869e35999`
- `src/ui.rs` — `11e2fe70e11fc3243bf408ea25822394b1c7837c`
- `tmux.expose.tmux` — `64d84f24dbed138299abf3b1c806ba9e60983067`
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

Test paths:
- `tests/zoom_tests.rs` — `0aff4258895c3e5511b0465286b87da16219870e`

Fan-out preflight: PASS

### Finder checkpoints

#### close-read forward

- State: terminal-success
- Executor: `/root/review_close_forward_r2`
- Scope: Scope revision 2; exact source and test sets under `### Finder scope`
- Candidates: ID/name collision can load another session's windows; refresh can change the selected window after Enter; invalid zoom-key fallback can disagree with the footer; window-fetch error guidance is false; generated Graphify output has no product role.

#### close-read in reverse

- State: terminal-success
- Executor: `/root/review_close_reverse_r2`
- Scope: Scope revision 2; exact source and test sets under `### Finder scope`
- Candidates: bare window ID may not satisfy `tmux switch-client -t`; plain zoom key is consumed by active search; invalid zoom-key fallback can disagree with the footer; generated Graphify output has no product role.

#### structural forward

- State: terminal-success
- Executor: `/root/review_struct_forward_r2`
- Scope: Scope revision 2; exact source and test sets under `### Finder scope`
- Candidates: bare window ID may not satisfy `tmux switch-client -t`; window-fetch errors persist after zoom out until refresh; generated Graphify output has no product role.

#### structural in reverse

- State: terminal-success
- Executor: `/root/review_struct_reverse_r2`
- Scope: Scope revision 2; exact source and test sets under `### Finder scope`
- Candidates: hidden session preview refresh work; invalid and conflicting configured zoom keys; generated Graphify output is stale and machine-specific.

#### cleanup

- State: terminal-success
- Executor: `/root/review_cleanup_r2`
- Scope: Scope revision 2; exact production source set under `### Finder scope`; test-source cleanup excluded
- Candidates: duplicate cursor state; hidden session preview refresh work; per-redraw shortcut allocations; unused `Window.index`; per-line parser allocation; generated Graphify output.

### Candidate classification

- Confirmed: prior test-evidence gaps as BLK1-BLK5; custom key validation and conflicts as BLK7; plain search-key interception as BLK8; ID/name collision as BLK9; Enter/refresh retargeting as BLK10; generated Graphify output as NIT1; stale error guidance and overlay as NIT5; hidden session refresh work as NIT6; unused `Window.index` as NIT7.
- Plausible unresolved: bare window-ID `switch-client -t` compatibility as QST1.
- Retired: prior BLK6 because valid configured keys now reach the plugin environment and footer; prior NIT2-NIT4 because shared rendering, shared preview capture, and dead constructor rework is present.
- Refuted: removing `selected_window_index`; session and window cursors retain distinct selection state. Per-redraw static shortcut allocations and per-line parser allocation did not establish material cost beyond the reported refresh process fan-out.

### Intent and Plan conformance

- Research and Plan conformance: FAIL for BLK7-BLK10 and unresolved QST1. The main zoom, grid navigation, render, and model transitions otherwise follow REQ1-1 through REQ1-5.
- Scope: The Rust, plugin, and README changes are ticket-related. The generated `graphify-out/` tree remains unrelated volatile output under NIT1.
- Removed behavior: The vim `q` invariant is restored in source, subject to BLK4's missing gate evidence and BLK7's configurable conflict.
- Interfaces and callers: FAIL for the session identity lookup in BLK9, deferred target capture in BLK10, and target compatibility uncertainty in QST1.
- Directly affected documentation and guidance: FAIL for the custom-key contract in BLK7 and error guidance in NIT5.
- `git diff --check 4741e630226f5feee7f6ce94abc9c5d860b21a89`: PASS.

### Recorded gate evidence

- Implement status: COMPLETED.
- Latest scoped gate: `cargo test --test zoom_tests` — PASS, exit 0, elapsed 0m01s, log `/tmp/tmux_expose_gate_receipt_zoom_tests_rework2.log`.
- Earlier scoped gates: `cargo test --test zoom_tests` — PASS in Implement receipts 1 and 2.
- Gate scope: the recognized integration suite only; no receipt executes the added `src/model.rs`, `src/input.rs`, `src/tmux.rs`, or `src/ui.rs` unit tests.

### Good

- The shared grid calculator and shared card renderer cover both session and window cards.
- Window targets remain explicit in `selected_target`, and window refresh preserves a surviving selected window ID.
- Valid plugin zoom keys now reach the popup environment and dynamic footer.

### Failure history

none
