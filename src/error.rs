//! Error types
#[derive(Debug)]
pub enum EntityError {
    /// attempted to access unregistered component
    ComponentNotRegistered,
    /// attempted to access entity that does not exist
    EntityDoesNotExist,
    /// attempted to access component not included in query
    ComponentNotInQuery,
    /// attemted getting component data that does not exist
    ComponentDataDoesNotExist,
    /// attemted downcasting to wrong type
    DowncastToWrongType,
}

#[derive(Debug)]
pub enum ResourceError {
    ResourceDoesNotExist,
    ResourceAlreadyPresent,
}
