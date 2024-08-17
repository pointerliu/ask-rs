#[derive(Debug)]
pub struct PromptError {
    msg: String,
}

impl PromptError {
    pub fn new(msg: &str) -> PromptError {
        Self {
            msg: msg.to_string()
        }
    }
}