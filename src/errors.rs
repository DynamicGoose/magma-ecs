use thiserror::Error;

#[derive(Debug, Error)]
pub enum MecsErrors {
    #[error("Attempted to access unregistered component")]
    ComponentNotRegistered,
    #[error("Attempted to access entity, that doesn't exist")]
    EntityDoesNotExist,
}
