use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

use crate::{model::App, ui};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToggleKey {
    code: KeyCode,
    modifiers: KeyModifiers,
}

impl ToggleKey {
    pub fn from_tmux_key(value: &str) -> Option<Self> {
        let (mut modifiers, key) = if let Some(key) = value.strip_prefix("M-") {
            (KeyModifiers::ALT, key)
        } else if let Some(key) = value.strip_prefix("C-") {
            (KeyModifiers::CONTROL, key)
        } else {
            (KeyModifiers::NONE, value)
        };

        let code = match key {
            "Esc" => KeyCode::Esc,
            key if key.chars().count() == 1 => {
                let ch = key.chars().next()?;
                if ch.is_ascii_uppercase() {
                    modifiers.insert(KeyModifiers::SHIFT);
                }
                KeyCode::Char(ch)
            }
            _ => return None,
        };

        Some(Self { code, modifiers })
    }

    fn matches(self, key: KeyEvent) -> bool {
        self.code == key.code && self.modifiers == key.modifiers
    }
}

fn is_valid_zoom_key(key: ToggleKey, toggle_key: Option<ToggleKey>, vim_keys: bool) -> bool {
    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
        return false;
    }
    if key.code == KeyCode::Esc {
        return false;
    }
    if toggle_key == Some(key) {
        return false;
    }
    if vim_keys
        && key.modifiers == KeyModifiers::NONE
        && matches!(key.code, KeyCode::Char('h' | 'j' | 'k' | 'l' | '/' | 'q'))
    {
        return false;
    }
    true
}

pub fn resolve_zoom_key(
    raw: Option<&str>,
    toggle_key: Option<ToggleKey>,
    vim_keys: bool,
) -> (Option<ToggleKey>, String) {
    let default_key = ToggleKey::from_tmux_key("z").unwrap();
    let default_label = "z".to_string();

    if let Some(s) = raw {
        let trimmed = s.trim();
        if !trimmed.is_empty()
            && let Some(parsed) = ToggleKey::from_tmux_key(trimmed)
            && is_valid_zoom_key(parsed, toggle_key, vim_keys)
        {
            return (Some(parsed), trimmed.to_string());
        }
    }

    if is_valid_zoom_key(default_key, toggle_key, vim_keys) {
        (Some(default_key), default_label)
    } else {
        (None, String::new())
    }
}

pub fn handle_key(app: &mut App, key: KeyEvent, columns: usize) {
    handle_key_with_toggle(app, key, columns, None, ToggleKey::from_tmux_key("z"));
}

pub fn handle_key_with_toggle(
    app: &mut App,
    key: KeyEvent,
    columns: usize,
    toggle_key: Option<ToggleKey>,
    zoom_key: Option<ToggleKey>,
) {
    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
        app.should_quit = true;
        return;
    }

    if toggle_key.is_some_and(|toggle_key| toggle_key.matches(key)) {
        // A typeable toggle key — e.g. `e`/`E` from a `prefix + e` binding — must
        // stay usable as filter input while searching, so only close on it when a
        // query is not being entered. A modified toggle (e.g. M-e) can't be typed
        // into the filter, so it still closes from anywhere.
        let typeable_during_search = app.is_searching() && is_typeable_filter_key(key);
        if !typeable_during_search {
            app.should_quit = true;
            return;
        }
    }

    let is_zoom_match = zoom_key.is_some_and(|zk| zk.matches(key));

    if is_zoom_match {
        let typeable_during_vim_search =
            app.vim_keys && app.is_searching() && is_typeable_filter_key(key);
        if !typeable_during_vim_search {
            app.toggle_zoom();
            return;
        }
    }

    if app.is_searching() {
        handle_search_key(app, key, columns);
        return;
    }

    if app.vim_keys {
        handle_vim_normal_key(app, key, columns);
        return;
    }

    handle_default_key(app, key, columns);
}

