use core::fmt;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Issue {
    pub html_url: String,
    pub number: usize,
    pub title: String,
    pub body: String,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.number, self.title)
    }
}
