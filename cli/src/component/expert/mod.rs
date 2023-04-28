use strum::{Display, EnumCount, EnumIter, FromRepr, IntoEnumIterator};

#[derive(Debug, EnumCount, EnumIter, FromRepr, Clone, Copy, Display)]
pub enum ExpertTabs {
    Performance,
    Containers,
    Logs,
}
