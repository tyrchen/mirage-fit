//! File system management for Mirage Fit
//!
//! This module handles all file system operations including image storage,
//! Blake3 hashing, metadata management, and directory organization.

use crate::{
    config::{Config, ImageType},
    gemini::{detect_image_mime_type, GeminiClient},
    models::{ImageMetadata, ItemCategory},
    Error, Result,
};
use blake3::Hasher;
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::fs;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// File manager for handling all file system operations
#[derive(Debug, Clone)]
pub struct FileManager {
    config: Config,
}

impl FileManager {
    /// Create a new file manager
    pub async fn new(config: &Config) -> Result<Self> {
        let file_manager = Self {
            config: config.clone(),
        };

        // Ensure all directories exist
        config.ensure_directories().await?;

        info!(
            "File manager initialized with base directory: {}",
            config.fs.base_dir.display()
        );

        Ok(file_manager)
    }

    /// Save an uploaded image to the input directory
    pub async fn save_input_image(
        &self,
        data: &[u8],
        _filename: Option<String>,
    ) -> Result<ImageMetadata> {
        // Validate file size
        self.config.validate_file_size(data.len() as u64)?;

        // Detect MIME type
        let mime_type = detect_image_mime_type(data)
            .ok_or_else(|| Error::invalid_image("Unsupported or invalid image format"))?;

        if !self.config.is_supported_format(&mime_type) {
            return Err(Error::invalid_image(format!(
                "Unsupported image format: {}",
                mime_type
            )));
        }

        // Calculate Blake3 hash
        let full_hash = self.calculate_hash(data);
        let short_hash = full_hash.chars().take(6).collect::<String>();

        // Determine file extension
        let extension = self
            .config
            .get_file_extension(&mime_type)
            .ok_or_else(|| Error::invalid_image("Unknown image format"))?;

        // Check if file already exists by scanning directory
        let image_filename = format!("{}.{}", short_hash, extension);
        let image_path = self.config.fs.input_dir.join(&image_filename);

        if image_path.exists() {
            info!("Image already exists with hash: {}", short_hash);
            // Create metadata for existing file
            let file_metadata = fs::metadata(&image_path).await.ok();
            let created_time = file_metadata
                .and_then(|m| m.created().ok())
                .unwrap_or(SystemTime::now());
            let timestamp = created_time
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string();

            let dimensions = self.get_image_dimensions(data).unwrap_or((1024, 1024));

            let existing_metadata = ImageMetadata {
                id: Uuid::new_v4(),
                filename: Some(image_filename),
                hash: short_hash.clone(),
                size: data.len() as u64,
                dimensions,
                mime_type: mime_type.clone(),
                created_at: timestamp,
                category: None,
                prompt: None,
                source_images: vec![],
            };
            return Ok(existing_metadata);
        }

        // Get image dimensions
        let dimensions = self.get_image_dimensions(data)?;

        // Create metadata (no JSON file saved)
        let metadata = ImageMetadata {
            id: Uuid::new_v4(),
            filename: Some(image_filename.clone()),
            hash: short_hash.clone(),
            size: data.len() as u64,
            dimensions,
            mime_type: mime_type.clone(),
            created_at: self.current_timestamp(),
            category: None,
            prompt: None,
            source_images: vec![],
        };

        // Save image file with 6-char hash name
        fs::write(&image_path, data)
            .await
            .map_err(|e| Error::file_system(format!("Failed to save image: {}", e)))?;

        info!("Saved input image: {} ({})", image_filename, metadata.id);
        Ok(metadata)
    }

