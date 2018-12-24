pub(crate) struct PromptMsg {
    pub msg: String,
    pub repeat: bool,
}

impl PromptMsg {
    pub fn new() -> Self {
        Self {
            msg: String::new(),
            repeat: false,
        }
    }
    pub fn from_str(s: impl ToString) -> Self {
        Self {
            msg: s.to_string(),
            repeat: false,
        }
    }
    pub fn repeat_from_str(s: impl ToString) -> Self {
        Self {
            msg: s.to_string(),
            repeat: true,
        }
    }
}
