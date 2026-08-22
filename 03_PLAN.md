# Branch Plan

**Issue:** 1
**Branch:** 1-feature-drill-down-zoom-into-session-windows
**Date:** 2026-08-22
**Status:** COMPLETED

## Purpose

Implement zooming into a session's windows. This allows users to drill down into a specific session's windows, view previews for each window, and directly switch to a window.

## Summary

Implement: required

Tasks:
- PHS1: Data Fetching and Models
  - TSK1: Define Window and update App state
  - TSK2: Implement tmux list-windows and parsing
  - TSK3: Implement capture_window_preview
- PHS2: Application Input and Navigation
  - TSK1: Update App navigation for zoomed state
  - TSK2: Handle zoom keybinds and toggle logic
  - TSK3: Handle Esc key in zoomed state
  - TSK4: Update attach logic on Enter
  - TSK5: Preserve zoomed state during background refresh
- PHS3: User Interface
  - TSK1: Generalize render_card
  - TSK2: Conditionally render window grid

Questions:
- none

## Points

### PHS1: Data Fetching and Models

#### TSK1
- **Tags:** REQ1-2, HAZ1-1
- **Files:** `src/model.rs`
- **Existing Logic:** `Session` struct defines fields for a tmux session. `App` manages session state.
- **Approach:** Add a `Window` struct mirroring `Session`: `id`, `name`, `attached`, `preview`, `preview_error`. Add to `App`: `pub zoomed_session: Option<String>`, `pub windows: Vec<Window>`, `pub selected_window_index: usize`. Add `app.toggle_zoom()` and `app.is_zoomed()`.
- **Outcome:** Domain models support window-level data and app state tracks zoomed views.

#### TSK2
- **Tags:** REQ1-2
- **Files:** `src/tmux.rs`
- **Existing Logic:** `list_sessions` uses `tmux list-sessions` to fetch sessions.
- **Approach:** Add `list_windows(session_id: &str) -> Result<Vec<Window>>`. Run `tmux list-windows -t <session_id> -F "#{window_id}:#{window_name}:#{window_active}"`. Parse output. For each window, call `capture_window_preview`.
- **Outcome:** The app can fetch a session's windows and their states.

#### TSK3
- **Tags:** REQ1-2
- **Files:** `src/tmux.rs`
- **Existing Logic:** `capture_session_preview` captures a pane.
- **Approach:** Add `capture_window_preview(window_target: &str, max_lines: usize) -> Result<Vec<String>>` using `tmux capture-pane -e -p -t <window_target>`.
- **Outcome:** The app can fetch previews for specific windows.

### PHS2: Application Input and Navigation

#### TSK1
- **Tags:** REQ1-2, REQ1-4
- **Files:** `src/model.rs`
- **Existing Logic:** `App` provides `move_left`, `move_right`, `move_up`, `move_down` which manage `selected_index`.
- **Approach:** Update navigation methods in `App` to modify `selected_window_index` when `zoomed_session` is `Some`.
- **Outcome:** Keyboard navigation works seamlessly in both session and window views.

#### TSK2
- **Tags:** REQ1-1
- **Files:** `src/input.rs`, `src/main.rs`
- **Existing Logic:** `handle_key_with_toggle` takes an optional `toggle_key`.
- **Approach:** Update `handle_key_with_toggle` to accept an additional `zoom_key: Option<ToggleKey>`. In `main.rs`, parse `TMUX_EXPOSE_ZOOM_KEY` env var (defaulting to `z`) and pass it down. If the key matches `zoom_key`, trigger `app.toggle_zoom()`.
- **Outcome:** Users can toggle zoom with a configurable keybind.

#### TSK3
- **Tags:** REQ1-4
- **Files:** `src/input.rs`
- **Existing Logic:** Pressing `Esc` sets `should_quit = true` or clears the search filter.
- **Approach:** Update `handle_default_key`, `handle_vim_normal_key`, and `handle_search_key`. If `app.is_zoomed()` and `Esc` is pressed (and search is empty/cleared), call `app.toggle_zoom()` instead of setting `should_quit = true`.
- **Outcome:** Pressing `Esc` zooms out instead of quitting the application.

#### TSK4
- **Tags:** REQ1-3
- **Files:** `src/main.rs`
- **Existing Logic:** On `app.should_switch`, it switches to `app.selected_session()`.
- **Approach:** When `app.should_switch` is true, check if `app.is_zoomed()`. If zoomed, retrieve the selected window and `switch_client` to `window.id`. Otherwise, switch to the selected session.
- **Outcome:** Pressing `Enter` attaches directly to the zoomed window.

#### TSK5
- **Tags:** HAZ1-1
- **Files:** `src/main.rs`
- **Existing Logic:** The background loop refreshes sessions via `tmux::list_sessions_skipping_preview_for`.
- **Approach:** In the refresh loop, if `app.zoomed_session` is `Some(session_id)`, also fetch `list_windows(&session_id)`. Update `app.windows` and preserve existing window previews to avoid flashing.
- **Outcome:** Background refreshes preserve the zoomed state and window previews.

### PHS3: User Interface

#### TSK1
- **Tags:** REQ1-5
- **Files:** `src/ui.rs`
- **Existing Logic:** `render_card` accepts `&Session`.
- **Approach:** Modify `render_card` to accept extracted fields instead of `&Session` directly, or define a `RenderableCard` trait. The fields needed are `name`, `attached`, `window_count` (optional), `current_window` (optional), `preview`, and `preview_error`.
- **Outcome:** The rendering function is decoupled from the `Session` domain model, allowing reuse for `Window`.

#### TSK2
- **Tags:** REQ1-2, REQ1-5
- **Files:** `src/ui.rs`
- **Existing Logic:** `render_grid` iterates over `app.visible_sessions()`.
- **Approach:** Update `render_grid` to check `app.is_zoomed()`. If zoomed, calculate grid for `app.windows.len()`, and map `app.windows` to the decoupled `render_card` interface. Otherwise, render sessions as before.
- **Outcome:** The UI smoothly transitions between session and window grids using the same layout calculation.

## Evidence
- `src/ui.rs` allows passing decoupled data to `render_card`.
- `tmux list-windows -F` allows retrieving window info just like sessions.
- `capture_window_preview` will work exactly like `capture_session_preview` with a window target.
