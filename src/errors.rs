use thiserror::Error;

//TODO: make private, once implementation is finished
#[derive(Debug, Error)]
pub enum EntityErrors {
    #[error("Attempted to access unregistered component")]
    ComponentNotRegistered,
    #[error("Attempted to access entity, that doesn't exist")]
    EntityDoesNotExist,
}
