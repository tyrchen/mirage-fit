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
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Public base URL for the API (used in responses)
    pub public_url: String,
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
            .filter(|key| !key.trim().is_empty())  // Filter out empty strings and whitespace-only strings
            .or_else(|| std::env::var("GEMINI_API_KEY").ok().filter(|key| !key.trim().is_empty()))
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
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(3000),
                public_url: std::env::var("PUBLIC_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string()),
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

    /// Get the generate endpoint URL for the configured model (without API key)
    pub fn gemini_generate_url(&self) -> String {
        self.gemini_url(&format!("models/{}:generateContent", self.gemini.model))
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
        let base_url = &self.server.public_url;
        match image_type {
            ImageType::Input => format!("{}/api/images/input/{}", base_url, path),
            ImageType::Item => format!("{}/api/images/items/{}", base_url, path),
            ImageType::Output => format!("{}/api/images/output/{}", base_url, path),
        }
    }

    /// Get the server binding address
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
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
    use tempfile::tempdir;

    #[test]
    fn test_config_creation_with_api_key() {
        let config = Config::new(Some("test-key".to_string())).unwrap();
        assert_eq!(config.gemini.api_key, "test-key");
        assert!(config.fs.base_dir.ends_with(".mirage-fit"));

        // Check default values
        assert_eq!(config.gemini.model, "gemini-2.5-flash-image-preview");
        assert_eq!(config.gemini.timeout_seconds, 120);
        assert_eq!(config.gemini.max_retries, 3);
        assert_eq!(config.gemini.rate_limit_rpm, 60);
        assert_eq!(config.fs.max_file_size, 10 * 1024 * 1024);
        assert_eq!(config.server.port, 3000);
        assert!(config.server.enable_cors);
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

    #[test]
    fn test_config_creation_with_empty_key() {
        // Save current environment state
        let original_key = env::var("GEMINI_API_KEY").ok();

        // Remove env var to ensure clean test environment
        env::remove_var("GEMINI_API_KEY");

        // Test with empty string - should fail since empty strings are filtered out
        let result = Config::new(Some("".to_string()));
        assert!(
            result.is_err(),
            "Config creation with empty string should fail"
        );

        // Test with None and no env var - should fail
        let result = Config::new(None);
        assert!(
            result.is_err(),
            "Config creation with no API key should fail"
        );

        // Test with whitespace-only string - should also fail
        let result = Config::new(Some("   ".to_string()));
        assert!(
            result.is_err(),
            "Config creation with whitespace-only string should fail"
        );

        // Restore original value if it existed
        if let Some(key) = original_key {
            env::set_var("GEMINI_API_KEY", key);
        }
    }

    #[test]
    fn test_config_creation_prioritizes_parameter() {
        // Set env var
        let original_key = env::var("GEMINI_API_KEY").ok();
        env::set_var("GEMINI_API_KEY", "env-key");

        // Parameter should override env var
        let config = Config::new(Some("param-key".to_string())).unwrap();
        assert_eq!(config.gemini.api_key, "param-key");

        // Restore original value
        match original_key {
            Some(key) => env::set_var("GEMINI_API_KEY", key),
            None => env::remove_var("GEMINI_API_KEY"),
        }
    }

    #[test]
    fn test_server_environment_variables() {
        // Save original values
        let original_host = env::var("SERVER_HOST").ok();
        let original_port = env::var("SERVER_PORT").ok();
        let original_url = env::var("PUBLIC_URL").ok();

        // Test with custom values
        env::set_var("SERVER_HOST", "0.0.0.0");
        env::set_var("SERVER_PORT", "8080");
        env::set_var("PUBLIC_URL", "https://api.example.com");

        let config = Config::new(Some("test-key".to_string())).unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.public_url, "https://api.example.com");

        // Restore original values
        if let Some(host) = original_host {
            env::set_var("SERVER_HOST", host);
        } else {
            env::remove_var("SERVER_HOST");
        }
        if let Some(port) = original_port {
            env::set_var("SERVER_PORT", port);
        } else {
            env::remove_var("SERVER_PORT");
        }
        if let Some(url) = original_url {
            env::set_var("PUBLIC_URL", url);
        } else {
            env::remove_var("PUBLIC_URL");
        }
    }

    #[test]
    fn test_invalid_server_port() {
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("SERVER_PORT", "invalid");
        let config = Config::new(Some("test-key".to_string())).unwrap();
        assert_eq!(config.server.port, 3000); // Should use default

        if let Some(port) = original_port {
            env::set_var("SERVER_PORT", port);
        } else {
            env::remove_var("SERVER_PORT");
        }
    }

    #[test]
    fn test_supported_formats() {
        let config = Config::default();
        assert!(config.is_supported_format("image/jpeg"));
        assert!(config.is_supported_format("image/png"));
        assert!(config.is_supported_format("image/webp"));
        assert!(!config.is_supported_format("image/gif"));
        assert!(!config.is_supported_format("image/bmp"));
        assert!(!config.is_supported_format("text/plain"));
        assert!(!config.is_supported_format(""));
    }

    #[test]
    fn test_file_extensions() {
        let config = Config::default();
        assert_eq!(config.get_file_extension("image/jpeg"), Some("jpg"));
        assert_eq!(config.get_file_extension("image/png"), Some("png"));
        assert_eq!(config.get_file_extension("image/webp"), Some("webp"));
        assert_eq!(config.get_file_extension("image/gif"), None);
        assert_eq!(config.get_file_extension("application/pdf"), None);
        assert_eq!(config.get_file_extension(""), None);
    }

    #[test]
    fn test_gemini_urls() {
        let config = Config::default();
        let base_url = config.gemini_url("test/endpoint");
        assert_eq!(
            base_url,
            "https://generativelanguage.googleapis.com/v1beta/test/endpoint"
        );

        let generate_url = config.gemini_generate_url();
        assert!(generate_url.contains("models/gemini-2.5-flash-image-preview:generateContent"));
        assert!(generate_url.starts_with("https://generativelanguage.googleapis.com/v1beta"));
    }

    #[test]
    fn test_file_size_validation() {
        let config = Config::default();

        // Valid sizes
        assert!(config.validate_file_size(1024).is_ok());
        assert!(config.validate_file_size(5 * 1024 * 1024).is_ok());
        assert!(config.validate_file_size(config.fs.max_file_size).is_ok());

        // Invalid sizes
        assert!(config
            .validate_file_size(config.fs.max_file_size + 1)
            .is_err());
        assert!(config.validate_file_size(100 * 1024 * 1024).is_err());

        // Zero size (edge case)
        assert!(config.validate_file_size(0).is_ok());
    }

    #[tokio::test]
    async fn test_ensure_directories() {
        let temp_dir = tempdir().unwrap();
        let mut config = Config::default();
        config.fs.base_dir = temp_dir.path().to_path_buf();
        config.fs.input_dir = config.fs.base_dir.join("input");
        config.fs.items_dir = config.fs.base_dir.join("items");
        config.fs.output_dir = config.fs.base_dir.join("output");

        // Directories shouldn't exist yet
        assert!(!config.fs.input_dir.exists());
        assert!(!config.fs.output_dir.exists());

        // Create directories
        config.ensure_directories().await.unwrap();

        // Directories should now exist
        assert!(config.fs.input_dir.exists());
        assert!(config.fs.output_dir.exists());
        assert!(config.fs.items_dir.exists());

        // Check that category directories exist
        use crate::models::ItemCategory;
        for category in ItemCategory::all() {
            let category_dir = config.fs.items_dir.join(category.dir_name());
            assert!(
                category_dir.exists(),
                "Category directory should exist: {:?}",
                category
            );
        }
    }

    #[test]
    fn test_category_path() {
        use crate::models::ItemCategory;
        let config = Config::default();

        let hat_path = config.category_path(&ItemCategory::Hat);
        assert!(hat_path.ends_with("hats"));

        let shoes_path = config.category_path(&ItemCategory::Shoes);
        assert!(shoes_path.ends_with("shoes"));
    }

    #[test]
    fn test_image_url_paths() {
        let config = Config::default();

        let input_url = config.image_url_path(ImageType::Input, "abc123.jpg");
        assert_eq!(
            input_url,
            "http://127.0.0.1:3000/api/images/input/abc123.jpg"
        );

        let item_url = config.image_url_path(ImageType::Item, "hats/def456.png");
        assert_eq!(
            item_url,
            "http://127.0.0.1:3000/api/images/items/hats/def456.png"
        );

        let output_url = config.image_url_path(ImageType::Output, "ghi789.webp");
        assert_eq!(
            output_url,
            "http://127.0.0.1:3000/api/images/output/ghi789.webp"
        );
    }

    #[test]
    fn test_server_addr() {
        let config = Config::default();
        assert_eq!(config.server_addr(), "127.0.0.1:3000");

        let original_host = env::var("SERVER_HOST").ok();
        let original_port = env::var("SERVER_PORT").ok();

        env::set_var("SERVER_HOST", "0.0.0.0");
        env::set_var("SERVER_PORT", "8080");

        let custom_config = Config::new(Some("test-key".to_string())).unwrap();
        assert_eq!(custom_config.server_addr(), "0.0.0.0:8080");

        // Restore environment
        if let Some(host) = original_host {
            env::set_var("SERVER_HOST", host);
        } else {
            env::remove_var("SERVER_HOST");
        }
        if let Some(port) = original_port {
            env::set_var("SERVER_PORT", port);
        } else {
            env::remove_var("SERVER_PORT");
        }
    }

    #[test]
    fn test_image_type_enum() {
        // Test that ImageType enum works correctly
        assert_eq!(ImageType::Input as i32, ImageType::Input as i32);
        assert_ne!(ImageType::Input as i32, ImageType::Item as i32);
        assert_ne!(ImageType::Item as i32, ImageType::Output as i32);
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.gemini.api_key, "test-api-key");
        assert!(config.fs.base_dir.ends_with(".mirage-fit"));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();

        // Test that config can be serialized (for debugging/logging)
        let json_result = serde_json::to_string(&config);
        assert!(json_result.is_ok());

        let json = json_result.unwrap();
        assert!(json.contains("gemini"));
        assert!(json.contains("fs"));
        assert!(json.contains("server"));
    }

    #[test]
    fn test_config_field_access() {
        let config = Config::default();

        // Test that all config fields are accessible
        assert!(!config.gemini.api_key.is_empty());
        assert!(!config.gemini.base_url.is_empty());
        assert!(!config.gemini.model.is_empty());
        assert!(config.gemini.timeout_seconds > 0);
        assert!(config.gemini.max_retries > 0);
        assert!(config.gemini.rate_limit_rpm > 0);

        assert!(
            config.fs.base_dir.is_absolute()
                || config.fs.base_dir.to_str().unwrap().contains("mirage-fit")
        );
        assert!(config.fs.max_file_size > 0);
        assert!(!config.fs.supported_formats.is_empty());
        assert!(config.fs.cache_size_limit_mb > 0);

        assert!(!config.server.host.is_empty());
        assert!(config.server.port > 0);
        assert!(!config.server.public_url.is_empty());
        assert!(config.server.max_request_size > 0);
        assert!(config.server.request_timeout > 0);
        assert!(!config.server.cors_origins.is_empty());

        assert!(config.server.static_files.cache_control_max_age > 0);
    }

    #[test]
    fn test_base_directory_resolution() {
        // This test checks that the base directory is correctly resolved
        let config = Config::default();
        let base_dir = &config.fs.base_dir;

        // Should be an absolute path containing .mirage-fit
        assert!(base_dir.is_absolute() || base_dir.to_string_lossy().contains(".mirage-fit"));
        assert!(base_dir.to_string_lossy().contains(".mirage-fit"));
    }

    #[test]
    fn test_config_constants() {
        let config = Config::default();

        // Test reasonable default values
        assert!(config.fs.max_file_size >= 1024 * 1024); // At least 1MB
        assert!(config.server.max_request_size >= config.fs.max_file_size); // Request size should accommodate file size
        assert!(config.server.request_timeout >= 30); // At least 30 seconds
        assert!(config.gemini.timeout_seconds >= 30); // At least 30 seconds for AI processing
    }

    #[test]
    fn test_error_cases() {
        // Test error case for missing API key
        let result = Config::new(None);
        if env::var("GEMINI_API_KEY").is_err() || env::var("GEMINI_API_KEY").unwrap().is_empty() {
            assert!(result.is_err());
            if let Err(error) = result {
                assert!(error.to_string().contains("API key"));
            }
        }
    }
}
