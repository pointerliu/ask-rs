use std::io::{IsTerminal, Read, Stdin};

#[derive(Debug)]
pub struct Prompt {
    pub message: String,
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
