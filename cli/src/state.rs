use rust_decimal::Decimal;

pub struct LaunchpadState {
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
}

impl LaunchpadState {
    pub fn new() -> Self {
        let tari_mining = TariMiningInfo {
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {};
        Self {
            tari_mining,
            merged_mining,
        }
    }
}

pub struct TariMiningInfo {
    tari_amount: Decimal,
}

impl TariMiningInfo {
    pub fn is_mining_active(&self) -> bool {
        true
    }

    pub fn tari_mining_tari_amount(&self) -> Decimal {
        123_456.into()
    }
}

pub struct MergedMiningInfo {}

impl MergedMiningInfo {
    pub fn is_mining_active(&self) -> bool {
        false
    }
}
