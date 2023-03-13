use core::fmt;
use std::slice::Iter;

#[derive(PartialEq)]
pub enum MenuItems {
    Issues,
}

impl MenuItems {
    pub fn iterator() -> Iter<'static, MenuItems> {
        static MENU_ITEMS: [MenuItems; 1] = [MenuItems::Issues];
        MENU_ITEMS.iter()
    }
}

impl fmt::Display for MenuItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Issues => write!(f, "[I]ssues"),
        }
    }
}
