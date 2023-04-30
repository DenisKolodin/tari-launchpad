pub struct LaunchpadState {}

impl LaunchpadState {
    pub fn is_tari_mining_active(&self) -> bool {
        true
    }

    pub fn is_merged_mining_active(&self) -> bool {
        false
    }
}
