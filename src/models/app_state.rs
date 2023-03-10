use super::menu_items::MenuItems;

pub struct AppState {
    pub current_menu: MenuItems,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_menu: MenuItems::Issues,
        }
    }
}
