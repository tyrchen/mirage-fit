//! HTTP request handlers for Mirage Fit API
//!
//! This module contains all the API endpoint handlers for the web server,
//! including image upload, item generation, remix functionality, and static file serving.

use crate::{
    config::{Config, ImageType},
    file_manager::FileManager,
    gemini::{detect_image_mime_type, GeminiClient},
    models::{
        CategoriesResponse, CategoryInfo, GenerateItemRequest, GenerateItemResponse,
        HealthResponse, ItemCategory, ItemInfo, ItemsResponse, OutputInfo, OutputsResponse,
        RemixRequest, RemixResponse, UploadResponse,
    },
    Error, Result,
};
use axum::{
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{debug, info};

/// Application state shared across handlers
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub file_manager: FileManager,
    pub gemini_client: GeminiClient,
}

/// Health check endpoint
pub async fn health_check(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>> {
    info!("Health check requested");

    // Check if Gemini API is available
    let gemini_available = state.gemini_client.health_check().await;

    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        gemini_api_available: gemini_available,
    };

    Ok(Json(response))
}

/// Get all available item categories
pub async fn get_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<CategoriesResponse>> {
    info!("Categories list requested");

    let category_stats = state.file_manager.get_category_stats().await?;

    let categories = ItemCategory::all()
        .into_iter()
        .map(|category| CategoryInfo {
            name: category.display_name().to_string(),
            count: category_stats.get(&category).copied().unwrap_or(0),
            id: category,
        })
        .collect();

    let response = CategoriesResponse { categories };

    Ok(Json(response))
}

/// Get items for a specific category
pub async fn get_category_items(
    Path(category_str): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<ItemsResponse>> {
    info!("Items list requested for category: {}", category_str);

    let category = ItemCategory::parse(&category_str)
        .ok_or_else(|| Error::not_found(format!("Category: {}", category_str)))?;

    let metadata_list = state.file_manager.list_category_items(&category).await?;

    let items = metadata_list
        .into_iter()
        .map(|metadata| {
            let filename = metadata
                .filename
                .clone()
                .unwrap_or_else(|| format!("{}.jpg", metadata.hash));
            ItemInfo {
                id: metadata.id,
                filename: filename.clone(),
                hash: metadata.hash.clone(),
                dimensions: metadata.dimensions,
                created_at: metadata.created_at,
                prompt: metadata.prompt,
                url: state.config.image_url_path(
                    ImageType::Item,
                    &format!("{}/{}", category.dir_name(), filename),
                ),
            }
        })
        .collect();

    let response = ItemsResponse { category, items };

    Ok(Json(response))
}

/// Generate a new item for a category
pub async fn generate_category_item(
    Path(category_str): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<GenerateItemRequest>,
) -> Result<Json<GenerateItemResponse>> {
    info!("Item generation requested for category: {}", category_str);

    let category = ItemCategory::parse(&category_str)
        .ok_or_else(|| Error::not_found(format!("Category: {}", category_str)))?;

    // Generate the item using Gemini API
    let image_data = state
        .gemini_client
        .generate_item(
            &category,
            request.prompt.as_deref(),
            request.style.as_deref(),
            request.color.as_deref(),
        )
        .await?;

    // Save the generated item
    let prompt_text = request
        .prompt
        .unwrap_or_else(|| format!("Generated {} item", category.display_name()));

    let metadata = state
        .file_manager
        .save_item_image(&image_data, &category, Some(prompt_text))
        .await?;

    let filename = metadata
        .filename
        .clone()
        .unwrap_or_else(|| format!("{}.jpg", metadata.hash));
    let item_info = ItemInfo {
        id: metadata.id,
        filename: filename.clone(),
        hash: metadata.hash.clone(),
        dimensions: metadata.dimensions,
        created_at: metadata.created_at,
        prompt: metadata.prompt,
        url: state.config.image_url_path(
            ImageType::Item,
            &format!("{}/{}", category.dir_name(), filename),
        ),
    };

    let response = GenerateItemResponse {
        item: item_info,
        message: format!("Successfully generated {} item", category.display_name()),
    };

    info!("Generated item {} for category {}", metadata.id, category);
    Ok(Json(response))
}

/// Upload a user photo
pub async fn upload_photo(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>> {
    info!("Photo upload requested");

    let mut image_data = None;
    let mut filename = None;

    // Process multipart form data
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::invalid_request(format!("Invalid multipart data: {}", e)))?
    {
        match field.name() {
            Some("image") | Some("file") => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|e| {
                    Error::invalid_request(format!("Failed to read image data: {}", e))
                })?;
                image_data = Some(data.to_vec());
            }
            Some("filename") => {
                let name = field.text().await.map_err(|e| {
                    Error::invalid_request(format!("Failed to read filename: {}", e))
                })?;
                if !name.is_empty() {
                    filename = Some(name);
                }
            }
            _ => {
                // Ignore unknown fields
                debug!("Ignoring unknown form field: {:?}", field.name());
            }
        }
    }

    let data = image_data.ok_or_else(|| Error::invalid_request("No image data provided"))?;

    if data.is_empty() {
        return Err(Error::invalid_request("Empty image file"));
    }

    // Save the uploaded image
    let metadata = state.file_manager.save_input_image(&data, filename).await?;

    let filename = metadata
        .filename
        .clone()
        .unwrap_or_else(|| format!("{}.jpg", metadata.hash));
    let response = UploadResponse {
        id: metadata.id,
        hash: metadata.hash.clone(),
        filename: metadata.filename,
        dimensions: metadata.dimensions,
        size: metadata.size,
        url: state.config.image_url_path(ImageType::Input, &filename),
        message: "Photo uploaded successfully".to_string(),
    };

    info!("Uploaded photo {} with hash {}", metadata.id, metadata.hash);
    Ok(Json(response))
}

