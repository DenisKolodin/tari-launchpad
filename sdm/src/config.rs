use std::fmt;

pub trait ManagedProtocol: fmt::Debug + Sync + Send + 'static {
    type Config: fmt::Debug + Sync + Send + 'static;
    type Inner: fmt::Debug + Clone + Send;

    /// A part of a report.
    type Outer: fmt::Debug + Clone + Send;
}
