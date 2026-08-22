# Branch Analysis

**Issue:** 1
**Branch:** 1-feature-drill-down-zoom-into-session-windows
**Base:** 4741e630226f5feee7f6ce94abc9c5d860b21a89
**Date:** 2026-08-22
**Tools:** git
**Status:** COMPLETED

## Purpose

Run the supplied analyzer lanes against the branch change scope and record analyzer-owned findings or safe corrections.

## Summary

Analysis: not applicable. No analyzer lanes were supplied by `.project-context/05_ANALYSIS.md` or developer direction, so the role authorized no analyzer command or production mutation.

- Analyzer Results: not applicable
- Applied IDs: none
- Unapplied IDs: none
- Blocking IDs: none

## Points

none

## Evidence

### Change Scope

- Merge base: `4741e630226f5feee7f6ce94abc9c5d860b21a89`
- Tracked-scope command: `git diff --name-only "$base"`
- Untracked-scope command: `git ls-files --others --exclude-standard`
- Tracked files: `src/input.rs`, `src/main.rs`, `src/model.rs`, `src/tmux.rs`, `src/ui.rs`
- Untracked files: `graphify-out/.graphify_labels.json`, `graphify-out/.graphify_python`, `graphify-out/.graphify_root`, `graphify-out/GRAPH_REPORT.md`, `graphify-out/cache/ast/v0.9.32/4544322e308ffa07a4322ede79a08a38182cbc337cbb7610b92398b1dfba8308.json`, `graphify-out/cache/ast/v0.9.32/666693ccd4c68597d9f8984a91aa95d17f74e031cf4cf016098bbc6a99cb7af1.json`, `graphify-out/cache/ast/v0.9.32/68891e4eb9894d30524ebedc1d3863b83c1cb3490484e3afef61d4b2d43e7acb.json`, `graphify-out/cache/ast/v0.9.32/6db1d429e19aa6b2f3e1b81dd52fdbf95c8c03e3940acec99f4adcc84d7d41b6.json`, `graphify-out/cache/ast/v0.9.32/d474a558379a40ed8755006b79c2db6d5002e40f97bdd5e0d327ad497add5cbb.json`, `graphify-out/cache/ast/v0.9.32/d6edc9cef721f85b7ac6f3a94af9fc341e3139c8ac542efbaf666595bc84cba2.json`, `graphify-out/cache/ast/v0.9.32/d8cc86535d54b0fa759e7450e90cc2ce17866a36548f815560638cf4629c25cd.json`, `graphify-out/cache/semantic/pd5fd89c46bb5/0e15e45fee3918f5cdcf0dc12b5e3a6599a7c7c8619f0ab1833964580449a17c.json`, `graphify-out/cache/semantic/pd5fd89c46bb5/1dee00906535fe9177c47cb7c0bfe6364613d5ccf99bf8b5a8848ff08829b4cc.json`, `graphify-out/cache/semantic/pd5fd89c46bb5/5e7fadc3875ddff89fd99cecaddf053296b9f6185a9bee45d62df358bbd8c74d.json`, `graphify-out/cache/semantic/pd5fd89c46bb5/631c38a46196c78da6ede3e99e067e6156bb6bd6a6e2372a8949e194c5fd005f.json`, `graphify-out/cache/semantic/pd5fd89c46bb5/6692972132ed85d94c4277431ed0e0c70ead0e129d85a1cd2d47c045c9888f76.json`, `graphify-out/cache/stat-index.json`, `graphify-out/cost.json`, `graphify-out/graph.html`, `graphify-out/graph.json`, `graphify-out/manifest.json`, `tests/zoom_tests.rs`

### Analyzer Lanes

- Supplied lanes: none
- Result: not applicable; no analyzer command was authorized or run

### Applied Changes

none

### Decision Families

none
