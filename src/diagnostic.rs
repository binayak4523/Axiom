#[derive(Debug)]
pub struct Diagnostic {
    pub title: String,
    pub message: String,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            help: None,
        }
    }

    pub fn with_help(mut self, help: &str) -> Self {
        self.help = Some(help.to_string());
        self
    }
}
