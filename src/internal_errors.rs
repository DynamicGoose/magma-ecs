use thiserror::Error;

// make private, once implementation is finished
#[derive(Debug, Error)]
pub enum EntityErrors {
    #[error("Attempted to use unregistered component")]
    ComponentNotRegistered,
}
