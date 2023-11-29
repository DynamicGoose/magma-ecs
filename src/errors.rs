use thiserror::Error;

#[derive(Debug, Error)]
pub enum MecsErrors {
    #[error("attempted to access unregistered component")]
    ComponentNotRegistered,
    #[error("attempted to access entity that does not exist")]
    EntityDoesNotExist,
    #[error("attempted to access component not included in query")]
    ComponentNotInQuery,
    #[error("attemted getting component data that does not exist")]
    ComponentDataDoesNotExist,
    #[error("attemted downcasting to wrong type")]
    DowncastToWrongType,
}
