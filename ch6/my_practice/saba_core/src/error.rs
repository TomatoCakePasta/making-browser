use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedInput(String),
    InvalidUI(String),
    Other(String),
}

// we can know type of error by looking at enum
// data example
/*
    This is an example of Network error
    Error::Network {
        code: 504,
        detail: "gateway timeout".to_string(),
    }
*/