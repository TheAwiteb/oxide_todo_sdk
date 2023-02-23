//! The errors module. This module contains all the errors which can be returned by the oxide todo client.

/// The error message returned by the server.
#[derive(Debug, thiserror::Error, serde::Deserialize)]
#[error("{status}: {message}")]
pub struct ErrorMessage {
    /// The error message.
    message: String,
    /// The error code.
    status: u16,
}

#[derive(Debug, thiserror::Error)]
/// The error returned by the oxide todo sdk.
pub enum SDKError {
    /// Error when the field is missing. (This is not an error from the server.)
    /// For example, when the user try
    #[error("Missing field: {0}")]
    MissingField(String),
}

impl SDKError {
    /// Returns a new `SDKError::MissingField` error.
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }
}

/// The errors coming from the oxide todo client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The error coming from the server.
    #[error("API error: {0}")]
    APIError(#[from] ErrorMessage),
    /// The error coming from the reqwest library.
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    ///  The SDK error.
    #[error("SDK error: {0}")]
    SDKError(#[from] SDKError),
}

/// The result type of the oxide todo client.
pub type Result<T> = std::result::Result<T, Error>;
