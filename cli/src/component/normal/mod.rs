use strum::{Display, EnumCount, EnumIter, FromRepr};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum NormalTabs {
    Mining,
    #[strum(serialize = "Base Node")]
    BaseNode,
    Wallet,
}
