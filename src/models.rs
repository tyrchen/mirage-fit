//! Data models and types for Mirage Fit
//!
//! This module defines the data structures used throughout the application,
//! including API request/response types, image metadata, and item categories.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported item categories for fashion items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    #[serde(rename = "hat")]
    Hat,
    #[serde(rename = "glasses")]
    Glasses,
    #[serde(rename = "shoes")]
    Shoes,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    BottomSkirt,
    #[serde(rename = "socks")]
    Socks,
    #[serde(rename = "gloves")]
    Gloves,
    #[serde(rename = "scarf")]
    Scarf,
    #[serde(rename = "bag")]
    Bag,
    #[serde(rename = "accessory")]
    Accessory,
    #[serde(rename = "other")]
    Other,
}

impl ItemCategory {
    /// Get all available categories
    pub fn all() -> Vec<Self> {
        vec![
            Self::Hat,
            Self::Glasses,
            Self::Shoes,
            Self::Top,
            Self::BottomSkirt,
            Self::Socks,
            Self::Gloves,
            Self::Scarf,
            Self::Bag,
            Self::Accessory,
            Self::Other,
        ]
    }

    /// Get the display name (English name) for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Hat => "Hat",
            Self::Glasses => "Glasses",
            Self::Shoes => "Shoes",
            Self::Top => "Top",
            Self::BottomSkirt => "Bottom/Skirt",
            Self::Socks => "Socks",
            Self::Gloves => "Gloves",
            Self::Scarf => "Scarf",
            Self::Bag => "Bag",
            Self::Accessory => "Accessory",
            Self::Other => "Other",
        }
    }

    /// Get the directory name (safe for file system, using English)
    pub fn dir_name(&self) -> &'static str {
        match self {
            Self::Hat => "hats",
            Self::Glasses => "glasses",
            Self::Shoes => "shoes",
            Self::Top => "tops",
            Self::BottomSkirt => "bottoms",
            Self::Socks => "socks",
            Self::Gloves => "gloves",
            Self::Scarf => "scarves",
            Self::Bag => "bags",
            Self::Accessory => "accessories",
            Self::Other => "other",
        }
    }

    /// Parse from string (display name or directory name)
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            // English names (primary)
            "hat" | "Hat" | "hats" => Some(Self::Hat),
            "glasses" | "Glasses" => Some(Self::Glasses),
            "shoes" | "Shoes" => Some(Self::Shoes),
            "top" | "Top" | "tops" => Some(Self::Top),
            "bottom" | "Bottom" | "bottoms" | "Bottom/Skirt" => Some(Self::BottomSkirt),
            "socks" | "Socks" => Some(Self::Socks),
            "gloves" | "Gloves" => Some(Self::Gloves),
            "scarf" | "Scarf" | "scarves" => Some(Self::Scarf),
            "bag" | "Bag" | "bags" => Some(Self::Bag),
            "accessory" | "Accessory" | "accessories" => Some(Self::Accessory),
            "other" | "Other" => Some(Self::Other),
            // Chinese names (backward compatibility)
            "帽子" => Some(Self::Hat),
            "眼镜" => Some(Self::Glasses),
            "鞋子" => Some(Self::Shoes),
            "上衣" => Some(Self::Top),
            "裤子/裙子" | "裤子_裙子" => Some(Self::BottomSkirt),
            "袜子" => Some(Self::Socks),
            "手套" => Some(Self::Gloves),
            "围巾" => Some(Self::Scarf),
            "包包" => Some(Self::Bag),
            "饰品" => Some(Self::Accessory),
            "其他" => Some(Self::Other),
            _ => None,
        }
    }
}

impl std::fmt::Display for ItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Metadata for an image file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    /// Unique identifier for the image
    pub id: Uuid,
    /// Original filename (if available)
    pub filename: Option<String>,
    /// Blake3 hash of the image content
    pub hash: String,
    /// File size in bytes
    pub size: u64,
    /// Image dimensions (width, height)
    pub dimensions: (u32, u32),
    /// MIME type (e.g., "image/jpeg", "image/png")
    pub mime_type: String,
    /// Creation timestamp (ISO 8601)
    pub created_at: String,
    /// Category (for item images)
    pub category: Option<ItemCategory>,
    /// Generation prompt (for AI-generated images)
    pub prompt: Option<String>,
    /// Source images used for remix (list of hashes)
    pub source_images: Vec<String>,
}

/// API response for listing categories
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub categories: Vec<CategoryInfo>,
}

/// Information about a category
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    /// Category identifier
    pub id: ItemCategory,
    /// Display name
    pub name: String,
    /// Number of items in this category
    pub count: usize,
}

/// API response for listing items in a category
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemsResponse {
    pub category: ItemCategory,
    pub items: Vec<ItemInfo>,
}

/// Information about an item image
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemInfo {
    /// Unique identifier
    pub id: Uuid,
    /// Filename
    pub filename: String,
    /// Blake3 hash
    pub hash: String,
    /// Image dimensions
    pub dimensions: (u32, u32),
    /// Creation timestamp
    pub created_at: String,
    /// Generation prompt (if AI-generated)
    pub prompt: Option<String>,
    /// URL to access the image
    pub url: String,
}