/// Default mode: typing (when not zoomed) fuzzy-filters the list; the zoom key zooms into the selected session.
fn handle_default_key(app: &mut App, key: KeyEvent, columns: usize) {
    match (key.code, key.modifiers) {
        (KeyCode::Esc, _) => {
            if app.is_zoomed() {
                app.toggle_zoom();
            } else {
                app.should_quit = true;
            }
        }
        (KeyCode::Enter, _) => app.should_switch = true,
        (KeyCode::Left, _) => move_left(app, columns),
        (KeyCode::Right, _) => move_right(app, columns),
        (KeyCode::Up, _) => app.move_up(columns),
        (KeyCode::Down, _) => app.move_down(columns),
        (KeyCode::Char(ch), KeyModifiers::NONE | KeyModifiers::SHIFT) if !app.is_zoomed() => {
            push_filter_char(app, ch);
        }
        _ => {}
    }
}

/// Vim NORMAL mode: hjkl (and arrows) move the selection, `/` enters search.
fn handle_vim_normal_key(app: &mut App, key: KeyEvent, columns: usize) {
    match (key.code, key.modifiers) {
        (KeyCode::Char('q'), KeyModifiers::NONE) => app.should_quit = true,
        (KeyCode::Esc, _) => {
            if app.is_zoomed() {
                app.toggle_zoom();
            } else {
                app.should_quit = true;
            }
        }
        (KeyCode::Enter, _) => app.should_switch = true,
        (KeyCode::Char('/'), KeyModifiers::NONE) if !app.is_zoomed() => {
            app.start_search();
        }
        (KeyCode::Left, _) | (KeyCode::Char('h'), KeyModifiers::NONE) => move_left(app, columns),
        (KeyCode::Right, _) | (KeyCode::Char('l'), KeyModifiers::NONE) => move_right(app, columns),
        (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => app.move_up(columns),
        (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => app.move_down(columns),
        _ => {}
    }
}

pub fn handle_mouse(
    app: &mut App,
    mouse: MouseEvent,
    grid_area: Rect,
    min_card_width: Option<u16>,
    forced_columns: Option<usize>,
) {
    if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
        return;
    }

    let count = if app.is_zoomed() {
        app.visible_window_count()
    } else {
        app.visible_session_count()
    };

    let grid = ui::calculate_grid(grid_area, count, min_card_width, forced_columns);
    if let Some(index) = grid
        .cards
        .iter()
        .position(|card| contains(*card, mouse.column, mouse.row))
    {
        if app.is_zoomed() {
            app.selected_window_index = index;
        } else {
            app.selected_index = index;
        }
        app.should_switch = true;
    }
}

fn contains(area: Rect, x: u16, y: u16) -> bool {
    x >= area.x
        && x < area.x.saturating_add(area.width)
        && y >= area.y
        && y < area.y.saturating_add(area.height)
}

/// Keys the search filter accepts as text input — matching the `Char` arm in
/// `handle_search_key`. Uppercase keys arrive with `SHIFT`, so both lower- and
/// uppercase configured toggle keys count as typeable.
fn is_typeable_filter_key(key: KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char(_))
        && matches!(key.modifiers, KeyModifiers::NONE | KeyModifiers::SHIFT)
}

fn handle_search_key(app: &mut App, key: KeyEvent, columns: usize) {
    match (key.code, key.modifiers) {
        // In vim mode, Esc always returns to NORMAL rather than quitting.
        (KeyCode::Esc, _) if app.vim_keys => app.clear_search(),
        (KeyCode::Esc, _) if app.search_text().is_some_and(str::is_empty) => {
            app.should_quit = true;
        }
        (KeyCode::Esc, _) => app.clear_search(),
        (KeyCode::Enter, _) => app.should_switch = true,
        (KeyCode::Backspace, _) => app.pop_search_char(),
        (KeyCode::Left, _) => move_left(app, columns),
        (KeyCode::Right, _) => move_right(app, columns),
        (KeyCode::Up, _) => app.move_up(columns),
        (KeyCode::Down, _) => app.move_down(columns),
        (KeyCode::Char(ch), KeyModifiers::NONE | KeyModifiers::SHIFT) => push_filter_char(app, ch),
        _ => {}
    }
}

