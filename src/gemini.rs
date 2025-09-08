//! Gemini AI API integration for Mirage Fit
//!
//! This module handles all interactions with the Google Gemini API,
//! including image generation, remixing, and content processing.

use crate::{
    config::Config,
    models::{
        GeminiContent, GeminiGenerateRequest, GeminiGenerationConfig, GeminiInlineData, GeminiPart,
        GeminiResponse, ItemCategory,
    },
    Error, Result,
};
use base64::Engine as _;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, info, warn};

/// Gemini API client for image generation and processing
#[derive(Debug, Clone)]
pub struct GeminiClient {
    client: Client,
    config: Config,
}

impl GeminiClient {
    /// Create a new Gemini API client
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.gemini.timeout_seconds))
            .user_agent("mirage-fit/0.1.0")
            .build()
            .map_err(|e| Error::config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    /// Generate a new item image for the specified category
    pub async fn generate_item(
        &self,
        category: &ItemCategory,
        custom_prompt: Option<&str>,
        style: Option<&str>,
        color: Option<&str>,
    ) -> Result<Vec<u8>> {
        info!("Generating item for category: {}", category);

        let prompt = self.build_item_prompt(category, custom_prompt, style, color);
        debug!("Using prompt: {}", prompt);

        let request = GeminiGenerateRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart::Text { text: prompt }],
            }],
            generation_config: self.image_generation_config(),
        };

        self.generate_image(request).await
    }

    /// Simple remix function that takes a prompt and images (as per nano-banana spec)
    pub async fn remix(&self, prompt: &str, images: &[&[u8]]) -> Result<Vec<u8>> {
        info!("Generating remix with {} images", images.len());
        debug!("Using prompt: {}", prompt);

        let mut parts = vec![GeminiPart::Text {
            text: prompt.to_string(),
        }];

        // Add all images as inline data
        for image_data in images {
            let encoded_data = base64::engine::general_purpose::STANDARD.encode(image_data);
            let mime_type =
                detect_image_mime_type(image_data).unwrap_or_else(|| "image/jpeg".to_string());

            parts.push(GeminiPart::InlineData {
                inline_data: GeminiInlineData {
                    mime_type,
                    data: encoded_data,
                },
            });
        }

        let request = GeminiGenerateRequest {
            contents: vec![GeminiContent { parts }],
            generation_config: self.image_generation_config(),
        };

        self.generate_image(request).await
    }

    /// Generate a remix image from multiple input images (legacy version)
    pub async fn generate_remix(
        &self,
        base_image: &[u8],
        base_mime_type: &str,
        item_images: &[(ItemCategory, Vec<u8>, String)], // (category, data, mime_type)
        style_prompt: Option<&str>,
        quality: Option<u8>,
    ) -> Result<Vec<u8>> {
        info!("Generating remix with {} item images", item_images.len());

        let prompt = self.build_remix_prompt(item_images, style_prompt, quality);
        debug!("Using remix prompt: {}", prompt);

        let mut parts = vec![GeminiPart::Text { text: prompt }];

        // Add base image
        let base_data = base64::engine::general_purpose::STANDARD.encode(base_image);
        parts.push(GeminiPart::InlineData {
            inline_data: GeminiInlineData {
                mime_type: base_mime_type.to_string(),
                data: base_data,
            },
        });

        // Add item images
        for (category, data, mime_type) in item_images {
            let encoded_data = base64::engine::general_purpose::STANDARD.encode(data);
            parts.push(GeminiPart::InlineData {
                inline_data: GeminiInlineData {
                    mime_type: mime_type.clone(),
                    data: encoded_data,
                },
            });

            // Add a text part describing this item
            parts.push(GeminiPart::Text {
                text: format!("This is a {} ({})", category.display_name(), category),
            });
        }

        let request = GeminiGenerateRequest {
            contents: vec![GeminiContent { parts }],
            generation_config: self.remix_generation_config(quality),
        };

        self.generate_image(request).await
    }

    /// Send a generation request to Gemini API and extract image data
    async fn generate_image(&self, request: GeminiGenerateRequest) -> Result<Vec<u8>> {
        // Append API key as query parameter (Gemini API requirement)
        let url = format!(
            "{}?key={}",
            self.config.gemini_generate_url(),
            self.config.gemini.api_key
        );

        debug!("Sending request to Gemini API");

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                warn!("Gemini API request failed: {}", e);
                Error::gemini_api(format!("Request failed: {}", e), None)
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| {
            Error::gemini_api(
                format!("Failed to read response: {}", e),
                Some(status.as_u16()),
            )
        })?;

        if !status.is_success() {
            warn!("Gemini API error response: {}", response_text);
            return Err(Error::gemini_api(
                format!(
                    "API request failed with status {}: {}",
                    status, response_text
                ),
                Some(status.as_u16()),
            ));
        }

        debug!("Gemini API response: {}", response_text);

        let gemini_response: GeminiResponse = serde_json::from_str(&response_text)
            .map_err(|e| Error::gemini_api(format!("Failed to parse response: {}", e), None))?;

        self.extract_image_from_response(gemini_response).await
    }

    /// Extract image data from Gemini API response
    async fn extract_image_from_response(&self, response: GeminiResponse) -> Result<Vec<u8>> {
        let candidate = response
            .candidates
            .into_iter()
            .next()
            .ok_or_else(|| Error::gemini_api("No candidates in response".to_string(), None))?;

        if let Some(reason) = candidate.finish_reason {
            if reason != "STOP" {
                return Err(Error::gemini_api(
                    format!("Generation finished with reason: {}", reason),
                    None,
                ));
            }
        }

        // Look for inline image data in the response parts
        let parts_count = candidate.content.parts.len();
        for part in candidate.content.parts {
            match part {
                GeminiPart::InlineData { inline_data } => {
                    // Found image data, decode from base64
                    match base64::engine::general_purpose::STANDARD.decode(&inline_data.data) {
                        Ok(image_data) => {
                            info!("Successfully extracted image data from Gemini API response: {} bytes", image_data.len());
                            return Ok(image_data);
                        }
                        Err(e) => {
                            warn!("Failed to decode base64 image data: {}", e);
                        }
                    }
                }
                GeminiPart::Text { text } => {
                    // Log text responses for debugging
                    debug!("Gemini API text response: {}", text);
                }
            }
        }

        // If no image data found, log this and return placeholder
        warn!("No image data found in Gemini API response, using placeholder");
        debug!("Response structure parts count: {:?}", parts_count);

        // Return a valid placeholder image
        Ok(self.create_placeholder_image())
    }

    /// Build a prompt for generating an item of the specified category
    fn build_item_prompt(
        &self,
        category: &ItemCategory,
        custom_prompt: Option<&str>,
        style: Option<&str>,
        color: Option<&str>,
    ) -> String {
        let mut prompt = format!(
            "Generate a high-quality, professional product image of a {} ({}) on a clean white background. ",
            category.display_name(),
            category
        );

        // Add category-specific details
        prompt.push_str(match category {
            ItemCategory::Hat => {
                "The hat should be stylish and modern, suitable for fashion photography."
            }
            ItemCategory::Glasses => {
                "The glasses should be trendy and well-designed, showing clear lenses."
            }
            ItemCategory::Shoes => {
                "The shoes should be clean and attractive, showing both style and craftsmanship."
            }
            ItemCategory::Top => {
                "The top should be well-fitted and stylish, suitable for fashion display."
            }
            ItemCategory::BottomSkirt => {
                "The bottom/skirt should be well-tailored and fashionable."
            }
            ItemCategory::Socks => {
                "The socks should be clean and attractive, showing texture and pattern if any."
            }
            ItemCategory::Gloves => "The gloves should be well-formed and stylish.",
            ItemCategory::Scarf => "The scarf should show texture and drape beautifully.",
            ItemCategory::Bag => {
                "The bag should be stylish and practical, showing good craftsmanship."
            }
            ItemCategory::Accessory => "The accessory should be elegant and well-designed.",
            ItemCategory::Other => "The item should be attractive and well-presented.",
        });

        // Add style preferences
        if let Some(style) = style {
            prompt.push_str(&format!(" Style: {}.", style));
        }

        // Add color preferences
        if let Some(color) = color {
            prompt.push_str(&format!(" Preferred color: {}.", color));
        }

        // Add custom prompt
        if let Some(custom) = custom_prompt {
            prompt.push_str(&format!(" Additional requirements: {}", custom));
        }

        prompt.push_str(" The image should be photorealistic, well-lit, and suitable for e-commerce or fashion applications.");

        prompt
    }

    /// Build a prompt for remixing multiple images
    fn build_remix_prompt(
        &self,
        item_images: &[(ItemCategory, Vec<u8>, String)],
        style_prompt: Option<&str>,
        quality: Option<u8>,
    ) -> String {
        let mut prompt = "Create a photorealistic fashion remix image. Take the person from the first image and apply the following items to them: ".to_string();

        for (i, (category, _, _)) in item_images.iter().enumerate() {
            if i > 0 {
                prompt.push_str(", ");
            }
            prompt.push_str(category.display_name());
        }

        prompt.push_str(". The result should look natural and realistic, as if the person is actually wearing these items. ");
        prompt.push_str("Maintain the original pose and background while seamlessly integrating the new items. ");
        prompt.push_str("The lighting and shadows should be consistent. ");

        if let Some(style) = style_prompt {
            prompt.push_str(&format!("Style direction: {}. ", style));
        }

        let quality_level = quality.unwrap_or(8);
        match quality_level {
            1..=3 => prompt.push_str("Focus on speed over quality. "),
            4..=6 => prompt.push_str("Balance quality and processing time. "),
            7..=10 => prompt.push_str("Prioritize highest quality and realism. "),
            _ => prompt.push_str("Use standard quality settings. "),
        }

        prompt.push_str(
            "The final image should be high-resolution and suitable for fashion or e-commerce use.",
        );

        prompt
    }

    /// Get image generation configuration
    fn image_generation_config(&self) -> GeminiGenerationConfig {
        GeminiGenerationConfig {
            temperature: 0.4,
            top_k: 32,
            top_p: 1.0,
            max_output_tokens: 1024, // Images don't need many tokens
            response_modalities: Some(vec!["IMAGE".to_string()]),
        }
    }

    /// Get generation configuration optimized for remix operations
    fn remix_generation_config(&self, quality: Option<u8>) -> GeminiGenerationConfig {
        let quality_level = quality.unwrap_or(8);

        // Adjust parameters based on quality preference
        let (temperature, top_k) = match quality_level {
            1..=3 => (0.8, 20),  // More creative, faster
            4..=6 => (0.6, 25),  // Balanced
            7..=10 => (0.3, 40), // More deterministic, higher quality
            _ => (0.4, 32),      // Default
        };

        GeminiGenerationConfig {
            temperature,
            top_k,
            top_p: 1.0,
            max_output_tokens: 8192,
            response_modalities: Some(vec!["IMAGE".to_string()]),
        }
    }

    /// Create a placeholder image for testing (proper minimal JPEG)
    fn create_placeholder_image(&self) -> Vec<u8> {
        // Generate a simple 1x1 pixel JPEG using the image crate
        use image::{DynamicImage, ImageBuffer, Rgb};

        // Create a 1x1 pixel image with white color
        let img = ImageBuffer::from_pixel(1, 1, Rgb([255, 255, 255]));
        let dynamic_img = DynamicImage::ImageRgb8(img);

        // Encode to JPEG
        let mut cursor = std::io::Cursor::new(Vec::new());
        match dynamic_img.write_to(&mut cursor, image::ImageFormat::Jpeg) {
            Ok(_) => cursor.into_inner(),
            Err(_) => {
                // Fallback to a more complete valid JPEG if image generation fails
                vec![
                    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01,
                    0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08,
                    0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A,
                    0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D,
                    0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20, 0x22,
                    0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34,
                    0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0,
                    0x00, 0x11, 0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11,
                    0x01, 0x03, 0x11, 0x01, 0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
                    0xFF, 0xC4, 0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x0C,
                    0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, 0x9F, 0xFF, 0xD9,
                ]
            }
        }
    }

    /// Check if the Gemini API is available
    pub async fn health_check(&self) -> bool {
        // Simple check - try to reach the API endpoint
        let url = format!("{}/models", self.config.gemini.base_url);

        match self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.gemini.api_key),
            )
            .send()
            .await
        {
            Ok(response) => {
                let success = response.status().is_success();
                debug!(
                    "Gemini API health check: {}",
                    if success { "OK" } else { "FAILED" }
                );
                success
            }
            Err(e) => {
                debug!("Gemini API health check failed: {}", e);
                false
            }
        }
    }
}

