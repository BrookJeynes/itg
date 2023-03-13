use core::fmt;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub open_issues_count: isize
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