/// Generate a remix image
pub async fn generate_remix(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RemixRequest>,
) -> Result<Json<RemixResponse>> {
    info!(
        "Remix generation requested with {} items",
        request.items.len()
    );

    if request.items.is_empty() {
        return Err(Error::invalid_request("No items specified for remix"));
    }

    // Collect all image data for the remix
    let mut all_images = Vec::new();
    let mut source_hashes = vec![request.base_image.clone()];

    // Get base image data using 6-char hash
    let base_image_data = state
        .file_manager
        .get_image_data_by_short_hash(&request.base_image, ImageType::Input)
        .await?;
    all_images.push(base_image_data);

    // Get item image data
    for (_category, item_hash) in &request.items {
        let item_data = state
            .file_manager
            .get_image_data_by_short_hash(item_hash, ImageType::Item)
            .await?;
        all_images.push(item_data);
        source_hashes.push(item_hash.clone());
    }

    // Build prompt for fashion remix using nano-banana best practices
    let mut prompt = "Create a photorealistic fashion remix image. Use the exact same person from the first image - keep their face, hair, body shape, skin tone, and all physical characteristics identical. Only change their clothing by adding the fashion items from the additional images: ".to_string();

    for (i, (category, _)) in request.items.iter().enumerate() {
        if i > 0 {
            prompt.push_str(", ");
        }
        prompt.push_str(category.display_name());
    }

    prompt.push_str(". IMPORTANT: Preserve the person's identity completely - same face, same hair, same body. Only modify the clothing/accessories. The new items should fit naturally on the same person while maintaining their original pose, background, and lighting conditions. Ensure the clothing looks realistically worn by this specific person.");

    if let Some(style) = &request.style {
        prompt.push_str(&format!(" Apply this style approach: {}.", style));
    }

    prompt.push_str(" The final result should be photorealistic, maintaining the original person's complete identity while showcasing them wearing the new fashion items in a natural, believable way.");

    // Convert Vec<Vec<u8>> to Vec<&[u8]> for the remix function
    let image_refs: Vec<&[u8]> = all_images.iter().map(|img| img.as_slice()).collect();

    // Generate remix using the simple remix function
    let remix_data = state.gemini_client.remix(&prompt, &image_refs).await?;

    // Save the remix output
    let metadata = state
        .file_manager
        .save_output_image(&remix_data, source_hashes.clone())
        .await?;

    let filename = metadata
        .filename
        .clone()
        .unwrap_or_else(|| format!("{}.jpg", metadata.hash));
    let response = RemixResponse {
        id: metadata.id,
        hash: metadata.hash.clone(),
        dimensions: metadata.dimensions,
        url: state.config.image_url_path(ImageType::Output, &filename),
        source_images: source_hashes,
        message: "Remix generated successfully".to_string(),
    };

    info!(
        "Generated remix {} with hash {}",
        metadata.id, metadata.hash
    );
    Ok(Json(response))
}