/// Request to generate a new item for a category
#[derive(Debug, Deserialize)]
pub struct GenerateItemRequest {
    /// Optional custom prompt for item generation
    pub prompt: Option<String>,
    /// Style preferences
    pub style: Option<String>,
    /// Color preferences
    pub color: Option<String>,
}

/// Response after generating a new item
#[derive(Debug, Serialize)]
pub struct GenerateItemResponse {
    pub item: ItemInfo,
    pub message: String,
}

/// Request to upload a user photo
#[derive(Debug)]
pub struct UploadRequest {
    /// Image file data
    pub data: Vec<u8>,
    /// Original filename
    pub filename: Option<String>,
}

/// Response after uploading a photo
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub id: Uuid,
    pub hash: String,
    pub filename: Option<String>,
    pub dimensions: (u32, u32),
    pub size: u64,
    pub url: String,
    pub message: String,
}

/// Request to remix multiple images
#[derive(Debug, Deserialize)]
pub struct RemixRequest {
    /// Base user photo hash
    pub base_image: String,
    /// Item images to apply (category, hash pairs)
    pub items: Vec<(ItemCategory, String)>,
    /// Optional custom style prompt
    pub style: Option<String>,
    /// Output quality preference (1-10)
    pub quality: Option<u8>,
}

/// Response after generating a remix
#[derive(Debug, Serialize)]
pub struct RemixResponse {
    pub id: Uuid,
    pub hash: String,
    pub dimensions: (u32, u32),
    pub url: String,
    pub source_images: Vec<String>,
    pub message: String,
}

/// API response for listing output images
#[derive(Debug, Serialize)]
pub struct OutputsResponse {
    pub outputs: Vec<OutputInfo>,
}

/// Information about an output image
#[derive(Debug, Serialize)]
pub struct OutputInfo {
    pub id: Uuid,
    pub hash: String,
    pub dimensions: (u32, u32),
    pub created_at: String,
    pub source_images: Vec<String>,
    pub url: String,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub gemini_api_available: bool,
}

/// Generic API error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

/// Error details in API responses
#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub message: String,
    pub code: u16,
}

/// Gemini API request for image generation
#[derive(Debug, Serialize)]
pub struct GeminiGenerateRequest {
    pub contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    pub generation_config: GeminiGenerationConfig,
}

/// Content for Gemini API request
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

/// Part of Gemini content (text or image)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeminiPart {
    Text {
        text: String,
    },
    InlineData {
        #[serde(rename = "inlineData")]
        inline_data: GeminiInlineData,
    },
}

/// Inline data for Gemini API (base64 encoded images)
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiInlineData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub data: String, // base64 encoded
}

/// Generation configuration for Gemini API
#[derive(Debug, Serialize)]
pub struct GeminiGenerationConfig {
    pub temperature: f32,
    #[serde(rename = "topK")]
    pub top_k: u32,
    #[serde(rename = "topP")]
    pub top_p: f32,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: u32,
    #[serde(rename = "responseModalities", skip_serializing_if = "Option::is_none")]
    pub response_modalities: Option<Vec<String>>,
}

/// Gemini API response
#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

/// Gemini response candidate
#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
    #[serde(rename = "finishReason")]
    pub finish_reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_category_all() {
        let categories = ItemCategory::all();
        assert_eq!(categories.len(), 11);
        assert!(categories.contains(&ItemCategory::Hat));
        assert!(categories.contains(&ItemCategory::Other));
    }

    #[test]
    fn test_item_category_display_name() {
        assert_eq!(ItemCategory::Hat.display_name(), "Hat");
        assert_eq!(ItemCategory::BottomSkirt.display_name(), "Bottom/Skirt");
        assert_eq!(ItemCategory::Other.display_name(), "Other");
    }

    #[test]
    fn test_item_category_dir_name() {
        assert_eq!(ItemCategory::Hat.dir_name(), "hats");
        assert_eq!(ItemCategory::BottomSkirt.dir_name(), "bottoms");
        assert_eq!(ItemCategory::Other.dir_name(), "other");
    }

    #[test]
    fn test_item_category_from_str() {
        // Test English names
        assert_eq!(ItemCategory::parse("Hat"), Some(ItemCategory::Hat));
        assert_eq!(ItemCategory::parse("hats"), Some(ItemCategory::Hat));
        assert_eq!(
            ItemCategory::parse("Bottom/Skirt"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(
            ItemCategory::parse("bottoms"),
            Some(ItemCategory::BottomSkirt)
        );
        // Test Chinese backward compatibility
        assert_eq!(ItemCategory::parse("帽子"), Some(ItemCategory::Hat));
        assert_eq!(
            ItemCategory::parse("裤子/裙子"),
            Some(ItemCategory::BottomSkirt)
        );
        // Test invalid
        assert_eq!(ItemCategory::parse("invalid"), None);
    }

    #[test]
    fn test_serialization() {
        let category = ItemCategory::Hat;
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, r#""hat""#);

        let deserialized: ItemCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, category);
    }
}
