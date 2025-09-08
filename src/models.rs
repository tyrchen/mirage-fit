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
#[derive(Debug, Serialize, Deserialize)]
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
    use serde_json::json;

    #[test]
    fn test_item_category_all() {
        let categories = ItemCategory::all();
        assert_eq!(categories.len(), 11);
        assert!(categories.contains(&ItemCategory::Hat));
        assert!(categories.contains(&ItemCategory::Other));

        // Test that all expected categories are present
        let expected_categories = [
            ItemCategory::Hat,
            ItemCategory::Glasses,
            ItemCategory::Shoes,
            ItemCategory::Top,
            ItemCategory::BottomSkirt,
            ItemCategory::Socks,
            ItemCategory::Gloves,
            ItemCategory::Scarf,
            ItemCategory::Bag,
            ItemCategory::Accessory,
            ItemCategory::Other,
        ];

        for expected in &expected_categories {
            assert!(
                categories.contains(expected),
                "Missing category: {:?}",
                expected
            );
        }
    }

    #[test]
    fn test_item_category_display_name() {
        assert_eq!(ItemCategory::Hat.display_name(), "Hat");
        assert_eq!(ItemCategory::Glasses.display_name(), "Glasses");
        assert_eq!(ItemCategory::Shoes.display_name(), "Shoes");
        assert_eq!(ItemCategory::Top.display_name(), "Top");
        assert_eq!(ItemCategory::BottomSkirt.display_name(), "Bottom/Skirt");
        assert_eq!(ItemCategory::Socks.display_name(), "Socks");
        assert_eq!(ItemCategory::Gloves.display_name(), "Gloves");
        assert_eq!(ItemCategory::Scarf.display_name(), "Scarf");
        assert_eq!(ItemCategory::Bag.display_name(), "Bag");
        assert_eq!(ItemCategory::Accessory.display_name(), "Accessory");
        assert_eq!(ItemCategory::Other.display_name(), "Other");
    }

    #[test]
    fn test_item_category_dir_name() {
        assert_eq!(ItemCategory::Hat.dir_name(), "hats");
        assert_eq!(ItemCategory::Glasses.dir_name(), "glasses");
        assert_eq!(ItemCategory::Shoes.dir_name(), "shoes");
        assert_eq!(ItemCategory::Top.dir_name(), "tops");
        assert_eq!(ItemCategory::BottomSkirt.dir_name(), "bottoms");
        assert_eq!(ItemCategory::Socks.dir_name(), "socks");
        assert_eq!(ItemCategory::Gloves.dir_name(), "gloves");
        assert_eq!(ItemCategory::Scarf.dir_name(), "scarves");
        assert_eq!(ItemCategory::Bag.dir_name(), "bags");
        assert_eq!(ItemCategory::Accessory.dir_name(), "accessories");
        assert_eq!(ItemCategory::Other.dir_name(), "other");
    }

    #[test]
    fn test_item_category_parse_comprehensive() {
        // Test English names (primary)
        assert_eq!(ItemCategory::parse("hat"), Some(ItemCategory::Hat));
        assert_eq!(ItemCategory::parse("Hat"), Some(ItemCategory::Hat));
        assert_eq!(ItemCategory::parse("hats"), Some(ItemCategory::Hat));

        assert_eq!(ItemCategory::parse("glasses"), Some(ItemCategory::Glasses));
        assert_eq!(ItemCategory::parse("Glasses"), Some(ItemCategory::Glasses));

        assert_eq!(ItemCategory::parse("shoes"), Some(ItemCategory::Shoes));
        assert_eq!(ItemCategory::parse("Shoes"), Some(ItemCategory::Shoes));

        assert_eq!(ItemCategory::parse("top"), Some(ItemCategory::Top));
        assert_eq!(ItemCategory::parse("Top"), Some(ItemCategory::Top));
        assert_eq!(ItemCategory::parse("tops"), Some(ItemCategory::Top));

        assert_eq!(
            ItemCategory::parse("bottom"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(
            ItemCategory::parse("Bottom"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(
            ItemCategory::parse("bottoms"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(
            ItemCategory::parse("Bottom/Skirt"),
            Some(ItemCategory::BottomSkirt)
        );

        assert_eq!(ItemCategory::parse("socks"), Some(ItemCategory::Socks));
        assert_eq!(ItemCategory::parse("Socks"), Some(ItemCategory::Socks));

        assert_eq!(ItemCategory::parse("gloves"), Some(ItemCategory::Gloves));
        assert_eq!(ItemCategory::parse("Gloves"), Some(ItemCategory::Gloves));

        assert_eq!(ItemCategory::parse("scarf"), Some(ItemCategory::Scarf));
        assert_eq!(ItemCategory::parse("Scarf"), Some(ItemCategory::Scarf));
        assert_eq!(ItemCategory::parse("scarves"), Some(ItemCategory::Scarf));

        assert_eq!(ItemCategory::parse("bag"), Some(ItemCategory::Bag));
        assert_eq!(ItemCategory::parse("Bag"), Some(ItemCategory::Bag));
        assert_eq!(ItemCategory::parse("bags"), Some(ItemCategory::Bag));

        assert_eq!(
            ItemCategory::parse("accessory"),
            Some(ItemCategory::Accessory)
        );
        assert_eq!(
            ItemCategory::parse("Accessory"),
            Some(ItemCategory::Accessory)
        );
        assert_eq!(
            ItemCategory::parse("accessories"),
            Some(ItemCategory::Accessory)
        );

        assert_eq!(ItemCategory::parse("other"), Some(ItemCategory::Other));
        assert_eq!(ItemCategory::parse("Other"), Some(ItemCategory::Other));

        // Test Chinese backward compatibility
        assert_eq!(ItemCategory::parse("帽子"), Some(ItemCategory::Hat));
        assert_eq!(ItemCategory::parse("眼镜"), Some(ItemCategory::Glasses));
        assert_eq!(ItemCategory::parse("鞋子"), Some(ItemCategory::Shoes));
        assert_eq!(ItemCategory::parse("上衣"), Some(ItemCategory::Top));
        assert_eq!(
            ItemCategory::parse("裤子/裙子"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(
            ItemCategory::parse("裤子_裙子"),
            Some(ItemCategory::BottomSkirt)
        );
        assert_eq!(ItemCategory::parse("袜子"), Some(ItemCategory::Socks));
        assert_eq!(ItemCategory::parse("手套"), Some(ItemCategory::Gloves));
        assert_eq!(ItemCategory::parse("围巾"), Some(ItemCategory::Scarf));
        assert_eq!(ItemCategory::parse("包包"), Some(ItemCategory::Bag));
        assert_eq!(ItemCategory::parse("饰品"), Some(ItemCategory::Accessory));
        assert_eq!(ItemCategory::parse("其他"), Some(ItemCategory::Other));

        // Test invalid inputs
        assert_eq!(ItemCategory::parse("invalid"), None);
        assert_eq!(ItemCategory::parse(""), None);
        assert_eq!(ItemCategory::parse("hat123"), None);
        assert_eq!(ItemCategory::parse("INVALID"), None);
    }

    #[test]
    fn test_item_category_display() {
        let category = ItemCategory::Hat;
        assert_eq!(format!("{}", category), "Hat");

        let category = ItemCategory::BottomSkirt;
        assert_eq!(format!("{}", category), "Bottom/Skirt");
    }

    #[test]
    fn test_serialization_all_categories() {
        // Test serialization for all categories
        let test_cases = [
            (ItemCategory::Hat, "hat"),
            (ItemCategory::Glasses, "glasses"),
            (ItemCategory::Shoes, "shoes"),
            (ItemCategory::Top, "top"),
            (ItemCategory::BottomSkirt, "bottom"),
            (ItemCategory::Socks, "socks"),
            (ItemCategory::Gloves, "gloves"),
            (ItemCategory::Scarf, "scarf"),
            (ItemCategory::Bag, "bag"),
            (ItemCategory::Accessory, "accessory"),
            (ItemCategory::Other, "other"),
        ];

        for (category, expected_json) in test_cases.iter() {
            let json = serde_json::to_string(category).unwrap();
            assert_eq!(json, format!(r#""{}""#, expected_json));

            let deserialized: ItemCategory = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, *category);
        }
    }

    #[test]
    fn test_image_metadata_creation() {
        let metadata = ImageMetadata {
            id: Uuid::new_v4(),
            filename: Some("test.jpg".to_string()),
            hash: "abc123".to_string(),
            size: 1024,
            dimensions: (800, 600),
            mime_type: "image/jpeg".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            category: Some(ItemCategory::Hat),
            prompt: Some("A red hat".to_string()),
            source_images: vec!["source1".to_string(), "source2".to_string()],
        };

        assert!(metadata.filename.is_some());
        assert_eq!(metadata.hash, "abc123");
        assert_eq!(metadata.size, 1024);
        assert_eq!(metadata.dimensions, (800, 600));
        assert_eq!(metadata.mime_type, "image/jpeg");
        assert_eq!(metadata.category, Some(ItemCategory::Hat));
        assert_eq!(metadata.prompt, Some("A red hat".to_string()));
        assert_eq!(metadata.source_images.len(), 2);
    }

    #[test]
    fn test_image_metadata_serialization() {
        let metadata = ImageMetadata {
            id: Uuid::new_v4(),
            filename: Some("test.jpg".to_string()),
            hash: "abc123".to_string(),
            size: 1024,
            dimensions: (800, 600),
            mime_type: "image/jpeg".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            category: Some(ItemCategory::Hat),
            prompt: Some("A red hat".to_string()),
            source_images: vec!["source1".to_string()],
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: ImageMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.filename, metadata.filename);
        assert_eq!(deserialized.hash, metadata.hash);
        assert_eq!(deserialized.size, metadata.size);
        assert_eq!(deserialized.dimensions, metadata.dimensions);
        assert_eq!(deserialized.mime_type, metadata.mime_type);
        assert_eq!(deserialized.category, metadata.category);
        assert_eq!(deserialized.prompt, metadata.prompt);
        assert_eq!(deserialized.source_images, metadata.source_images);
    }

    #[test]
    fn test_api_response_serialization() {
        // Test CategoriesResponse
        let categories_response = CategoriesResponse {
            categories: vec![
                CategoryInfo {
                    id: ItemCategory::Hat,
                    name: "Hat".to_string(),
                    count: 5,
                },
                CategoryInfo {
                    id: ItemCategory::Shoes,
                    name: "Shoes".to_string(),
                    count: 3,
                },
            ],
        };

        let json = serde_json::to_string(&categories_response).unwrap();
        let deserialized: CategoriesResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.categories.len(), 2);
        assert_eq!(deserialized.categories[0].id, ItemCategory::Hat);
        assert_eq!(deserialized.categories[0].count, 5);
    }

    #[test]
    fn test_request_deserialization() {
        // Test GenerateItemRequest
        let json = json!({
            "prompt": "vintage style hat",
            "style": "retro",
            "color": "brown"
        });

        let request: GenerateItemRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.prompt, Some("vintage style hat".to_string()));
        assert_eq!(request.style, Some("retro".to_string()));
        assert_eq!(request.color, Some("brown".to_string()));

        // Test with missing optional fields
        let json = json!({});
        let request: GenerateItemRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.prompt, None);
        assert_eq!(request.style, None);
        assert_eq!(request.color, None);
    }

    #[test]
    fn test_remix_request_deserialization() {
        let json = json!({
            "base_image": "abc123",
            "items": [
                ["hat", "def456"],
                ["shoes", "ghi789"]
            ],
            "style": "casual outdoor",
            "quality": 8
        });

        let request: RemixRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.base_image, "abc123");
        assert_eq!(request.items.len(), 2);
        assert_eq!(request.items[0].0, ItemCategory::Hat);
        assert_eq!(request.items[0].1, "def456");
        assert_eq!(request.items[1].0, ItemCategory::Shoes);
        assert_eq!(request.items[1].1, "ghi789");
        assert_eq!(request.style, Some("casual outdoor".to_string()));
        assert_eq!(request.quality, Some(8));
    }

    #[test]
    fn test_upload_response_serialization() {
        let response = UploadResponse {
            id: Uuid::new_v4(),
            hash: "abc123".to_string(),
            filename: Some("photo.jpg".to_string()),
            dimensions: (1920, 1080),
            size: 2048000,
            url: "http://localhost:3000/api/images/input/abc123.jpg".to_string(),
            message: "Photo uploaded successfully".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        // Test that JSON contains expected values
        assert!(json.contains("abc123"));
        assert!(json.contains("1920"));
        assert!(json.contains("1080"));
        assert!(json.contains("2048000"));
    }

    #[test]
    fn test_health_response() {
        let response = HealthResponse {
            status: "ok".to_string(),
            version: "0.1.0".to_string(),
            gemini_api_available: true,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ok"));
        assert!(json.contains("0.1.0"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_error_response_format() {
        let error_response = ErrorResponse {
            error: ErrorDetails {
                message: "Invalid input".to_string(),
                code: 400,
            },
        };

        let json = serde_json::to_string(&error_response).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["error"]["message"], "Invalid input");
        assert_eq!(parsed["error"]["code"], 400);
    }

    #[test]
    fn test_gemini_request_serialization() {
        let request = GeminiGenerateRequest {
            contents: vec![GeminiContent {
                parts: vec![
                    GeminiPart::Text {
                        text: "Generate a hat image".to_string(),
                    },
                    GeminiPart::InlineData {
                        inline_data: GeminiInlineData {
                            mime_type: "image/jpeg".to_string(),
                            data: "base64data".to_string(),
                        },
                    },
                ],
            }],
            generation_config: GeminiGenerationConfig {
                temperature: 0.4,
                top_k: 32,
                top_p: 1.0,
                max_output_tokens: 1024,
                response_modalities: Some(vec!["IMAGE".to_string()]),
            },
        };

        let json = serde_json::to_string(&request).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed["contents"].is_array());
        assert_eq!(parsed["generationConfig"]["temperature"], 0.4);
        assert_eq!(parsed["generationConfig"]["topK"], 32);
        assert_eq!(parsed["generationConfig"]["topP"], 1.0);
        assert_eq!(parsed["generationConfig"]["maxOutputTokens"], 1024);
    }

    #[test]
    fn test_gemini_response_deserialization() {
        let json = json!({
            "candidates": [
                {
                    "content": {
                        "parts": [
                            {
                                "text": "Generated response"
                            }
                        ]
                    },
                    "finishReason": "STOP"
                }
            ]
        });

        let response: GeminiResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.candidates.len(), 1);
        assert_eq!(
            response.candidates[0].finish_reason,
            Some("STOP".to_string())
        );
        assert_eq!(response.candidates[0].content.parts.len(), 1);

        if let GeminiPart::Text { text } = &response.candidates[0].content.parts[0] {
            assert_eq!(text, "Generated response");
        } else {
            panic!("Expected text part");
        }
    }

    #[test]
    fn test_gemini_part_serialization() {
        let text_part = GeminiPart::Text {
            text: "Hello world".to_string(),
        };
        let json = serde_json::to_string(&text_part).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["text"], "Hello world");

        let inline_data_part = GeminiPart::InlineData {
            inline_data: GeminiInlineData {
                mime_type: "image/png".to_string(),
                data: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==".to_string(),
            },
        };
        let json = serde_json::to_string(&inline_data_part).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["inlineData"]["mimeType"], "image/png");
    }

    #[test]
    fn test_category_info_completeness() {
        // Ensure CategoryInfo can be created for all categories
        for category in ItemCategory::all() {
            let info = CategoryInfo {
                id: category.clone(),
                name: category.display_name().to_string(),
                count: 0,
            };

            assert_eq!(info.id, category);
            assert_eq!(info.name, category.display_name());
        }
    }

    #[test]
    fn test_item_info_url_format() {
        let item_info = ItemInfo {
            id: Uuid::new_v4(),
            filename: "test.jpg".to_string(),
            hash: "abc123".to_string(),
            dimensions: (800, 600),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            prompt: Some("Test item".to_string()),
            url: "http://localhost:3000/api/images/items/hats/abc123.jpg".to_string(),
        };

        assert!(item_info.url.contains("/api/images/items/"));
        assert!(item_info.url.ends_with(".jpg"));
        assert!(item_info.url.contains(&item_info.hash));
    }

    #[test]
    fn test_boundary_conditions() {
        // Test empty strings
        let empty_category = ItemCategory::parse("");
        assert_eq!(empty_category, None);

        // Test very long strings
        let long_string = "a".repeat(1000);
        let long_category = ItemCategory::parse(&long_string);
        assert_eq!(long_category, None);

        // Test special characters
        let special_chars = "hat!@#$%";
        let special_category = ItemCategory::parse(special_chars);
        assert_eq!(special_category, None);
    }

    #[test]
    fn test_category_consistency() {
        // Ensure all categories have consistent naming
        for category in ItemCategory::all() {
            let display_name = category.display_name();
            let dir_name = category.dir_name();

            assert!(
                !display_name.is_empty(),
                "Display name should not be empty for {:?}",
                category
            );
            assert!(
                !dir_name.is_empty(),
                "Directory name should not be empty for {:?}",
                category
            );
            assert!(
                dir_name.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "Directory name should be lowercase ASCII for {:?}: {}",
                category,
                dir_name
            );
        }
    }
}
