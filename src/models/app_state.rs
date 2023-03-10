use crate::Issue;

use super::{menu_items::MenuItems, stateful_list::StatefulList};

pub struct AppState {
    pub current_menu: MenuItems,
    pub issues: StatefulList<Issue>,
}

impl AppState {
    pub fn new(issues: Vec<Issue>) -> Self {
        Self {
            current_menu: MenuItems::Issues,
            issues: StatefulList::with_items(issues),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_menu: MenuItems::Issues,
            issues: StatefulList::with_items(vec![]),
        }
    }
}
