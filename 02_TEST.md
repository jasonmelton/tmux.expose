# Branch Test

**Issue:** 1
**Branch:** 1-feature-drill-down-zoom-into-session-windows
**Date:** 2026-08-22
**Stack:** rust
**Runners:** cargo test --test zoom_tests
**Status:** COMPLETED

## Purpose

Define logical test coverage, rejected test considerations, and initial Red evidence for Issue 1 (drill-down zoom into session windows).

## Summary

Initial Red result settled with EXPECTED_RED. Test suite `tests/zoom_tests.rs` defines coverage for zoom-in/out key transitions, window model parsing, window attachment on Enter, grid navigation across windows, and zoom state preservation during background refresh. The test run fails compilation as expected due to missing production interfaces.

## Points

### TST1
- **Requirements:** REQ1-1, REQ1-4
- **Surface:** mixed
- **Path:** `tests/zoom_tests.rs`
- **Framework Test Name:** `test_req1_1_press_zoom_key_zooms_into_selected_session`, `test_req1_4_zoom_key_or_esc_zooms_out_to_session_view_without_quitting`
- **Classification:** CORE_GREEN
- **Observable Outcome:** Pressing `z` from session view toggles the app into zoomed window view for the selected session. Pressing `z` again or `Esc` while zoomed returns to session view without setting `should_quit`.

### TST2
- **Requirements:** REQ1-2
- **Surface:** mixed
- **Path:** `tests/zoom_tests.rs`
- **Framework Test Name:** `test_req1_2_parses_window_lines_from_tmux_format`, `test_req1_2_zoomed_view_provides_windows_for_selected_session`
- **Classification:** CORE_GREEN
- **Observable Outcome:** `parse_windows` correctly extracts window IDs, names, and active states from tmux output format. The zoomed view exposes window count and selected window model for the active session.

### TST3
- **Requirements:** REQ1-3
- **Surface:** mixed
- **Path:** `tests/zoom_tests.rs`
- **Framework Test Name:** `test_req1_3_enter_on_zoomed_window_targets_specific_window_for_switch`
- **Classification:** CORE_GREEN
- **Observable Outcome:** Pressing `Enter` while focused on a window card in zoomed view sets `app.should_switch = true` and updates `selected_target` to the window identifier.

### TST4
- **Requirements:** REQ1-5
- **Surface:** mixed
- **Path:** `tests/zoom_tests.rs`
- **Framework Test Name:** `test_req1_5_zoomed_window_grid_navigation_clamps_at_edges`
- **Classification:** CORE_GREEN
- **Observable Outcome:** Directional navigation (Left/Right/Up/Down) within the zoomed window view moves selection across window cards and clamps at grid boundaries.

### TST5
- **Requirements:** HAZ1-1
- **Surface:** mixed
- **Path:** `tests/zoom_tests.rs`
- **Framework Test Name:** `test_haz1_1_refresh_preserves_zoomed_state_and_session`
- **Classification:** CORE_GREEN
- **Observable Outcome:** Background session refreshes via `replace_sessions` preserve the active zoomed mode and active session context.

### NOT1
- **Requirements:** REQ1-1
- **Surface:** non-code
- **Consideration:** Custom configurable keybind overrides for zoom (e.g. `@tmux-expose-zoom-key` tmux option).
- **Reason:** Default `z` key navigation fulfills the primary requirement contract; custom key option parsing is not requested in project input or dispatch.

### NOT2
- **Requirements:** REQ1-2
- **Surface:** test
- **Consideration:** Low-level ANSI escape code parsing and color rendering of individual window previews.
- **Reason:** Preview ANSI processing is shared with session cards and already proven by `ui::tests::ansi_foreground_colors_become_styled_spans` and `ui::tests::ansi_truncation_counts_only_visible_characters`.

### NOT3
- **Requirements:** REQ1-5
- **Surface:** test
- **Consideration:** Raw layout math for grid slot aspect ratios and dimension distribution with various window card counts.
- **Reason:** Layout calculation math in `calculate_grid` accepts arbitrary item counts and is already proven by `ui::tests::grid_fits_default_cards_to_available_screen_space`, `ui::tests::grid_keeps_wide_screens_balanced_by_default`, and `ui::tests::thumbnail_width_uses_as_many_min_width_columns_as_fit`.

## Questions

none

## Evidence

### Cited Unchanged Coverage
- `ui::tests::ansi_foreground_colors_become_styled_spans` (`src/ui.rs:L808-817`): Verifies ANSI escape sequence handling for preview rendering.
- `ui::tests::ansi_truncation_counts_only_visible_characters` (`src/ui.rs:L820-824`): Verifies ANSI string truncation width calculation.
- `ui::tests::grid_fits_default_cards_to_available_screen_space` (`src/ui.rs:L575-585`): Proves item-count agnostic grid layout calculations.
- `ui::tests::thumbnail_width_uses_as_many_min_width_columns_as_fit` (`src/ui.rs:L615-623`): Proves automatic column calculation with minimum card width.

### Gate receipt 1
- Owner: TEST
- Command: cargo test --test zoom_tests
- Authorization: "Operation: RUN"
- Elapsed: 0m01s
- Result: FAIL
- Exit: 101
- Log: target/debug/deps/zoom_tests

### Failure Owners
- `EXPECTED_RED` (tests/zoom_tests.rs): Missing `Window` model, `parse_windows`, and `App` zoom methods (`is_zoomed`, `zoomed_session`, `set_windows_for_zoomed_session`, `visible_window_count`, `selected_window`, `selected_target`).

### Uncovered Authorized Risks
none
