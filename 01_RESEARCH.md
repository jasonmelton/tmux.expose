# Branch Research

**Issue:** 1
**Branch:** 1-feature-drill-down-zoom-into-session-windows
**Date:** 2026-08-22
**Status:** COMPLETED

## Requirements

### Points

#### REQ1-1
**Surface:** mixed
**Description:** Configurable Zoom Keybind
**Evidence:**
- `src/input.rs:L46`: Inputs currently dispatch via `handle_key_with_toggle` or mode-specific handlers. A new key binding (default `z`) is required to transition the `App` state to a zoomed view.
- `src/model.rs:L21`: The `App` state needs to differentiate between viewing the session list and the zoomed window list.
**Expected Outcome:** Pressing the zoom key (default `z`) from the session view transitions the app to the zoomed window view for the currently selected session.

#### REQ1-2
**Surface:** mixed
**Description:** Fetch and Display Window Cards
**Evidence:**
- `src/tmux.rs:L14`: `list_sessions` parses tmux output. A new `list_windows(session_id: &str)` function is needed using `tmux list-windows` to produce `Window` models.
- `src/tmux.rs:L96`: `capture_session_preview` uses `tmux capture-pane`. We need to capture the preview for individual windows (e.g. `capture_window_preview(window_id)`).
**Expected Outcome:** When zoomed, the view renders window cards using fetched `tmux list-windows` data and `tmux capture-pane` previews, filtering only to the zoomed session's windows.

#### REQ1-3
**Surface:** mixed
**Description:** Attach to Window on Enter
**Evidence:**
- `src/input.rs:L73`: Pressing `Enter` triggers `app.should_switch = true`.
- `src/main.rs`: Attaching to a session currently happens by getting the selected session id. Attaching to a specific window will require updating the target identifier passed to `tmux switch-client` or `tmux attach-session`.
**Expected Outcome:** Pressing `Enter` on a zoomed window card attaches directly to that specific window.

#### REQ1-4
**Surface:** mixed
**Description:** Zoom Out Keybind
**Evidence:**
- `src/input.rs:L72`: `Esc` currently sets `app.should_quit = true`.
**Expected Outcome:** When in the zoomed window view, pressing the zoom key (e.g., `z`) or `Esc` transitions the app back to the session view instead of quitting.

#### REQ1-5
**Surface:** mixed
**Description:** Dynamic Grid Calculation for Windows
**Evidence:**
- `src/ui.rs:L283`: `calculate_grid` currently computes layout independent of `Session` models by accepting `item_count: usize`.
- `src/ui.rs:L116`: `render_card` will need to accept `Window` data or a generalized trait representing a renderable card.
**Expected Outcome:** The grid layout behaves identically for window cards as it does for session cards.

## Bugs

none

## Cosmetic / Design-Only

none

## Hazards

### Points

#### HAZ1-1
**Surface:** mixed
**Description:** Background Data Refresh During Zoom
**Evidence:**
- The background thread periodically refreshes sessions. If the user is zoomed in and a refresh occurs, the `App` must preserve the zoomed state and re-fetch window previews for the zoomed session, rather than resetting to the session view or dropping window state.

## Questions

none

## Evidence
- `src/model.rs` and `src/input.rs` were audited for current session state and keyboard shortcuts.
- `src/ui.rs` grid functions like `calculate_grid` demonstrate they are decoupled from domain models (taking `item_count: usize`), making it viable to reuse the identical layout engine for both sessions and windows.
- `src/tmux.rs` shows existing `tmux list-sessions` and `capture-pane` functionality, confirming the pattern to adapt for `tmux list-windows`.

Live requirements: 5
