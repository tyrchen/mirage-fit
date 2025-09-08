//! Mirage Fit - Image remix application using Gemini AI
//!
//! This library provides the core functionality for the Mirage Fit application,
//! including Gemini API integration, file management, and web server setup.
//!
//! ## Features
//!
//! - **Gemini AI Integration**: Uses gemini-2.5-flash-image-preview model for image generation and remix
//! - **File Management**: Organized storage system for user photos, item images, and outputs
//! - **Web API**: RESTful API for image upload, item generation, and remix functionality
//! - **Static File Serving**: Embedded static files using rust-embed
//! - **Blake3 Hashing**: Efficient content-based caching and deduplication
//!
//! ## Directory Structure
//!
//! The application uses the following directory structure in `~/.mirage-fit/`:
//!
//! ```text
//! ~/.mirage-fit/
//! ├── input/           # User uploaded photos
//! ├── items/           # Generated item images by category
//! │   ├── hats/
//! │   ├── glasses/
//! │   ├── shoes/
//! │   ├── tops/
//! │   ├── pants_skirts/
//! │   ├── socks/
//! │   ├── gloves/
//! │   ├── scarves/
//! │   ├── bags/
//! │   ├── accessories/
//! │   └── others/
//! └── output/          # Generated remix images (Blake3 hash names)
//! ```

pub mod config;
pub mod error;
pub mod file_manager;
pub mod gemini;
pub mod handlers;
pub mod models;
pub mod server;
pub mod static_files;
pub mod validation;

pub use error::{Error, Result};

#[cfg(test)]
mod tests {

    #[test]
    fn test_library_imports() {
        // Basic import test to ensure all modules compile correctly
        // This test ensures that all public modules are accessible
    }
}
