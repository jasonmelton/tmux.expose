# Branch Researcher
Status: DRAFT
Issue: 2
Branch: 2-feature-persistent-top-banner-for-command-embedding-ccmux
Date: 2026-08-22

## Purpose
Establish the smallest current, observable need for the branch to support a top banner for command embedding (`ccmux`).

## Requirements
### Points
- `REQ2-1` — Surface: mixed — Introduce a top-level horizontal layout split with a configurable height percentage (default `25%`) and a configurable command.
- `REQ2-2` — Surface: mixed — Render the configurable command inside the new top banner cleanly (preserving colors, formatting, etc.), with `tmux.expose` in the bottom split.

## Bugs
### Points
none

## Cosmetic
### Points
none

## Hazards
### Points
- `HAZ1` — Surface: non-code — Performance/Layout: A nested terminal multiplexer or Rust PTY integration could introduce layout constraints or event handling conflicts with the main `tmux.expose` UI.
- `HAZ2` — Surface: mixed — PTY implementation in Rust would require adding a heavy dependency to parse and render VT sequences.

## Questions
### Points
- `RESEARCH:QST1` — Surface: mixed — The issue gives two options for embedding `ccmux`: a PTY integration in Rust, or altering the startup script to use native `tmux` splits. Implementing a PTY in Rust requires significant new dependencies to handle VT sequence parsing and terminal emulation in `ratatui`. Altering the plugin script (`tmux.expose.tmux`) to spawn a nested tmux server inside the popup (e.g., `-E "tmux -L expose_popup new-session -d 'ccmux' \; split-window -v -p 75 'tmux-expose' \; attach"`) leverages native tmux for PTY rendering and preserves the popup workflow without needing heavy Rust dependencies.
  QUESTION: Should we implement a heavy PTY integration within the Rust app, or alter the `tmux.expose.tmux` plugin script to run a nested tmux session with standard splits inside the popup?

## Evidence
- **Relevant files:**
  - `src/main.rs`: CLI argument parsing for banner command and height options.
  - `src/ui.rs`: Layout calculation for the main grid.
  - `tmux.expose.tmux`: Plugin script creating the `display-popup` command.
  - `tests/plugin_entrypoint_test.sh`: Existing tests for `tmux.expose.tmux` arguments.
- **Analysis:**
  - The `tmux.expose.tmux` script currently passes CLI options to `tmux-expose` and runs it in a `display-popup` block.
  - A `display-popup` cannot be targeted by native `split-window` operations from the host tmux.
  - Thus, if the rendering relies on tmux (the fallback option), the popup itself must invoke a script that launches a nested tmux session to enable splits inside the popup.
