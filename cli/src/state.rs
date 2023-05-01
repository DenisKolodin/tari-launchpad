use rust_decimal::Decimal;

pub struct AppState {
    pub tari_mining: TariMiningInfo,
    pub merged_mining: MergedMiningInfo,
}

impl AppState {
    pub fn new() -> Self {
        let tari_mining = TariMiningInfo {
            is_active: true,
            tari_amount: 123_456.into(),
        };
        let merged_mining = MergedMiningInfo {
            is_active: false,
            tari_amount: 45_000.into(),
            monero_amount: Decimal::new(35, 1),
        };
        Self {
            tari_mining,
            merged_mining,
        }
    }
}

pub struct TariMiningInfo {
    pub is_active: bool,
    pub tari_amount: Decimal,
}

pub struct MergedMiningInfo {
    pub is_active: bool,
    pub tari_amount: Decimal,
    pub monero_amount: Decimal,
}
