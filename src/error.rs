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
    use axum::response::IntoResponse;
    use serde_json::Value;
    use std::error::Error as StdError;

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
    fn test_all_error_constructors() {
        // Test all error constructor methods
        let config_err = Error::config("Config issue");
        assert!(matches!(config_err, Error::Config { .. }));

        let fs_err = Error::file_system("File system issue");
        assert!(matches!(fs_err, Error::FileSystem { .. }));

        let gemini_err = Error::gemini_api("API issue", Some(400));
        assert!(matches!(gemini_err, Error::GeminiApi { .. }));

        let invalid_req_err = Error::invalid_request("Bad request");
        assert!(matches!(invalid_req_err, Error::InvalidRequest { .. }));

        let not_found_err = Error::not_found("resource");
        assert!(matches!(not_found_err, Error::NotFound { .. }));

        let invalid_img_err = Error::invalid_image("Bad image");
        assert!(matches!(invalid_img_err, Error::InvalidImage { .. }));

        let rate_limit_err = Error::rate_limit("Too fast");
        assert!(matches!(rate_limit_err, Error::RateLimit { .. }));

        let internal_err = Error::internal("Something went wrong");
        assert!(matches!(internal_err, Error::Internal { .. }));
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
        assert_eq!(
            Error::invalid_image("corrupt image").status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            Error::config("missing key").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            Error::file_system("disk full").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            Error::internal("panic").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        // Test Gemini API error with status code
        assert_eq!(
            Error::gemini_api("Bad request", Some(400)).status_code(),
            StatusCode::BAD_REQUEST
        );

        // Test Gemini API error without status code
        assert_eq!(
            Error::gemini_api("Unknown error", None).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );

        // Test Gemini API error with invalid status code
        assert_eq!(
            Error::gemini_api("Invalid status", Some(500)).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_user_messages() {
        let error = Error::invalid_request("Invalid file format");
        assert_eq!(error.user_message(), "Invalid file format");

        let error = Error::not_found("avatar.jpg");
        assert_eq!(error.user_message(), "Resource not found: avatar.jpg");

        let error = Error::invalid_image("Corrupted JPEG");
        assert_eq!(error.user_message(), "Invalid image: Corrupted JPEG");

        let error = Error::rate_limit("API quota exceeded");
        assert_eq!(
            error.user_message(),
            "Rate limit exceeded: API quota exceeded"
        );

        let error = Error::gemini_api("Service unavailable", Some(503));
        assert_eq!(
            error.user_message(),
            "AI service error: Service unavailable"
        );

        let error = Error::internal("Database connection failed");
        assert_eq!(error.user_message(), "Internal server error occurred");

        let error = Error::config("Missing API key");
        assert_eq!(error.user_message(), "Internal server error occurred");

        let error = Error::file_system("Disk full");
        assert_eq!(error.user_message(), "Internal server error occurred");
    }

    #[test]
    fn test_error_display() {
        let error = Error::config("API key not set");
        assert_eq!(error.to_string(), "Configuration error: API key not set");

        let error = Error::gemini_api("Rate limited", Some(429));
        assert_eq!(
            error.to_string(),
            "Gemini API error: Rate limited, status: Some(429)"
        );

        let error = Error::not_found("image.jpg");
        assert_eq!(error.to_string(), "Resource not found: image.jpg");
    }

    #[test]
    fn test_from_conversions() {
        // Test automatic conversions from common error types
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io(_)));

        let json_error = serde_json::from_str::<Value>("invalid json").unwrap_err();
        let error: Error = json_error.into();
        assert!(matches!(error, Error::Json(_)));

        let uuid_error = uuid::Uuid::parse_str("invalid-uuid").unwrap_err();
        let error: Error = uuid_error.into();
        assert!(matches!(error, Error::Uuid(_)));

        let anyhow_error = anyhow::anyhow!("Custom error");
        let error: Error = anyhow_error.into();
        assert!(matches!(error, Error::Anyhow(_)));
    }

    #[tokio::test]
    async fn test_into_response() {
        let error = Error::invalid_request("Bad input data");
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Extract and verify response body
        let (parts, body) = response.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap();

        // Parse JSON response
        let json: Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(json["error"]["message"], "Bad input data");
        assert_eq!(json["error"]["code"], 400);

        // Check content type header
        assert_eq!(
            parts.headers.get("content-type").unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_error_response_formats() {
        let test_cases = vec![
            (
                Error::invalid_request("Invalid format"),
                StatusCode::BAD_REQUEST,
            ),
            (Error::not_found("user"), StatusCode::NOT_FOUND),
            (
                Error::rate_limit("Too many requests"),
                StatusCode::TOO_MANY_REQUESTS,
            ),
            (
                Error::internal("Server error"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        ];

        for (error, expected_status) in test_cases {
            let response = error.into_response();
            assert_eq!(response.status(), expected_status);

            // Verify response contains JSON error format
            let (_, body) = response.into_parts();
            let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
            let body_str = String::from_utf8(bytes.to_vec()).unwrap();
            let json: Value = serde_json::from_str(&body_str).unwrap();

            assert!(json.get("error").is_some());
            assert!(json["error"].get("message").is_some());
            assert!(json["error"].get("code").is_some());
            assert_eq!(json["error"]["code"], expected_status.as_u16());
        }
    }

    #[test]
    fn test_error_chaining() {
        // Test that error source information is preserved
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let error: Error = io_error.into();

        assert!(error.source().is_some());
        assert_eq!(error.to_string(), "IO error: Access denied");
    }

    #[test]
    fn test_error_debug() {
        let error = Error::config("Missing API key");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("Missing API key"));
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<i32> {
            Ok(42)
        }

        fn test_function_error() -> Result<i32> {
            Err(Error::invalid_request("Test error"))
        }

        assert_eq!(test_function().unwrap(), 42);
        assert!(test_function_error().is_err());
    }

    #[test]
    fn test_error_message_content_safety() {
        // Ensure sensitive information isn't leaked in user messages
        let sensitive_error = Error::internal("Database password: secret123");
        assert_eq!(
            sensitive_error.user_message(),
            "Internal server error occurred"
        );

        let config_error = Error::config("API key: sk-abcd1234");
        assert_eq!(
            config_error.user_message(),
            "Internal server error occurred"
        );

        // But user-facing errors should preserve their messages
        let user_error = Error::invalid_request("Email is required");
        assert_eq!(user_error.user_message(), "Email is required");
    }
}
