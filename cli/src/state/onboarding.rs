pub struct Message {
    pub text: String,
}

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
