#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum OnboardingAction {
    Next,
}

#[derive(Debug, Clone)]
pub enum OnboardingDelta {}
