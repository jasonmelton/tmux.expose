#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub attached: bool,
    pub window_count: u32,
    pub current_window: Option<String>,
    pub last_activity: Option<String>,
    pub preview: Vec<String>,
    pub preview_error: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PanePreview {
    pub id: String,
    pub active: bool,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: String,
    pub index: u32,
    pub name: String,
    pub active: bool,
    pub last_activity: Option<String>,
    pub panes: Vec<PanePreview>,
    pub preview_error: Option<String>,
}

#[derive(Debug)]
pub struct App {
    pub sessions: Vec<Session>,
    pub selected_index: usize,
    pub current_session_name: Option<String>,
    pub should_quit: bool,
    pub should_switch: bool,
    pub error: Option<String>,
    /// When true, the picker uses modal vim navigation (hjkl to move, `/` to search).
    pub vim_keys: bool,
    pub zoom_key: String,
    search_query: Option<String>,
    pub zoomed_session: Option<String>,
    pub windows: Vec<Window>,
    pub selected_window_index: usize,
}

impl App {
    pub fn new(sessions: Vec<Session>, current_session_name: Option<String>) -> Self {
        let selected_index = current_session_name
            .as_ref()
            .and_then(|name| sessions.iter().position(|session| &session.name == name))
            .unwrap_or(0);

        Self {
            sessions,
            selected_index,
            current_session_name,
            should_quit: false,
            should_switch: false,
            error: None,
            vim_keys: false,
            zoom_key: "z".to_string(),
            search_query: None,
            zoomed_session: None,
            windows: Vec::new(),
            selected_window_index: 0,
        }
    }

    pub fn is_zoomed(&self) -> bool {
        self.zoomed_session.is_some()
    }

    pub fn zoomed_session(&self) -> Option<&Session> {
        let target = self.zoomed_session.as_ref()?;
        self.sessions
            .iter()
            .find(|session| &session.id == target)
            .or_else(|| self.sessions.iter().find(|session| &session.name == target))
    }

    pub fn toggle_zoom(&mut self) {
        if self.is_zoomed() {
            let selected_name = self.zoomed_session().map(|s| s.name.clone());
            self.zoomed_session = None;
            self.windows.clear();
            self.selected_window_index = 0;
            if let Some(error) = &self.error
                && !error.contains("list-sessions")
            {
                self.error = None;
            }
            if let Some(name) = selected_name
                && let Some(pos) = self
                    .visible_sessions()
                    .into_iter()
                    .position(|s| s.name == name)
            {
                self.selected_index = pos;
            }
        } else if let Some(session) = self.selected_session() {
            let session_id = session.id.clone();
            self.clear_search();
            if let Some(error) = &self.error
                && !error.contains("list-sessions")
            {
                self.error = None;
            }
            self.zoomed_session = Some(session_id);
            self.selected_window_index = 0;
        }
    }

    pub fn set_windows_for_zoomed_session(&mut self, windows: Vec<Window>) {
        let selected_id = self.selected_window().map(|w| w.id.clone());
        self.windows = windows;
        if self.windows.is_empty() {
            self.selected_window_index = 0;
            return;
        }
        self.selected_window_index = selected_id
            .and_then(|id| self.windows.iter().position(|w| w.id == id))
            .or_else(|| self.windows.iter().position(|w| w.active))
            .unwrap_or(0);
        self.selected_index = self.selected_window_index;
    }

    pub fn visible_window_count(&self) -> usize {
        self.windows.len()
    }

    pub fn selected_window(&self) -> Option<&Window> {
        self.windows.get(self.selected_window_index)
    }

    pub fn selected_target(&self) -> Option<String> {
        if self.is_zoomed() {
            self.selected_window().map(|w| w.id.clone())
        } else {
            self.selected_session().map(|s| s.id.clone())
        }
    }

    pub fn selected_session(&self) -> Option<&Session> {
        self.visible_sessions().get(self.selected_index).copied()
    }

    pub fn visible_sessions(&self) -> Vec<&Session> {
        match self.search_query.as_deref() {
            Some(query) => self
                .sessions
                .iter()
                .filter(|session| fuzzy_matches(&session.name, query))
                .collect(),
            None => self.sessions.iter().collect(),
        }
    }

    pub fn visible_session_count(&self) -> usize {
        self.visible_sessions().len()
    }

    pub fn start_search(&mut self) {
        self.search_query = Some(String::new());
        self.selected_index = 0;
    }