fn push_filter_char(app: &mut App, ch: char) {
    if !app.is_searching() {
        app.start_search();
    }
    app.push_search_char(ch);
}

fn move_left(app: &mut App, columns: usize) {
    let columns = columns.max(1);
    let selected_index = if app.is_zoomed() {
        app.selected_window_index
    } else {
        app.selected_index
    };
    if !selected_index.is_multiple_of(columns) {
        app.move_left();
    }
}

fn move_right(app: &mut App, columns: usize) {
    let columns = columns.max(1);
    let selected_index = if app.is_zoomed() {
        app.selected_window_index
    } else {
        app.selected_index
    };
    if selected_index % columns != columns - 1 {
        app.move_right();
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{
        KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    };
    use ratatui::layout::Rect;

    use super::*;
    use crate::model::{App, Session, Window};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn session(name: &str) -> Session {
        Session {
            id: format!("${name}"),
            name: name.to_string(),
            attached: false,
            window_count: 1,
            current_window: None,
            last_activity: None,
            preview: Vec::new(),
            preview_error: None,
        }
    }

    fn window(id: &str, name: &str, active: bool) -> Window {
        Window {
            id: id.to_string(),
            index: 0,
            name: name.to_string(),
            active,
            preview: Vec::new(),
            preview_error: None,
        }
    }

    #[test]
    fn arrow_keys_move_selection() {
        let mut app = App::new(vec![session("one"), session("two"), session("three")], None);

        handle_key(&mut app, key(KeyCode::Right), 2);
        assert_eq!(app.selected_index, 1);

        handle_key(&mut app, key(KeyCode::Down), 2);
        assert_eq!(app.selected_index, 2);

        handle_key(&mut app, key(KeyCode::Left), 2);
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn hjkl_filter_instead_of_moving() {
        let mut app = App::new(vec![session("one"), session("two"), session("three")], None);

        handle_key(&mut app, key(KeyCode::Char('h')), 2);

        assert_eq!(app.selected_index, 0);
        assert_eq!(app.search_text(), Some("h"));
    }

    fn vim_app(names: &[&str]) -> App {
        let mut app = App::new(names.iter().map(|name| session(name)).collect(), None);
        app.vim_keys = true;
        app
    }

    #[test]
    fn vim_hjkl_move_selection_without_filtering() {
        let mut app = vim_app(&["one", "two", "three", "four"]);

        handle_key(&mut app, key(KeyCode::Char('l')), 2); // right
        assert_eq!(app.selected_index, 1);

        handle_key(&mut app, key(KeyCode::Char('j')), 2); // down
        assert_eq!(app.selected_index, 3);

        handle_key(&mut app, key(KeyCode::Char('h')), 2); // left
        assert_eq!(app.selected_index, 2);

        handle_key(&mut app, key(KeyCode::Char('k')), 2); // up
        assert_eq!(app.selected_index, 0);

        // hjkl must not leak into the search filter while navigating.
        assert!(!app.is_searching());
    }

    #[test]
    fn vim_arrows_still_move_selection() {
        let mut app = vim_app(&["one", "two", "three"]);

        handle_key(&mut app, key(KeyCode::Right), 2);
        assert_eq!(app.selected_index, 1);

        handle_key(&mut app, key(KeyCode::Down), 2);
        assert_eq!(app.selected_index, 2);
    }

    #[test]
    fn vim_slash_enters_search_mode() {
        let mut app = vim_app(&["one", "two"]);

        handle_key(&mut app, key(KeyCode::Char('/')), 1);

        assert!(app.is_searching());
        assert_eq!(app.search_text(), Some(""));
    }

    #[test]
    fn vim_typing_in_search_filters_including_hjkl() {
        let mut app = vim_app(&["backend", "frontend", "hjkl-box"]);

        handle_key(&mut app, key(KeyCode::Char('/')), 1);
        handle_key(&mut app, key(KeyCode::Char('h')), 1);

        assert_eq!(app.search_text(), Some("h"));
        assert_eq!(app.selected_session().unwrap().name, "hjkl-box");
    }

    #[test]
    fn vim_esc_in_search_returns_to_normal_without_quitting() {
        let mut app = vim_app(&["one", "two"]);

        handle_key(&mut app, key(KeyCode::Char('/')), 1);
        handle_key(&mut app, key(KeyCode::Char('o')), 1);
        handle_key(&mut app, key(KeyCode::Esc), 1);

        assert!(!app.is_searching());
        assert!(!app.should_quit);
    }

    #[test]
    fn vim_plain_letters_do_not_filter_in_normal_mode() {
        let mut app = vim_app(&["alpha", "beta"]);

        handle_key(&mut app, key(KeyCode::Char('a')), 1);

        assert!(!app.is_searching());
        assert_eq!(app.search_text(), None);
    }

    #[test]
    fn vim_q_quits_in_normal_mode() {
        let mut app = vim_app(&["queue"]);

        handle_key(&mut app, key(KeyCode::Char('q')), 1);

        assert!(app.should_quit);
        assert!(!app.is_searching());
    }

    #[test]
    fn vim_q_quits_even_when_zoomed() {
        let mut app = vim_app(&["dev"]);
        app.toggle_zoom();
        assert!(app.is_zoomed());

        handle_key(&mut app, key(KeyCode::Char('q')), 1);

        assert!(app.should_quit);
    }

    #[test]
    fn vim_esc_quits_in_normal_mode_when_not_zoomed() {
        let mut app = vim_app(&["one"]);

        handle_key(&mut app, key(KeyCode::Esc), 1);

        assert!(app.should_quit);
    }

    #[test]
    fn vim_esc_zooms_out_when_zoomed_without_quitting() {
        let mut app = vim_app(&["one"]);
        app.toggle_zoom();
        assert!(app.is_zoomed());

        handle_key(&mut app, key(KeyCode::Esc), 1);

        assert!(!app.is_zoomed());
        assert!(!app.should_quit);
    }

    #[test]
    fn vim_enter_marks_app_for_switch() {
        let mut app = vim_app(&["one"]);

        handle_key(&mut app, key(KeyCode::Enter), 1);

        assert!(app.should_switch);
    }

    #[test]
    fn vim_ctrl_c_still_quits() {
        // Ctrl-C is handled before the mode dispatch, so it works in vim mode too.
        let mut app = vim_app(&["one"]);

        handle_key(
            &mut app,
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            1,
        );

        assert!(app.should_quit);
    }

    #[test]
    fn q_filters_instead_of_quitting() {
        let mut app = App::new(vec![session("queue")], None);

        handle_key_with_toggle(&mut app, key(KeyCode::Char('q')), 1, None, None);

        assert_eq!(app.search_text(), Some("q"));
        assert!(!app.should_quit);
    }

    #[test]
    fn enter_marks_app_for_switch() {
        let mut app = App::new(vec![session("one")], None);

        handle_key(&mut app, key(KeyCode::Enter), 1);

        assert!(app.should_switch);
    }

    #[test]
    fn ctrl_c_marks_app_for_exit() {
        let mut app = App::new(vec![session("one")], None);

        handle_key(
            &mut app,
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            1,
        );

        assert!(app.should_quit);
    }

    #[test]
    fn alt_e_does_not_quit_without_configured_toggle_key() {
        let mut app = App::new(vec![session("one")], None);

        handle_key(
            &mut app,
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::ALT),
            1,
        );

        assert!(!app.should_quit);
        assert!(!app.should_switch);
    }

    #[test]
    fn configured_alt_e_marks_app_for_exit() {
        let mut app = App::new(vec![session("one")], None);
        let toggle_key = ToggleKey::from_tmux_key("M-e");

        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::ALT),
            1,
            toggle_key,
            None,
        );

        assert!(app.should_quit);
        assert!(!app.should_switch);
    }

    #[test]
    fn plain_toggle_key_is_typeable_while_searching() {
        let mut app = App::new(vec![session("session")], None);
        let toggle_key = ToggleKey::from_tmux_key("s");
        app.start_search();
        app.push_search_char('e');

        // `s` is the toggle key, but while searching it must filter, not quit.
        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
            1,
            toggle_key,
            None,
        );

        assert!(!app.should_quit);
        assert_eq!(app.search_text(), Some("es"));
    }

    #[test]
    fn uppercase_toggle_key_is_typeable_while_searching() {
        let mut app = App::new(vec![session("Editor")], None);
        // `@tmux-expose-key 'E'` yields a toggle of Char('E') + SHIFT.
        let toggle_key = ToggleKey::from_tmux_key("E");
        app.start_search();

        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('E'), KeyModifiers::SHIFT),
            1,
            toggle_key,
            None,
        );

        assert!(!app.should_quit);
        assert_eq!(app.search_text(), Some("E"));
    }

    #[test]
    fn plain_toggle_key_still_quits_when_not_searching() {
        let mut app = App::new(vec![session("one")], None);
        let toggle_key = ToggleKey::from_tmux_key("s");

        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
            1,
            toggle_key,
            None,
        );

        assert!(app.should_quit);
    }

    #[test]
    fn modified_toggle_key_still_quits_while_searching() {
        let mut app = App::new(vec![session("one")], None);
        let toggle_key = ToggleKey::from_tmux_key("M-e");
        app.start_search();
        app.push_search_char('o');

        // A modified chord can't be typed into the filter, so it still closes.
        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::ALT),
            1,
            toggle_key,
            None,
        );

        assert!(app.should_quit);
    }

    #[test]
    fn left_click_on_session_marks_it_for_switch() {
        let mut app = App::new(vec![session("one"), session("two"), session("three")], None);
        let mouse = MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 40,
            row: 1,
            modifiers: KeyModifiers::NONE,
        };

        handle_mouse(&mut app, mouse, Rect::new(0, 0, 100, 20), None, Some(3));

        assert_eq!(app.selected_index, 1);
        assert!(app.should_switch);
        assert!(!app.should_quit);
    }

    #[test]
    fn characters_start_filtering_without_slash() {
        let mut app = App::new(
            vec![session("backend"), session("frontend"), session("database")],
            None,
        );

        handle_key(&mut app, key(KeyCode::Char('f')), 1);

        assert!(app.is_searching());
        assert_eq!(app.search_text(), Some("f"));
        assert_eq!(app.visible_session_count(), 1);
        assert_eq!(app.selected_session().unwrap().name, "frontend");
    }

    #[test]
    fn slash_filters_instead_of_starting_empty_search() {
        let mut app = App::new(vec![session("docs/api")], None);

        handle_key(&mut app, key(KeyCode::Char('/')), 1);

        assert_eq!(app.search_text(), Some("/"));
        assert_eq!(app.visible_session_count(), 1);
        assert_eq!(app.selected_session().unwrap().name, "docs/api");
    }

    #[test]
    fn characters_continue_filtering_while_searching() {
        let mut app = App::new(
            vec![session("backend"), session("frontend"), session("database")],
            None,
        );

        handle_key(&mut app, key(KeyCode::Char('f')), 1);
        handle_key(&mut app, key(KeyCode::Char('r')), 1);

        assert_eq!(app.search_text(), Some("fr"));
        assert_eq!(app.visible_session_count(), 1);
        assert_eq!(app.selected_session().unwrap().name, "frontend");
    }

    #[test]
    fn esc_clears_search_before_quitting() {
        let mut app = App::new(vec![session("one")], None);

        handle_key(&mut app, key(KeyCode::Char('o')), 1);
        handle_key(&mut app, key(KeyCode::Esc), 1);

        assert!(!app.is_searching());
        assert!(!app.should_quit);
    }

    #[test]
    fn esc_quits_when_search_query_is_empty() {
        let mut app = App::new(vec![session("one")], None);

        handle_key(&mut app, key(KeyCode::Char('o')), 1);
        handle_key(&mut app, key(KeyCode::Backspace), 1);
        handle_key(&mut app, key(KeyCode::Esc), 1);

        assert!(app.should_quit);
    }

    #[test]
    fn backspace_edits_search_query() {
        let mut app = App::new(vec![session("frontend")], None);

        handle_key(&mut app, key(KeyCode::Char('f')), 1);
        handle_key(&mut app, key(KeyCode::Backspace), 1);

        assert_eq!(app.search_text(), Some(""));
    }

    #[test]
    fn horizontal_navigation_clamps_at_row_edges() {
        let mut app = App::new(
            vec![
                session("one"),
                session("two"),
                session("three"),
                session("four"),
            ],
            None,
        );
        app.selected_index = 2;

        handle_key(&mut app, key(KeyCode::Right), 3);
        assert_eq!(app.selected_index, 2);

        app.selected_index = 3;
        handle_key(&mut app, key(KeyCode::Left), 3);
        assert_eq!(app.selected_index, 3);
    }

    #[test]
    fn zoom_key_toggles_zoom_state() {
        let mut app = App::new(vec![session("dev")], None);
        assert!(!app.is_zoomed());

        handle_key(&mut app, key(KeyCode::Char('z')), 1);
        assert!(app.is_zoomed());

        handle_key(&mut app, key(KeyCode::Char('z')), 1);
        assert!(!app.is_zoomed());
    }

    #[test]
    fn esc_zooms_out_when_zoomed_without_quitting() {
        let mut app = App::new(vec![session("dev")], None);
        app.toggle_zoom();
        assert!(app.is_zoomed());

        handle_key(&mut app, key(KeyCode::Esc), 1);
        assert!(!app.is_zoomed());
        assert!(!app.should_quit);
    }

    #[test]
    fn zoomed_grid_navigation_clamps_at_row_edges() {
        let mut app = App::new(vec![session("dev")], None);
        app.toggle_zoom();
        app.set_windows_for_zoomed_session(vec![
            window("@1", "one", false),
            window("@2", "two", false),
            window("@3", "three", false),
            window("@4", "four", false),
        ]);
        app.selected_window_index = 2;

        handle_key(&mut app, key(KeyCode::Right), 3);
        assert_eq!(app.selected_window_index, 2);

        app.selected_window_index = 3;
        handle_key(&mut app, key(KeyCode::Left), 3);
        assert_eq!(app.selected_window_index, 3);
    }

    #[test]
    fn type_filter_and_press_default_zoom_key_zooms_into_session() {
        let mut app = App::new(vec![session("backend"), session("frontend")], None);

        // In default mode, typing 'f' starts search and narrows selection to "frontend"
        handle_key(&mut app, key(KeyCode::Char('f')), 2);
        assert!(app.is_searching());
        assert_eq!(app.selected_session().unwrap().name, "frontend");

        // Pressing default zoom key 'z' must zoom into the selected session rather than appending 'z'
        handle_key(&mut app, key(KeyCode::Char('z')), 2);
        assert!(app.is_zoomed());
        assert!(!app.is_searching());
        assert_eq!(app.zoomed_session().unwrap().name, "frontend");
    }

    #[test]
    fn modified_zoom_key_from_search_zooms_and_single_esc_exits() {
        let mut app = App::new(vec![session("dev")], None);
        let zoom_key = ToggleKey::from_tmux_key("M-z");

        // Type a search query
        handle_key_with_toggle(&mut app, key(KeyCode::Char('d')), 1, None, zoom_key);
        assert!(app.is_searching());

        // Trigger zoom with modified key
        handle_key_with_toggle(
            &mut app,
            KeyEvent::new(KeyCode::Char('z'), KeyModifiers::ALT),
            1,
            None,
            zoom_key,
        );
        assert!(app.is_zoomed());
        assert!(!app.is_searching());

        // Single Esc zooms out
        handle_key_with_toggle(&mut app, key(KeyCode::Esc), 1, None, zoom_key);
        assert!(!app.is_zoomed());
        assert!(!app.should_quit);
    }

    #[test]
    fn resolve_zoom_key_validates_and_handles_conflicts() {
        let default_z = ToggleKey::from_tmux_key("z");

        // Valid custom key
        let (key_x, label_x) = resolve_zoom_key(Some("x"), None, false);
        assert_eq!(key_x, ToggleKey::from_tmux_key("x"));
        assert_eq!(label_x, "x");

        // Valid modified custom key
        let (key_mz, label_mz) = resolve_zoom_key(Some("M-z"), None, false);
        assert_eq!(key_mz, ToggleKey::from_tmux_key("M-z"));
        assert_eq!(label_mz, "M-z");

        // Unsupported key string falls back to 'z'
        let (key_f1, label_f1) = resolve_zoom_key(Some("F1"), None, false);
        assert_eq!(key_f1, default_z);
        assert_eq!(label_f1, "z");

        // Esc is reserved and falls back to 'z'
        let (key_esc, label_esc) = resolve_zoom_key(Some("Esc"), None, false);
        assert_eq!(key_esc, default_z);
        assert_eq!(label_esc, "z");

        // C-c is reserved and falls back to 'z'
        let (key_cc, label_cc) = resolve_zoom_key(Some("C-c"), None, false);
        assert_eq!(key_cc, default_z);
        assert_eq!(label_cc, "z");

        // Conflict with toggle_key falls back to 'z'
        let toggle = ToggleKey::from_tmux_key("s");
        let (key_s, label_s) = resolve_zoom_key(Some("s"), toggle, false);
        assert_eq!(key_s, default_z);
        assert_eq!(label_s, "z");

        // Conflict with toggle key 'z' causes fallback 'z' to be rejected
        let toggle_z = ToggleKey::from_tmux_key("z");
        let (key_tz, label_tz) = resolve_zoom_key(None, toggle_z, false);
        assert_eq!(key_tz, None);
        assert_eq!(label_tz, "");

        // Conflict with vim normal mode keys in vim mode falls back to 'z'
        for k in &["h", "j", "k", "l", "/", "q"] {
            let (key_vim, label_vim) = resolve_zoom_key(Some(k), None, true);
            assert_eq!(key_vim, default_z, "key {k} should fall back to z");
            assert_eq!(label_vim, "z");
        }

        // Modified keys in vim mode (e.g. M-h) are allowed
        let (key_mh, label_mh) = resolve_zoom_key(Some("M-h"), None, true);
        assert_eq!(key_mh, ToggleKey::from_tmux_key("M-h"));
        assert_eq!(label_mh, "M-h");

        // 'q' without vim mode is valid
        let (key_q_normal, label_q_normal) = resolve_zoom_key(Some("q"), None, false);
        assert_eq!(key_q_normal, ToggleKey::from_tmux_key("q"));
        assert_eq!(label_q_normal, "q");

        // None falls back to 'z'
        let (key_none, label_none) = resolve_zoom_key(None, None, false);
        assert_eq!(key_none, default_z);
        assert_eq!(label_none, "z");
    }
}
