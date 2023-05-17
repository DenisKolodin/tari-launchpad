#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Onboarding {
    pub messages: Vec<Message>,
}

impl Onboarding {
    pub fn update(&mut self, delta: OnboardingDelta) {
        match delta {
            OnboardingDelta::Add(msg) => {
                self.messages.push(msg);
            }
        }
    }
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
pub enum OnboardingDelta {
    Add(Message),
}