    pub fn push_search_char(&mut self, ch: char) {
        if let Some(query) = &mut self.search_query {
            query.push(ch);
            self.selected_index = 0;
        }
    }

    pub fn pop_search_char(&mut self) {
        if let Some(query) = &mut self.search_query {
            query.pop();
            self.selected_index = 0;
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query = None;
        self.selected_index = 0;
    }

    pub fn is_searching(&self) -> bool {
        self.search_query.is_some()
    }

    pub fn search_text(&self) -> Option<&str> {
        self.search_query.as_deref()
    }

    pub fn replace_sessions(&mut self, sessions: Vec<Session>) {
        let selected_name = if self.is_zoomed() {
            self.zoomed_session().map(|session| session.name.clone())
        } else {
            self.selected_session().map(|session| session.name.clone())
        };
        self.sessions = sessions;

        if self.visible_session_count() == 0 {
            self.selected_index = 0;
            if self.is_zoomed() {
                self.zoomed_session = None;
                self.windows.clear();
                self.selected_window_index = 0;
            }
            return;
        }

        if self.is_zoomed() {
            if self.zoomed_session().is_none() {
                self.zoomed_session = None;
                self.windows.clear();
                self.selected_window_index = 0;
                self.selected_index = self
                    .selected_index
                    .min(self.visible_session_count().saturating_sub(1));
            }
        } else {
            self.selected_index = selected_name
                .and_then(|name| {
                    self.visible_sessions()
                        .into_iter()
                        .position(|session| session.name == name)
                })
                .unwrap_or_else(|| {
                    self.selected_index
                        .min(self.visible_session_count().saturating_sub(1))
                });
        }
    }

    pub fn replace_sessions_preserving_preview_for(
        &mut self,
        mut sessions: Vec<Session>,
        preserved_session_id: Option<&str>,
    ) {
        if let Some(preserved_session_id) = preserved_session_id
            && let Some(previous) = self
                .sessions
                .iter()
                .find(|session| session.id == preserved_session_id)
            && let Some(next) = sessions
                .iter_mut()
                .find(|session| session.id == preserved_session_id)
        {
            next.preview = previous.preview.clone();
            next.preview_error = previous.preview_error.clone();
        }

        self.replace_sessions(sessions);
    }

    pub fn replace_sessions_preserving_all_previews(&mut self, mut sessions: Vec<Session>) {
        for session in &mut sessions {
            if let Some(previous) = self.sessions.iter_mut().find(|s| s.id == session.id) {
                session.preview = std::mem::take(&mut previous.preview);
                session.preview_error = previous.preview_error.take();
                if session.current_window.is_none() {
                    session.current_window = previous.current_window.take();
                }
            }
        }
        self.replace_sessions(sessions);
    }

    pub fn move_left(&mut self) {
        if self.is_zoomed() {
            if self.selected_window_index > 0 {
                self.selected_window_index -= 1;
                self.selected_index = self.selected_window_index;
            }
        } else if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.is_zoomed() {
            if self.selected_window_index + 1 < self.visible_window_count() {
                self.selected_window_index += 1;
                self.selected_index = self.selected_window_index;
            }
        } else if self.selected_index + 1 < self.visible_session_count() {
            self.selected_index += 1;
        }
    }

    pub fn move_up(&mut self, columns: usize) {
        let columns = columns.max(1);
        if self.is_zoomed() {
            if self.selected_window_index >= columns {
                self.selected_window_index -= columns;
                self.selected_index = self.selected_window_index;
            }
        } else if self.selected_index >= columns {
            self.selected_index -= columns;
        }
    }

    pub fn move_down(&mut self, columns: usize) {
        let columns = columns.max(1);
        let count = if self.is_zoomed() {
            self.visible_window_count()
        } else {
            self.visible_session_count()
        };
        if count == 0 {
            return;
        }

        let last_index = count - 1;
        let current_index = if self.is_zoomed() {
            self.selected_window_index
        } else {
            self.selected_index
        };
        let current_row = current_index / columns;
        let last_row = last_index / columns;
        if current_row < last_row {
            let new_index = current_index.saturating_add(columns).min(last_index);
            if self.is_zoomed() {
                self.selected_window_index = new_index;
                self.selected_index = new_index;
            } else {
                self.selected_index = new_index;
            }
        }
    }
}

fn fuzzy_matches(name: &str, query: &str) -> bool {
    let query = query.to_lowercase();
    if query.is_empty() {
        return true;
    }

    let name = name.to_lowercase();
    let mut name_chars = name.chars();
    query
        .chars()
        .all(|query_ch| name_chars.any(|name_ch| name_ch == query_ch))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn window(id: &str, index: u32, name: &str, active: bool) -> Window {
        Window {
            id: id.to_string(),
            index,
            name: name.to_string(),
            active,
            last_activity: None, panes: Vec::new(),
            preview_error: None,
        }
    }

    #[test]
    fn selects_current_session_when_present() {
        let app = App::new(
            vec![session("dev"), session("logs"), session("notes")],
            Some("logs".to_string()),
        );

        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn clamps_navigation_at_grid_edges() {
        let mut app = App::new(vec![session("one"), session("two"), session("three")], None);

        app.move_left();
        assert_eq!(app.selected_index, 0);

        app.move_right();
        app.move_right();
        app.move_right();
        assert_eq!(app.selected_index, 2);

        app.move_down(2);
        assert_eq!(app.selected_index, 2);

        app.move_up(2);
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn preserves_selected_session_by_name_after_refresh() {
        let mut app = App::new(
            vec![session("dev"), session("logs"), session("notes")],
            None,
        );
        app.selected_index = 1;

        app.replace_sessions(vec![session("new"), session("logs"), session("dev")]);

        assert_eq!(app.selected_session().unwrap().name, "logs");
    }

    #[test]
    fn preserves_preview_for_matching_session_after_refresh() {
        let mut app = App::new(
            vec![session("dev"), session("logs")],
            Some("dev".to_string()),
        );
        app.sessions[0].preview = vec!["snapshot".to_string()];
        app.sessions[0].preview_error = None;

        let mut refreshed_dev = session("dev");
        refreshed_dev.preview = Vec::new();
        refreshed_dev.preview_error = Some("Current session preview disabled".to_string());

        let mut refreshed_logs = session("logs");
        refreshed_logs.preview = vec!["live".to_string()];

        app.replace_sessions_preserving_preview_for(
            vec![refreshed_dev, refreshed_logs],
            Some("$dev"),
        );

        assert_eq!(app.sessions[0].preview, vec!["snapshot".to_string()]);
        assert_eq!(app.sessions[0].preview_error, None);
        assert_eq!(app.sessions[1].preview, vec!["live".to_string()]);
    }

    #[test]
    fn search_filters_sessions_by_fuzzy_name() {
        let mut app = App::new(
            vec![
                session("backend-api"),
                session("frontend"),
                session("database"),
            ],
            None,
        );

        app.start_search();
        app.push_search_char('b');
        app.push_search_char('a');

        let names: Vec<&str> = app
            .visible_sessions()
            .into_iter()
            .map(|session| session.name.as_str())
            .collect();
        assert_eq!(names, vec!["backend-api", "database"]);
    }

    #[test]
    fn selected_session_uses_filtered_selection() {
        let mut app = App::new(
            vec![session("backend"), session("frontend"), session("database")],
            None,
        );

        app.start_search();
        app.push_search_char('f');

        assert_eq!(app.selected_index, 0);
        assert_eq!(app.selected_session().unwrap().name, "frontend");
    }

    #[test]
    fn clearing_search_restores_all_sessions() {
        let mut app = App::new(vec![session("backend"), session("frontend")], None);

        app.start_search();
        app.push_search_char('f');
        app.clear_search();

        assert!(!app.is_searching());
        assert_eq!(app.visible_session_count(), 2);
    }

    #[test]
    fn up_from_first_row_keeps_selection_in_place() {
        let mut app = App::new(vec![session("one"), session("two"), session("three")], None);
        app.selected_index = 1;

        app.move_up(2);

        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn down_to_incomplete_row_selects_nearest_card() {
        let mut app = App::new(
            vec![
                session("one"),
                session("two"),
                session("three"),
                session("four"),
                session("five"),
            ],
            None,
        );
        app.selected_index = 2;

        app.move_down(3);

        assert_eq!(app.selected_index, 4);
    }

    #[test]
    fn zoomed_window_navigation_and_selection() {
        let mut app = App::new(vec![session("dev")], None);
        assert!(!app.is_zoomed());
        assert_eq!(app.selected_target().as_deref(), Some("$dev"));

        app.toggle_zoom();
        assert!(app.is_zoomed());
        assert_eq!(app.zoomed_session().map(|s| s.name.as_str()), Some("dev"));

        app.set_windows_for_zoomed_session(vec![
            window("@1", 0, "bash", false),
            window("@2", 1, "editor", true),
            window("@3", 2, "logs", false),
        ]);

        assert_eq!(app.visible_window_count(), 3);
        assert_eq!(app.selected_window_index, 1);
        assert_eq!(app.selected_index, 1);
        assert_eq!(app.selected_target().as_deref(), Some("@2"));

        app.move_right();
        assert_eq!(app.selected_window_index, 2);
        assert_eq!(app.selected_index, 2);
        assert_eq!(app.selected_target().as_deref(), Some("@3"));

        app.move_left();
        assert_eq!(app.selected_window_index, 1);
        assert_eq!(app.selected_index, 1);

        app.move_up(2);
        assert_eq!(app.selected_window_index, 1);
        assert_eq!(app.selected_index, 1);

        app.toggle_zoom();
        assert!(!app.is_zoomed());
        assert_eq!(app.selected_target().as_deref(), Some("$dev"));
        assert_eq!(app.visible_window_count(), 0);
    }

    #[test]
    fn zoom_out_clears_windows_and_prevents_stale_ids_across_sessions() {
        let mut app = App::new(vec![session("dev"), session("prod")], None);
        app.selected_index = 0;
        app.toggle_zoom();
        assert!(app.is_zoomed());

        app.set_windows_for_zoomed_session(vec![
            window("@1", 0, "bash", true),
            window("@2", 1, "editor", false),
        ]);
        assert_eq!(app.visible_window_count(), 2);
        assert_eq!(app.selected_target().as_deref(), Some("@1"));

        // Zoom out
        app.toggle_zoom();
        assert!(!app.is_zoomed());
        assert_eq!(app.visible_window_count(), 0);
        assert_eq!(app.selected_target().as_deref(), Some("$dev"));

        // Move to session "prod" and zoom in
        app.selected_index = 1;
        app.toggle_zoom();
        assert!(app.is_zoomed());
        assert_eq!(app.zoomed_session().unwrap().name, "prod");
        // Windows list is empty until fetched for "prod" - no stale windows from "dev"
        assert_eq!(app.visible_window_count(), 0);
        assert_eq!(app.selected_target(), None);

        app.set_windows_for_zoomed_session(vec![window("@3", 0, "server", true)]);
        assert_eq!(app.visible_window_count(), 1);
        assert_eq!(app.selected_target().as_deref(), Some("@3"));
    }

    #[test]
    fn replace_sessions_clears_zoom_and_windows_if_zoomed_session_disappears() {
        let mut app = App::new(vec![session("dev"), session("prod")], None);
        app.toggle_zoom();
        app.set_windows_for_zoomed_session(vec![window("@1", 0, "bash", true)]);
        assert!(app.is_zoomed());

        // Replace sessions with list not containing "dev"
        app.replace_sessions(vec![session("prod")]);
        assert!(!app.is_zoomed());
        assert_eq!(app.visible_window_count(), 0);
    }

    #[test]
    fn zoomed_session_resolves_exact_id_over_colliding_name() {
        let session_colliding = Session {
            id: "$0".to_string(),
            name: "$1".to_string(),
            attached: false,
            window_count: 1,
            current_window: None,
            last_activity: None,
            preview: Vec::new(),
            preview_error: None,
        };
        let session_target = Session {
            id: "$1".to_string(),
            name: "work".to_string(),
            attached: false,
            window_count: 2,
            current_window: None,
            last_activity: None,
            preview: Vec::new(),
            preview_error: None,
        };
        let mut app = App::new(vec![session_colliding, session_target], None);
        app.zoomed_session = Some("$1".to_string());

        assert_eq!(app.zoomed_session().unwrap().name, "work");
        assert_eq!(app.zoomed_session().unwrap().id, "$1");
    }

    #[test]
    fn toggle_zoom_clears_window_error_on_zoom_out() {
        let mut app = App::new(vec![session("dev")], None);
        app.toggle_zoom();
        app.error = Some("window fetch error".to_string());

        app.toggle_zoom();
        assert!(!app.is_zoomed());
        assert_eq!(app.error, None);
    }

    #[test]
    fn toggle_zoom_preserves_session_refresh_error_on_zoom_out() {
        let mut app = App::new(vec![session("dev")], None);
        app.toggle_zoom();
        app.error = Some("tmux list-sessions failed: connection refused".to_string());

        app.toggle_zoom();
        assert!(!app.is_zoomed());
        assert_eq!(
            app.error,
            Some("tmux list-sessions failed: connection refused".to_string())
        );
    }
}
