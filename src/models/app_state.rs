use std::collections::HashMap;

use crate::Issue;

use super::{
    config::Config, input_mode::InputMode, menu_items::MenuItems, repository::Repository,
    screen::Screen, stateful_list::StatefulList,
};

pub struct AppState {
    /// App config file
    pub config: Config,
    /// The current menu item
    pub current_menu: MenuItems,
    /// All issues in the current selected repository
    pub issues: StatefulList<Issue>,
    /// A cache of issues
    pub issue_cache: HashMap<String, Vec<Issue>>,
    /// All repositories fetched when the app opened
    pub repositories: StatefulList<Repository>,
    /// The selected repository
    pub selected_repo: Option<Repository>,
    /// The current focused screen
    pub screen: Screen,
    /// The users current input mode
    pub input_mode: InputMode,
    /// Whether the repository search window is open or not
    pub show_search: bool,
    /// Handle the previous error
    pub error_message: String,
    /// The users current search string
    pub search_string: String,
}

impl AppState {
    pub fn new(config: Config, repositories: Vec<Repository>) -> Self {
        Self {
            config,
            current_menu: MenuItems::Issues,
            issues: StatefulList::with_items(vec![]),
            issue_cache: HashMap::new(),
            repositories: StatefulList::with_items(repositories),
            selected_repo: None,
            screen: Screen::Issues,
            input_mode: InputMode::Normal,
            show_search: false,
            error_message: String::new(),
            search_string: String::new(),
        }
    }

    pub fn change_focus(&mut self) {
        match self.screen {
            Screen::Issues => self.screen = Screen::Repositories,
            Screen::Repositories => self.screen = Screen::Issues,
            Screen::Error => {}
        }
    }

    pub fn cache_issues(&mut self, repository_name: String, issues: Vec<Issue>) {
        self.issue_cache.insert(repository_name, issues);
    }

    pub fn select_repo(&mut self, repository: Repository) {
        self.selected_repo = Some(repository)
    }

    pub fn show_search(&mut self) {
        self.show_search = true;
        self.input_mode = InputMode::Editing;
    }

    pub fn hide_search(&mut self) {
        self.show_search = false;
        self.input_mode = InputMode::Normal;
    }

    pub fn show_error(&mut self, error_message: String) {
        self.input_mode = InputMode::Normal;
        self.hide_search();
        self.error_message = error_message;
        self.screen = Screen::Error;
    }

    pub fn close_error(&mut self) {
        self.error_message = String::new();
        self.screen = Screen::Issues;
    }
}
