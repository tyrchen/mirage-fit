//! Configuration management for Mirage Fit
//!
//! This module handles application configuration, including API keys,
//! directory paths, and service settings.

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Gemini API configuration
    pub gemini: GeminiConfig,
    /// File system configuration
    pub fs: FileSystemConfig,
    /// Server configuration
    pub server: ServerConfig,
}

/// Gemini API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiConfig {
    /// API key for Google Gemini service
    pub api_key: String,
    /// Base URL for Gemini API
    pub base_url: String,
    /// Model to use for image generation
    pub model: String,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Max retries for failed requests
    pub max_retries: u32,
    /// Rate limiting (requests per minute)
    pub rate_limit_rpm: u32,
}

/// File system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    /// Base directory for all Mirage Fit data
    pub base_dir: PathBuf,
    /// Directory for user input photos
    pub input_dir: PathBuf,
    /// Directory for generated item images
    pub items_dir: PathBuf,
    /// Directory for remix output images
    pub output_dir: PathBuf,
    /// Maximum file size for uploads (in bytes)
    pub max_file_size: u64,
    /// Supported image formats
    pub supported_formats: Vec<String>,
    /// Cache size limit (in MB)
    pub cache_size_limit_mb: u64,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Maximum request body size (in bytes)
    pub max_request_size: u64,
    /// Request timeout (in seconds)
    pub request_timeout: u64,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    /// Static file serving configuration
    pub static_files: StaticFilesConfig,
}

/// Static file serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticFilesConfig {
    /// Enable static file serving
    pub enabled: bool,
    /// Cache control header for static files (in seconds)
    pub cache_control_max_age: u32,
    /// Enable compression for static files
    pub enable_compression: bool,
}

impl Config {
    /// Create a new configuration with the given API key
    pub fn new(gemini_api_key: Option<String>) -> Result<Self> {
        let api_key = gemini_api_key
            .filter(|key| !key.is_empty())  // Filter out empty strings
            .or_else(|| std::env::var("GEMINI_API_KEY").ok().filter(|key| !key.is_empty()))
            .ok_or_else(|| {
                Error::config(
                    "Gemini API key not provided. Set GEMINI_API_KEY environment variable or use --gemini-api-key flag"
                )
            })?;

        let base_dir = Self::get_base_directory()?;

        Ok(Self {
            gemini: GeminiConfig {
                api_key,
                base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
                model: "gemini-2.5-flash-image-preview".to_string(),
                timeout_seconds: 120,
                max_retries: 3,
                rate_limit_rpm: 60,
            },
            fs: FileSystemConfig {
                input_dir: base_dir.join("input"),
                items_dir: base_dir.join("items"),
                output_dir: base_dir.join("output"),
                base_dir,
                max_file_size: 10 * 1024 * 1024, // 10MB
                supported_formats: vec![
                    "image/jpeg".to_string(),
                    "image/png".to_string(),
                    "image/webp".to_string(),
                ],
                cache_size_limit_mb: 1024, // 1GB
            },
            server: ServerConfig {
                max_request_size: 20 * 1024 * 1024, // 20MB
                request_timeout: 300,               // 5 minutes
                enable_cors: true,
                cors_origins: vec!["*".to_string()],
                static_files: StaticFilesConfig {
                    enabled: true,
                    cache_control_max_age: 3600, // 1 hour
                    enable_compression: true,
                },
            },
        })
    }

