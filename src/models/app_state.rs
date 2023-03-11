use std::collections::HashMap;

use crate::Issue;

use super::{
    config::Config, menu_items::MenuItems, repository::Repository, screen::Screen,
    stateful_list::StatefulList,
};

pub struct AppState {
    pub config: Config,
    pub current_menu: MenuItems,
    pub issues: Option<StatefulList<Issue>>,
    pub issue_cache: HashMap<String, Vec<Issue>>,
    pub repositories: StatefulList<Repository>,
    pub selected_repo: Option<Repository>,
    pub screen: Screen,
}

impl AppState {
    pub fn new(config: Config, repositories: Vec<Repository>) -> Self {
        Self {
            config,
            current_menu: MenuItems::Issues,
            issues: None,
            issue_cache: HashMap::new(),
            repositories: StatefulList::with_items(repositories),
            selected_repo: None,
            screen: Screen::Issues,
        }
    }

    pub fn change_focus(&mut self) {
        match self.screen {
            Screen::Issues => self.screen = Screen::Repositories,
            Screen::Repositories => self.screen = Screen::Issues,
        }
    }

    pub fn cache_issues(&mut self, repository_name: String, issues: Vec<Issue>) {
        self.issue_cache.insert(repository_name, issues);
    }

    pub fn select_repo(&mut self, repository: Repository) {
        self.selected_repo = Some(repository)
    }

    pub fn get_issues(&self) -> StatefulList<Issue> {
        match &self.issues {
            Some(issues) => issues.clone(),
            None => StatefulList::with_items(Vec::new()),
        }
    }
}
