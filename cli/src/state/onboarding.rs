#[derive(Debug)]
pub struct Message {
    pub text: String,
}

#[derive(Debug)]
pub struct Onboarding {
    pub messages: Vec<Message>,
}

impl Default for Onboarding {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
        }
    }
}