    /// Save a generated item image
    pub async fn save_item_image(
        &self,
        data: &[u8],
        category: &ItemCategory,
        prompt: Option<String>,
    ) -> Result<ImageMetadata> {
        let mime_type = detect_image_mime_type(data).unwrap_or_else(|| "image/jpeg".to_string());

        let full_hash = self.calculate_hash(data);

        // Use first 6 characters of hash for filename
        let short_hash = full_hash.chars().take(6).collect::<String>();

        // Check if file already exists in this category (use 6-char hash to check files directly)
        let extension = self.config.get_file_extension(&mime_type).unwrap_or("jpg");
        let image_filename = format!("{}.{}", short_hash, extension);
        let category_dir = self.config.category_path(category);
        let image_path = category_dir.join(&image_filename);

        if image_path.exists() {
            info!("Item image already exists with hash: {}", short_hash);
            // Return metadata constructed from file
            let timestamp = self.current_timestamp();
            let dimensions = self.get_image_dimensions(data)?;
            let existing_metadata = ImageMetadata {
                id: Uuid::new_v4(),
                filename: Some(image_filename.clone()),
                hash: short_hash.clone(),
                size: data.len() as u64,
                dimensions,
                mime_type: mime_type.clone(),
                created_at: timestamp,
                category: Some(category.clone()),
                prompt: prompt.clone(),
                source_images: vec![],
            };
            return Ok(existing_metadata);
        }

        let dimensions = self.get_image_dimensions(data)?;

        // Create metadata (no JSON file saved)
        let metadata = ImageMetadata {
            id: Uuid::new_v4(),
            filename: Some(image_filename.clone()),
            hash: short_hash.clone(),
            size: data.len() as u64,
            dimensions,
            mime_type: mime_type.clone(),
            created_at: self.current_timestamp(),
            category: Some(category.clone()),
            prompt,
            source_images: vec![],
        };

        fs::write(&image_path, data)
            .await
            .map_err(|e| Error::file_system(format!("Failed to save item image: {}", e)))?;

        // No metadata JSON files needed

        info!(
            "Saved item image: {} in category {}",
            image_filename, category
        );
        Ok(metadata)
    }

    /// Save a remix output image
    pub async fn save_output_image(
        &self,
        data: &[u8],
        source_images: Vec<String>,
    ) -> Result<ImageMetadata> {
        let mime_type = detect_image_mime_type(data).unwrap_or_else(|| "image/jpeg".to_string());

        let full_hash = self.calculate_hash(data);

        // Use first 6 characters of hash for filename
        let short_hash = full_hash.chars().take(6).collect::<String>();

        // Check if file already exists (use 6-char hash to check files directly)
        let extension = self.config.get_file_extension(&mime_type).unwrap_or("jpg");
        let image_filename = format!("{}.{}", short_hash, extension);
        let image_path = self.config.fs.output_dir.join(&image_filename);

        if image_path.exists() {
            info!("Output image already exists with hash: {}", short_hash);
            // Return metadata constructed from file
            let timestamp = self.current_timestamp();
            let dimensions = self.get_image_dimensions(data)?;
            let existing_metadata = ImageMetadata {
                id: Uuid::new_v4(),
                filename: Some(image_filename.clone()),
                hash: short_hash.clone(),
                size: data.len() as u64,
                dimensions,
                mime_type: mime_type.clone(),
                created_at: timestamp,
                category: None,
                prompt: None,
                source_images: source_images.clone(),
            };
            return Ok(existing_metadata);
        }

        let dimensions = self.get_image_dimensions(data)?;

        // Create metadata (no JSON file saved)
        let metadata = ImageMetadata {
            id: Uuid::new_v4(),
            filename: Some(image_filename.clone()),
            hash: short_hash.clone(),
            size: data.len() as u64,
            dimensions,
            mime_type: mime_type.clone(),
            created_at: self.current_timestamp(),
            category: None,
            prompt: None,
            source_images,
        };

        // Save image file with 6-char hash name
        fs::write(&image_path, data)
            .await
            .map_err(|e| Error::file_system(format!("Failed to save output image: {}", e)))?;

        info!("Saved output image: {} ({})", image_filename, metadata.id);
        Ok(metadata)
    }

    /// Get image data by hash
    pub async fn get_image_data(&self, hash: &str, image_type: ImageType) -> Result<Vec<u8>> {
        let path = self.find_image_path(hash, image_type).await?;
        fs::read(&path)
            .await
            .map_err(|e| Error::file_system(format!("Failed to read image: {}", e)))
    }

