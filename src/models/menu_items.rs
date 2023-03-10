use core::fmt;
use std::slice::Iter;

#[derive(PartialEq)]
pub enum MenuItems {
    Issues,
    PullRequests,
}

impl MenuItems {
    pub fn iterator() -> Iter<'static, MenuItems> {
        static MENU_ITEMS: [MenuItems; 2] = [MenuItems::Issues, MenuItems::PullRequests];
        MENU_ITEMS.iter()
    }
}

impl fmt::Display for MenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issues => write!(f, "[I]ssues"),
            Self::PullRequests => write!(f, "[P]ull requests"),
        }
    }
}
