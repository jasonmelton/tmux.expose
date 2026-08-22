use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tmux_expose::{
    input::handle_key,
    model::{App, Session, Window},
    tmux::parse_windows,
};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn sample_session(name: &str) -> Session {
    Session {
        id: format!("${name}"),
        name: name.to_string(),
        attached: false,
        window_count: 2,
        current_window: Some("editor".to_string()),
        last_activity: None,
        preview: Vec::new(),
        preview_error: None,
    }
}

fn sample_window(id: &str, name: &str, index: u32) -> Window {
    Window {
        id: id.to_string(),
        name: name.to_string(),
        index,
        active: index == 0,
        last_activity: None, panes: Vec::new(),
        preview_error: None,
    }
}

#[test]
fn test_req1_1_press_zoom_key_zooms_into_selected_session() {
    let mut app = App::new(
        vec![sample_session("dev"), sample_session("logs")],
        Some("dev".to_string()),
    );

    assert!(!app.is_zoomed());
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);

    assert!(app.is_zoomed());
    assert_eq!(app.zoomed_session().map(|s| s.name.as_str()), Some("dev"));
}

#[test]
fn test_req1_4_zoom_key_or_esc_zooms_out_to_session_view_without_quitting() {
    let mut app = App::new(
        vec![sample_session("dev"), sample_session("logs")],
        Some("dev".to_string()),
    );

    handle_key(&mut app, key(KeyCode::Char(' ')), 2);
    assert!(app.is_zoomed());

    // Pressing 'z' again toggles back out to session view
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);
    assert!(!app.is_zoomed());
    assert!(!app.should_quit);

    // Zoom in and verify Esc also zooms out without quitting
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);
    assert!(app.is_zoomed());

    handle_key(&mut app, key(KeyCode::Esc), 2);
    assert!(!app.is_zoomed());
    assert!(!app.should_quit);
}

#[test]
fn test_req1_2_parses_window_lines_from_tmux_format() {
    let output = "@1:editor:1:3:1710000000\n@2:terminal:0:1:1710000100\n";
    let windows = parse_windows(output);

    assert_eq!(windows.len(), 2);
    assert_eq!(windows[0].id, "@1");
    assert_eq!(windows[0].name, "editor");
    assert!(windows[0].active);
    assert_eq!(windows[1].id, "@2");
    assert_eq!(windows[1].name, "terminal");
    assert!(!windows[1].active);
}

#[test]
fn test_req1_2_zoomed_view_provides_windows_for_selected_session() {
    let mut app = App::new(
        vec![sample_session("dev"), sample_session("logs")],
        Some("dev".to_string()),
    );

    let windows = vec![
        sample_window("@1", "code", 0),
        sample_window("@2", "shell", 1),
    ];
    app.set_windows_for_zoomed_session(windows);
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);

    assert_eq!(app.visible_window_count(), 2);
    assert_eq!(app.selected_window().map(|w| w.name.as_str()), Some("code"));
}

#[test]
fn test_req1_3_enter_on_zoomed_window_targets_specific_window_for_switch() {
    let mut app = App::new(
        vec![sample_session("dev"), sample_session("logs")],
        Some("dev".to_string()),
    );

    let windows = vec![
        sample_window("@1", "code", 0),
        sample_window("@2", "shell", 1),
    ];
    app.set_windows_for_zoomed_session(windows);
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);
    handle_key(&mut app, key(KeyCode::Right), 2);

    handle_key(&mut app, key(KeyCode::Enter), 2);
    assert!(app.should_switch);
    assert_eq!(app.selected_target().as_deref(), Some("@2"));
}

#[test]
fn test_req1_5_zoomed_window_grid_navigation_clamps_at_edges() {
    let mut app = App::new(vec![sample_session("dev")], Some("dev".to_string()));

    let windows = vec![
        sample_window("@1", "w1", 0),
        sample_window("@2", "w2", 1),
        sample_window("@3", "w3", 2),
    ];
    app.set_windows_for_zoomed_session(windows);
    handle_key(&mut app, key(KeyCode::Char(' ')), 2);

    assert_eq!(app.selected_index, 0);
    handle_key(&mut app, key(KeyCode::Right), 2);
    assert_eq!(app.selected_index, 1);
    handle_key(&mut app, key(KeyCode::Down), 2);
    assert_eq!(app.selected_index, 2);
    handle_key(&mut app, key(KeyCode::Down), 2);
    assert_eq!(app.selected_index, 2);
    handle_key(&mut app, key(KeyCode::Up), 2);
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_haz1_1_refresh_preserves_zoomed_state_and_session() {
    let mut app = App::new(
        vec![sample_session("dev"), sample_session("logs")],
        Some("dev".to_string()),
    );

    handle_key(&mut app, key(KeyCode::Char(' ')), 2);
    assert!(app.is_zoomed());

    let refreshed_sessions = vec![sample_session("dev"), sample_session("logs")];
    app.replace_sessions(refreshed_sessions);

    assert!(app.is_zoomed());
    assert_eq!(app.zoomed_session().map(|s| s.name.as_str()), Some("dev"));
}
