#[derive(Clone)]
pub struct Popup {
    pub message: String,
    pub title: String,
    pub show_popup: bool,
}

impl Popup {
    pub fn show_popup(&mut self, title: String, message: String) { self.message = message;
        self.title = title;
        self.show_popup = true;
    }

    pub fn close_popup(&mut self) {
        self.message = String::new();
        self.title = String::new();
        self.show_popup = false;
    }
}

impl Default for Popup {
    fn default() -> Self {
        Self {
            message: String::new(),
            title: String::new(),
            show_popup: false,
        }
    }
}