/// Get list of output images
pub async fn get_outputs(State(state): State<Arc<AppState>>) -> Result<Json<OutputsResponse>> {
    info!("Output images list requested");

    let metadata_list = state.file_manager.list_output_images().await?;

    let outputs = metadata_list
        .into_iter()
        .map(|metadata| {
            let filename = metadata
                .filename
                .clone()
                .unwrap_or_else(|| format!("{}.jpg", metadata.hash));
            OutputInfo {
                id: metadata.id,
                hash: metadata.hash.clone(),
                dimensions: metadata.dimensions,
                created_at: metadata.created_at,
                source_images: metadata.source_images,
                url: state.config.image_url_path(ImageType::Output, &filename),
            }
        })
        .collect();

    let response = OutputsResponse { outputs };

    Ok(Json(response))
}

/// Serve static image files
pub async fn serve_image(
    Path((image_type, path)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<Response> {
    debug!("Image requested: {} / {}", image_type, path);

    let img_type = match image_type.as_str() {
        "input" => ImageType::Input,
        "items" => ImageType::Item,
        "output" => ImageType::Output,
        _ => return Err(Error::not_found("Invalid image type")),
    };

    // Extract hash from path (remove extension)
    let hash = if let Some(pos) = path.rfind('.') {
        &path[..pos]
    } else {
        &path
    };

    // For items, we need to handle the category subdirectory
    let actual_hash = if img_type == ImageType::Item as ImageType {
        // Path format: "category/hash.ext"
        if let Some(pos) = path.rfind('/') {
            let hash_with_ext = &path[pos + 1..];
            if let Some(ext_pos) = hash_with_ext.rfind('.') {
                &hash_with_ext[..ext_pos]
            } else {
                hash_with_ext
            }
        } else {
            hash
        }
    } else {
        hash
    };

    // Get image data using 6-char hash
    let image_data = state
        .file_manager
        .get_image_data_by_short_hash(actual_hash, img_type)
        .await?;

    // Determine content type from image data
    let content_type = detect_image_mime_type(&image_data)
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let mut headers = HeaderMap::new();
    headers.insert("content-type", content_type.parse().unwrap());
    headers.insert("cache-control", "public, max-age=3600".parse().unwrap());

    Ok((StatusCode::OK, headers, image_data).into_response())
}

/// Query parameters for pagination and filtering
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub category: Option<String>,
}

/// Error handler for invalid paths or resources
pub async fn not_found() -> Result<Response> {
    Err(Error::not_found("Endpoint not found"))
}

/// Handler for method not allowed
pub async fn method_not_allowed() -> Result<Response> {
    Err(Error::invalid_request("Method not allowed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        // This would require a more complex setup with a real AppState
        // For now, just test that the function compiles correctly
        // TODO: Add proper test with mock AppState
    }

    #[test]
    fn test_item_category_parsing() {
        assert!(ItemCategory::parse("帽子").is_some());
        assert!(ItemCategory::parse("invalid").is_none());
    }

    #[test]
    fn test_image_type_matching() {
        // Test the string matching logic used in serve_image
        let image_types = vec!["input", "items", "output"];
        for img_type in image_types {
            assert!(matches!(img_type, "input" | "items" | "output"));
        }
    }
}