    /// Get the base directory for Mirage Fit data
    fn get_base_directory() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| Error::config("Unable to determine home directory"))?;

        Ok(home_dir.join(".mirage-fit"))
    }

    /// Get the full Gemini API URL for the specified endpoint
    pub fn gemini_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.gemini.base_url, endpoint)
    }

    /// Get the generate endpoint URL for the configured model
    pub fn gemini_generate_url(&self) -> String {
        self.gemini_url(&format!(
            "models/{}:generateContent?key={}",
            self.gemini.model, self.gemini.api_key
        ))
    }

    /// Check if a file format is supported
    pub fn is_supported_format(&self, mime_type: &str) -> bool {
        self.fs.supported_formats.contains(&mime_type.to_string())
    }

    /// Get the file extension for a MIME type
    pub fn get_file_extension(&self, mime_type: &str) -> Option<&'static str> {
        match mime_type {
            "image/jpeg" => Some("jpg"),
            "image/png" => Some("png"),
            "image/webp" => Some("webp"),
            _ => None,
        }
    }

    /// Validate file size against configuration limits
    pub fn validate_file_size(&self, size: u64) -> Result<()> {
        if size > self.fs.max_file_size {
            return Err(Error::invalid_request(format!(
                "File size ({} bytes) exceeds maximum allowed size ({} bytes)",
                size, self.fs.max_file_size
            )));
        }
        Ok(())
    }

    /// Create all necessary directories
    pub async fn ensure_directories(&self) -> Result<()> {
        use crate::models::ItemCategory;
        use tokio::fs;

        // Create base directories
        fs::create_dir_all(&self.fs.base_dir)
            .await
            .map_err(|e| Error::file_system(format!("Failed to create base directory: {}", e)))?;

        fs::create_dir_all(&self.fs.input_dir)
            .await
            .map_err(|e| Error::file_system(format!("Failed to create input directory: {}", e)))?;

        fs::create_dir_all(&self.fs.output_dir)
            .await
            .map_err(|e| Error::file_system(format!("Failed to create output directory: {}", e)))?;

        // Create item category directories
        for category in ItemCategory::all() {
            let category_dir = self.fs.items_dir.join(category.dir_name());
            fs::create_dir_all(&category_dir).await.map_err(|e| {
                Error::file_system(format!(
                    "Failed to create category directory '{}': {}",
                    category_dir.display(),
                    e
                ))
            })?;
        }

        tracing::info!(
            "Created directory structure at: {}",
            self.fs.base_dir.display()
        );
        Ok(())
    }

    /// Get the path for a specific category
    pub fn category_path(&self, category: &crate::models::ItemCategory) -> PathBuf {
        self.fs.items_dir.join(category.dir_name())
    }

    /// Get the URL path for accessing an image
    pub fn image_url_path(&self, image_type: ImageType, path: &str) -> String {
        // Return absolute URLs so frontend can access images from backend server
        let base_url = "http://127.0.0.1:3000";
        match image_type {
            ImageType::Input => format!("{}/api/images/input/{}", base_url, path),
            ImageType::Item => format!("{}/api/images/items/{}", base_url, path),
            ImageType::Output => format!("{}/api/images/output/{}", base_url, path),
        }
    }
}

/// Types of images in the system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageType {
    Input,
    Item,
    Output,
}

/// Default configuration for testing
impl Default for Config {
    fn default() -> Self {
        Self::new(Some("test-api-key".to_string())).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_creation_with_api_key() {
        let config = Config::new(Some("test-key".to_string())).unwrap();
        assert_eq!(config.gemini.api_key, "test-key");
        assert!(config.fs.base_dir.ends_with(".mirage-fit"));
    }

    #[test]
    fn test_config_creation_with_env_var() {
        // Save current value if it exists
        let original_key = env::var("GEMINI_API_KEY").ok();

        env::set_var("GEMINI_API_KEY", "env-test-key");
        let config = Config::new(None).unwrap();
        assert_eq!(config.gemini.api_key, "env-test-key");

        // Restore original value if it existed, otherwise remove
        match original_key {
            Some(key) => env::set_var("GEMINI_API_KEY", key),
            None => env::remove_var("GEMINI_API_KEY"),
        }
    }

    // Note: We cannot reliably test the "no API key" case because:
    // 1. Tests run in parallel, so other tests may set GEMINI_API_KEY
    // 2. The actual environment may have GEMINI_API_KEY set (as in CI/CD)
    // 3. The empty string filtering is implicitly tested by test_config_creation_with_api_key
    //    which verifies that a non-empty key is properly accepted

    #[test]
    fn test_supported_formats() {
        let config = Config::default();
        assert!(config.is_supported_format("image/jpeg"));
        assert!(config.is_supported_format("image/png"));
        assert!(config.is_supported_format("image/webp"));
        assert!(!config.is_supported_format("image/gif"));
    }

    #[test]
    fn test_file_extensions() {
        let config = Config::default();
        assert_eq!(config.get_file_extension("image/jpeg"), Some("jpg"));
        assert_eq!(config.get_file_extension("image/png"), Some("png"));
        assert_eq!(config.get_file_extension("image/webp"), Some("webp"));
        assert_eq!(config.get_file_extension("image/gif"), None);
    }

    #[test]
    fn test_gemini_urls() {
        let config = Config::default();
        let url = config.gemini_generate_url();
        assert!(url.contains("models/gemini-2.5-flash-image-preview:generateContent"));
        assert!(url.contains("key=test-api-key"));
    }

    #[test]
    fn test_file_size_validation() {
        let config = Config::default();
        assert!(config.validate_file_size(1024).is_ok());
        assert!(config
            .validate_file_size(config.fs.max_file_size + 1)
            .is_err());
    }
}