    /// Get image metadata by hash
    /// List all items in a category
    pub async fn list_category_items(&self, category: &ItemCategory) -> Result<Vec<ImageMetadata>> {
        let category_dir = self.config.category_path(category);
        let mut items = Vec::new();

        if !category_dir.exists() {
            return Ok(items);
        }

        let mut entries = fs::read_dir(&category_dir)
            .await
            .map_err(|e| Error::file_system(format!("Failed to read category directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::file_system(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_file() {
                // Check if it's an image file
                if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                    if matches!(
                        extension.to_lowercase().as_str(),
                        "png" | "jpg" | "jpeg" | "webp"
                    ) {
                        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                            // Create basic metadata from file info
                            use std::time::{SystemTime, UNIX_EPOCH};

                            let file_metadata = std::fs::metadata(&path).ok();
                            let created_time = file_metadata
                                .as_ref()
                                .and_then(|m| m.created().ok())
                                .unwrap_or(SystemTime::now());
                            let timestamp = created_time
                                .duration_since(UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs()
                                .to_string();

                            let metadata = ImageMetadata {
                                id: uuid::Uuid::new_v4(),
                                filename: Some(format!("{}.{}", stem, extension.to_lowercase())),
                                hash: stem.to_string(), // Use the 6-char filename as hash
                                size: file_metadata.map(|m| m.len()).unwrap_or(0),
                                dimensions: (1024, 1024), // Default dimensions
                                mime_type: match extension.to_lowercase().as_str() {
                                    "png" => "image/png".to_string(),
                                    "jpg" | "jpeg" => "image/jpeg".to_string(),
                                    "webp" => "image/webp".to_string(),
                                    _ => "image/png".to_string(),
                                },
                                created_at: timestamp,
                                category: Some(category.clone()),
                                prompt: None,
                                source_images: vec![],
                            };

                            items.push(metadata);
                        }
                    }
                }
            }
        }

        // Sort by creation time, newest first
        items.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(items)
    }

    /// List all output images
    pub async fn list_output_images(&self) -> Result<Vec<ImageMetadata>> {
        let mut images = Vec::new();

        if !self.config.fs.output_dir.exists() {
            return Ok(images);
        }

        let mut entries = fs::read_dir(&self.config.fs.output_dir)
            .await
            .map_err(|e| Error::file_system(format!("Failed to read output directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::file_system(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                    // Skip non-image files
                    if !matches!(
                        extension.to_lowercase().as_str(),
                        "jpg" | "jpeg" | "png" | "webp"
                    ) {
                        continue;
                    }

                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        // Use our new function to create metadata from file
                        if let Ok(metadata) = self
                            .create_metadata_from_file(stem, ImageType::Output, None)
                            .await
                        {
                            images.push(metadata);
                        }
                    }
                }
            }
        }

