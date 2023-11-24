use thiserror::Error;

// make private, once implementation is finished
#[derive(Debug, Error)]
pub enum EntityErrors {
    #[error("Attempted adding an unregistered component to an entity")]
    ComponentNeverRegistered,
    #[error("Attempted inserting data into unregistered component")]
    ComponentNotRegistered,
}
