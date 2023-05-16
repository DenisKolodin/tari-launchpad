use crate::component::expert::ExpertScene;
use crate::component::header::{mode::Mode, Header};
use crate::component::normal::NormalScene;
use crate::component::onboarding::OnboardingScene;
use crate::component::scene;
use crate::component::settings::SettingsScene;

use crate::component::{Component, ComponentEvent, Input};
use crate::state::AppState;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct MainView {
    header: Header,
    normal_scene: NormalScene,
    expert_scene: ExpertScene,
    settings_scene: SettingsScene,
    containers_scene: scene::Containers,
    wallet_scene: scene::Wallet,
    onboarding_scene: OnboardingScene,
}

impl MainView {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            normal_scene: NormalScene::new(),
            expert_scene: ExpertScene::new(),
            settings_scene: SettingsScene::new(),
            containers_scene: scene::Containers::new(),
            wallet_scene: scene::Wallet::new(),
            onboarding_scene: OnboardingScene::new(),
        }
    }
}

impl Input for MainView {
    fn on_event(&mut self, event: ComponentEvent, state: &mut AppState) {
        self.header.on_event(event, state);
        if state.focus_on.is_onboarding() {
            self.onboarding_scene.on_event(event, state);
        } else {
            match self.header.mode_selector.selected() {
                Mode::Normal => {
                    self.normal_scene.on_event(event, state);
                }
                Mode::Expert => {
                    self.expert_scene.on_event(event, state);
                }
                Mode::Settings => {
                    self.settings_scene.on_event(event, state);
                }
            }
        }
    }
}

impl<B: Backend> Component<B> for MainView {
    type State = AppState;

    fn draw(&self, f: &mut Frame<B>, rect: Rect, state: &Self::State) {
        let constraints = [Constraint::Length(1), Constraint::Min(0)];
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(rect);
        self.header.draw(f, chunks[0], state);
        if state.focus_on.is_onboarding() {
            self.onboarding_scene.draw(f, chunks[1], state);
        } else {
            match self.header.mode_selector.selected() {
                Mode::Normal => {
                    self.normal_scene.draw(f, chunks[1], state);
                }
                Mode::Expert => {
                    self.expert_scene.draw(f, chunks[1], state);
                }
                Mode::Settings => {
                    self.settings_scene.draw(f, chunks[1], state);
                }
            }
        }
    }
}