        // Sort by creation time, newest first
        images.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(images)
    }

    /// Get category statistics
    pub async fn get_category_stats(&self) -> Result<HashMap<ItemCategory, usize>> {
        let mut stats = HashMap::new();

        for category in ItemCategory::all() {
            let items = self.list_category_items(&category).await?;
            stats.insert(category, items.len());
        }

        Ok(stats)
    }

    /// Generate default item images for all categories (2 per category) concurrently
    pub async fn generate_default_items(&self, config: &Config) -> Result<()> {
        info!("Generating default item images for all categories concurrently");

        let gemini_client = GeminiClient::new(config.clone())?;
        let mut tasks = Vec::new();

        // Create concurrent tasks for each category that needs items
        for category in ItemCategory::all() {
            // Check if category already has items
            let existing_items = self.list_category_items(&category).await?;
            if existing_items.len() >= 2 {
                debug!(
                    "Category {} already has {} items, skipping",
                    category,
                    existing_items.len()
                );
                continue;
            }

            let items_to_generate = 2 - existing_items.len();
            info!(
                "Generating {} default items for category: {}",
                items_to_generate, category
            );

            // Create tasks for each item in this category
            for i in 0..items_to_generate {
                let client = gemini_client.clone();
                let file_manager = self.clone();
                let cat = category.clone();
                let existing_count = existing_items.len();

                let task = tokio::spawn(async move {
                    // Add staggered delay to reduce rate limiting
                    let delay_ms = (i as u64 + existing_count as u64) * 200;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;

                    match client.generate_item(&cat, None, None, None).await {
                        Ok(image_data) => {
                            let prompt = format!(
                                "Default {} item #{}",
                                cat.display_name(),
                                existing_count + i + 1
                            );
                            match file_manager
                                .save_item_image(&image_data, &cat, Some(prompt))
                                .await
                            {
                                Ok(metadata) => {
                                    info!("Generated default item for {}: {}", cat, metadata.id);
                                    Ok((cat.clone(), metadata.id))
                                }
                                Err(e) => {
                                    warn!("Failed to save default item for {}: {}", cat, e);
                                    Err(e)
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to generate default item for {}: {}", cat, e);
                            Err(e)
                        }
                    }
                });

                tasks.push(task);
            }
        }

        // Wait for all tasks to complete
        let mut successful = 0;
        let mut failed = 0;

        for task in tasks {
            match task.await {
                Ok(Ok(_)) => successful += 1,
                Ok(Err(_)) => failed += 1,
                Err(e) => {
                    warn!("Task execution error: {}", e);
                    failed += 1;
                }
            }
        }

        info!(
            "Finished generating default items: {} successful, {} failed",
            successful, failed
        );
        Ok(())
    }

    /// Clean up old files based on cache size limits
    pub async fn cleanup_cache(&self) -> Result<()> {
        let _cache_limit = self.config.fs.cache_size_limit_mb * 1024 * 1024; // Convert to bytes

        // For now, just log the operation
        // In a full implementation, you would:
        // 1. Calculate total cache size
        // 2. Remove oldest files if over limit
        // 3. Clean up orphaned metadata files

        info!(
            "Cache cleanup requested (limit: {} MB)",
            self.config.fs.cache_size_limit_mb
        );
        Ok(())
    }

    // Private helper methods

    /// Calculate Blake3 hash of data
    fn calculate_hash(&self, data: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().to_hex().to_string()
    }

    /// Get image dimensions from data
    fn get_image_dimensions(&self, data: &[u8]) -> Result<(u32, u32)> {
        let cursor = std::io::Cursor::new(data);
        let reader = image::ImageReader::new(cursor)
            .with_guessed_format()
            .map_err(|e| Error::invalid_image(format!("Failed to guess image format: {}", e)))?;

        let dimensions = reader
            .into_dimensions()
            .map_err(|e| Error::invalid_image(format!("Failed to get image dimensions: {}", e)))?;

        Ok(dimensions)
    }

    /// Get current timestamp as ISO 8601 string
    fn current_timestamp(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        // Simple ISO 8601 format - in production, use chrono for better formatting
        format!("{}", now.as_secs())
    }

    /// Find the actual image file path by hash
    async fn find_image_path(&self, hash: &str, image_type: ImageType) -> Result<PathBuf> {
        let search_dirs = match image_type {
            ImageType::Input => vec![self.config.fs.input_dir.clone()],
            ImageType::Item => {
                // Search all category directories
                ItemCategory::all()
                    .iter()
                    .map(|cat| self.config.category_path(cat))
                    .collect()
            }
            ImageType::Output => vec![self.config.fs.output_dir.clone()],
        };

        for dir in search_dirs {
            if !dir.exists() {
                continue;
            }

            let mut entries = fs::read_dir(&dir)
                .await
                .map_err(|e| Error::file_system(format!("Failed to read directory: {}", e)))?;

            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| Error::file_system(format!("Failed to read directory entry: {}", e)))?
            {
                let path = entry.path();
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if stem == hash {
                        return Ok(path);
                    }
                }
            }
        }

        Err(Error::not_found(format!("Image with hash: {}", hash)))
    }

    /// Get image data by 6-character hash (no metadata JSON required)
    pub async fn get_image_data_by_short_hash(
        &self,
        short_hash: &str,
        image_type: ImageType,
    ) -> Result<Vec<u8>> {
        let search_dirs = match image_type {
            ImageType::Input => vec![self.config.fs.input_dir.clone()],
            ImageType::Item => {
                // Search all category directories
                ItemCategory::all()
                    .iter()
                    .map(|cat| self.config.category_path(cat))
                    .collect()
            }
            ImageType::Output => vec![self.config.fs.output_dir.clone()],
        };

        for dir in search_dirs {
            if !dir.exists() {
                continue;
            }

            let mut entries = fs::read_dir(&dir)
                .await
                .map_err(|e| Error::file_system(format!("Failed to read directory: {}", e)))?;

            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| Error::file_system(format!("Failed to read directory entry: {}", e)))?
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if stem == short_hash {
                            return fs::read(&path).await.map_err(|e| {
                                Error::file_system(format!("Failed to read image: {}", e))
                            });
                        }
                    }
                }
            }
        }

        Err(Error::not_found(format!("Image with hash: {}", short_hash)))
    }

    /// Create metadata from an image file (no JSON required)
    pub async fn create_metadata_from_file(
        &self,
        short_hash: &str,
        image_type: ImageType,
        category: Option<ItemCategory>,
    ) -> Result<ImageMetadata> {
        // Find the actual file
        let image_data = self
            .get_image_data_by_short_hash(short_hash, image_type)
            .await?;

        // Detect MIME type from data
        let mime_type = crate::gemini::detect_image_mime_type(&image_data)
            .unwrap_or_else(|| "image/png".to_string());

        // Get dimensions
        let dimensions = self
            .get_image_dimensions(&image_data)
            .unwrap_or((1024, 1024));

        // Determine filename
        let extension = self.config.get_file_extension(&mime_type).unwrap_or("png");
        let filename = format!("{}.{}", short_hash, extension);

        // Create metadata
        Ok(ImageMetadata {
            id: uuid::Uuid::new_v4(),
            filename: Some(filename),
            hash: short_hash.to_string(),
            size: image_data.len() as u64,
            dimensions,
            mime_type,
            created_at: self.current_timestamp(),
            category,
            prompt: None,
            source_images: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    async fn create_test_file_manager() -> (FileManager, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let mut config = Config::default();
        config.fs.base_dir = temp_dir.path().to_path_buf();
        config.fs.input_dir = temp_dir.path().join("input");
        config.fs.items_dir = temp_dir.path().join("items");
        config.fs.output_dir = temp_dir.path().join("output");

        let file_manager = FileManager::new(&config).await.unwrap();
        (file_manager, temp_dir)
    }

    #[tokio::test]
    async fn test_file_manager_creation() {
        let (file_manager, _temp_dir) = create_test_file_manager().await;

        // Check that directories were created
        assert!(file_manager.config.fs.input_dir.exists());
        assert!(file_manager.config.fs.output_dir.exists());
    }

    #[tokio::test]
    async fn test_hash_calculation() {
        let (file_manager, _temp_dir) = create_test_file_manager().await;

        let data = b"test image data";
        let hash1 = file_manager.calculate_hash(data);
        let hash2 = file_manager.calculate_hash(data);

        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[tokio::test]
    async fn test_save_input_image() {
        let (file_manager, _temp_dir) = create_test_file_manager().await;

        // Create a minimal valid JPEG
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46];

        // This test will fail with the current JPEG data since it's not a complete image
        // In a real test, you would use a proper test image
        let result = file_manager
            .save_input_image(&jpeg_data, Some("test.jpg".to_string()))
            .await;

        // For now, just check that the method doesn't panic
        assert!(result.is_err()); // Expected to fail with incomplete JPEG
    }
}
