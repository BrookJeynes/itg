use tui::widgets::ListState;

#[derive(Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    /// Create a StatefulList with the items passed in.
    pub fn with_items(items: Vec<T>) -> Self {
        let mut list = Self {
            state: ListState::default(),
            items,
        };

        list.next();

        list
    }

    /// Move the internally selected item forward.
    pub fn next(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        i
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i));
        }
    }

    /// Move the internally selected item backwards.
    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        i
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i));
        }
    }

    /// Return the selected items index.
    pub fn selected(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        self.state.selected()
    }

    /// Return the selected items value.
    pub fn selected_value(&self) -> Option<&T> {
        match self.selected() {
            Some(index) => match self.items.get(index) {
                Some(items) => Some(items),
                None => None,
            },
            None => None,
        }
    }
}
