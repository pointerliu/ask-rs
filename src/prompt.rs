use std::io::{IsTerminal, Read, Stdin};
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Prompt {
    pub message: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            message: "".to_owned()
        }
    }
    pub fn add_source(mut self, src: impl Into<Prompt>) -> Self {
        self += src.into();
        self
    }
}

impl From<Stdin> for Prompt {
    fn from(mut value: Stdin) -> Self {
        let msg = if value.is_terminal() {
            "".to_string()
        } else {
            let mut buffer = Vec::new();
            let _len = value.read_to_end(&mut buffer).unwrap();
            String::from_utf8(buffer).unwrap()
        };
        Self {
            message: msg
        }
    }
}

impl From<String> for Prompt {
    fn from(value: String) -> Self {
        Self {
            message: value
        }
    }
}

impl AddAssign for Prompt {
    fn add_assign(&mut self, rhs: Self) {
        self.message += &rhs.message
    }
}
