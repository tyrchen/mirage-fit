//! Error types and handling for Mirage Fit
//!
//! This module defines the application's error types using thiserror for library
//! errors and provides conversion implementations for common error types.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Application result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for the Mirage Fit application
#[derive(Error, Debug)]
pub enum Error {
    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Image processing error
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    /// Base64 encoding/decoding error
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// UUID parsing error
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// File system error
    #[error("File system error: {message}")]
    FileSystem { message: String },

    /// Gemini API error
    #[error("Gemini API error: {message}, status: {status:?}")]
    GeminiApi {
        message: String,
        status: Option<u16>,
    },

    /// Invalid request data
    #[error("Invalid request: {message}")]
    InvalidRequest { message: String },

    /// Resource not found
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    /// Invalid image format or content
    #[error("Invalid image: {message}")]
    InvalidImage { message: String },

    /// Rate limiting error
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },

    /// Internal server error
    #[error("Internal server error: {message}")]
    Internal { message: String },

    /// Generic error from anyhow
    #[error("Error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Create a configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a file system error
    pub fn file_system(message: impl Into<String>) -> Self {
        Self::FileSystem {
            message: message.into(),
        }
    }

    /// Create a Gemini API error
    pub fn gemini_api(message: impl Into<String>, status: Option<u16>) -> Self {
        Self::GeminiApi {
            message: message.into(),
            status,
        }
    }

    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest {
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Create an invalid image error
    pub fn invalid_image(message: impl Into<String>) -> Self {
        Self::InvalidImage {
            message: message.into(),
        }
    }

    /// Create a rate limit error
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::RateLimit {
            message: message.into(),
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Error::InvalidRequest { .. } => StatusCode::BAD_REQUEST,
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::InvalidImage { .. } => StatusCode::BAD_REQUEST,
            Error::RateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            Error::GeminiApi {
                status: Some(status),
                ..
            } => StatusCode::from_u16(*status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Error::Config { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::FileSystem { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get a user-friendly error message (hiding internal details)
    pub fn user_message(&self) -> String {
        match self {
            Error::InvalidRequest { message } => message.clone(),
            Error::NotFound { resource } => format!("Resource not found: {}", resource),
            Error::InvalidImage { message } => format!("Invalid image: {}", message),
            Error::RateLimit { message } => format!("Rate limit exceeded: {}", message),
            Error::GeminiApi { message, .. } => format!("AI service error: {}", message),
            _ => "Internal server error occurred".to_string(),
        }
    }
}

/// Implement IntoResponse for automatic HTTP error handling
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let user_message = self.user_message();

        // Log the full error for debugging (in production, use structured logging)
        tracing::error!("HTTP error response: {:?}", self);

        let body = Json(json!({
            "error": {
                "message": user_message,
                "code": status.as_u16(),
            }
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let config_error = Error::config("Missing API key");
        assert!(matches!(config_error, Error::Config { .. }));
        assert_eq!(
            config_error.status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            Error::invalid_request("bad data").status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            Error::not_found("file.jpg").status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            Error::rate_limit("too many requests").status_code(),
            StatusCode::TOO_MANY_REQUESTS
        );
    }

    #[test]
    fn test_user_messages() {
        let error = Error::invalid_request("Invalid file format");
        assert_eq!(error.user_message(), "Invalid file format");

        let error = Error::internal("Database connection failed");
        assert_eq!(error.user_message(), "Internal server error occurred");
    }
}