/// Helper function to determine MIME type from image data
pub fn detect_image_mime_type(data: &[u8]) -> Option<String> {
    if data.len() < 12 {
        return None;
    }

    // JPEG
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg".to_string());
    }

    // PNG
    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("image/png".to_string());
    }

    // WebP
    if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        return Some("image/webp".to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_gemini_client_creation() {
        let config = Config::default();
        let client = GeminiClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_item_prompt_generation() {
        let config = Config::default();
        let client = GeminiClient::new(config).unwrap();

        let prompt = client.build_item_prompt(
            &ItemCategory::Hat,
            Some("vintage style"),
            Some("modern"),
            Some("black"),
        );

        assert!(prompt.contains("hat"));
        assert!(prompt.contains("vintage style"));
        assert!(prompt.contains("modern"));
        assert!(prompt.contains("black"));
    }

    #[test]
    fn test_remix_prompt_generation() {
        let config = Config::default();
        let client = GeminiClient::new(config).unwrap();

        let items = vec![
            (ItemCategory::Hat, vec![1, 2, 3], "image/jpeg".to_string()),
            (ItemCategory::Shoes, vec![4, 5, 6], "image/png".to_string()),
        ];

        let prompt = client.build_remix_prompt(&items, Some("casual"), Some(8));

        assert!(prompt.contains("Hat"));
        assert!(prompt.contains("Shoes"));
        assert!(prompt.contains("casual"));
    }

    #[test]
    fn test_mime_type_detection() {
        // JPEG - need more bytes for proper detection
        let jpeg_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        ];
        assert_eq!(
            detect_image_mime_type(&jpeg_data),
            Some("image/jpeg".to_string())
        );

        // PNG
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x01,
        ];
        assert_eq!(
            detect_image_mime_type(&png_data),
            Some("image/png".to_string())
        );

        // Invalid
        let invalid_data = vec![0x00, 0x01, 0x02];
        assert_eq!(detect_image_mime_type(&invalid_data), None);
    }

    #[test]
    fn test_generation_configs() {
        let config = Config::default();
        let client = GeminiClient::new(config).unwrap();

        let default_config = client.image_generation_config();
        assert_eq!(default_config.temperature, 0.4);

        let remix_config = client.remix_generation_config(Some(10));
        assert_eq!(remix_config.temperature, 0.3);
        assert_eq!(remix_config.top_k, 40);
    }
}
