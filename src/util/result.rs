use super::errors::Error;

/// Custom internal std result type
pub type InternalStdResult<T> = std::result::Result<T, Error>;
