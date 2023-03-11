use core::fmt;

pub enum Errors {
    FetchRequestError,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::FetchRequestError => write!(f, "Failed to fetch requested content."),
        }
    }
}
