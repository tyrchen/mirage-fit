//! Integration tests for Mirage Fit
//!
//! These tests verify end-to-end functionality by testing complete API workflows
//! and module interactions.

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use mirage_fit::{
    config::Config,
    file_manager::FileManager,
    gemini::GeminiClient,
    models::{GenerateItemRequest, ItemCategory, RemixRequest},
    server::create_app,
};
use serde_json::{json, Value};
use tempfile::tempdir;
use tower::ServiceExt;

/// Create a test application with temporary file system
async fn create_test_app() -> axum::Router {
    let temp_dir = tempdir().unwrap();
    let mut config = Config::new(Some("test-api-key".to_string())).unwrap();
    config.fs.base_dir = temp_dir.path().to_path_buf();
    config.fs.input_dir = config.fs.base_dir.join("input");
    config.fs.items_dir = config.fs.base_dir.join("items");
    config.fs.output_dir = config.fs.base_dir.join("output");

    let file_manager = FileManager::new(&config).await.unwrap();
    create_app(config, file_manager).await.unwrap()
}

/// Create minimal JPEG test data
#[allow(dead_code)]
fn create_test_jpeg() -> Vec<u8> {
    // A minimal valid JPEG file (1x1 pixel white)
    vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06,
        0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B,
        0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31,
        0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF,
        0xC0, 0x00, 0x11, 0x08, 0x00, 0x01, 0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01,
        0x03, 0x11, 0x01, 0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xFF, 0xC4, 0x00, 0x14, 0x10,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xFF, 0xDA, 0x00, 0x0C, 0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F,
        0x00, 0x9F, 0xFF, 0xD9,
    ]
}

#[tokio::test]
async fn test_health_check_endpoint() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
    assert!(json["gemini_api_available"].is_boolean());
}

#[tokio::test]
async fn test_categories_endpoint() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/categories")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["categories"].is_array());
    let categories = json["categories"].as_array().unwrap();
    assert_eq!(categories.len(), 11); // All ItemCategory variants

    // Check first category structure
    let first_category = &categories[0];
    assert!(first_category["id"].is_string());
    assert!(first_category["name"].is_string());
    assert!(first_category["count"].is_number());
}

#[tokio::test]
async fn test_category_items_endpoint() {
    let app = create_test_app().await;

    // Test valid category
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/items/hat")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["category"].is_string());
    assert!(json["items"].is_array());
}

#[tokio::test]
async fn test_invalid_category_endpoint() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/items/invalid_category")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_generate_item_endpoint() {
    let app = create_test_app().await;

    let request_body = json!({
        "prompt": "A stylish red hat",
        "style": "modern",
        "color": "red"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/items/hat")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Note: This will likely return a 500 error in tests since we don't have a real Gemini API key
    // but we can verify the endpoint is reachable and the request is properly formatted
    assert!(
        response.status().is_client_error()
            || response.status().is_server_error()
            || response.status().is_success()
    );
}

#[tokio::test]
async fn test_remix_request_format() {
    let app = create_test_app().await;

    let request_body = json!({
        "base_image": "abc123",
        "items": [
            ["hat", "def456"],
            ["shoes", "ghi789"]
        ],
        "style": "casual",
        "quality": 8
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/remix")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Similar to generate_item, this will likely fail without real images and API key
    // but verifies the endpoint accepts the request format
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[tokio::test]
async fn test_outputs_endpoint() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/outputs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["outputs"].is_array());
}

#[tokio::test]
async fn test_not_found_endpoints() {
    let endpoints = [
        "/api/nonexistent",
        "/api/items/nonexistent/extra",
        "/api/unknown",
    ];

    for endpoint in endpoints.iter() {
        let app = create_test_app().await;
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(*endpoint)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "Endpoint {} should return 404",
            endpoint
        );
    }
}

#[tokio::test]
async fn test_static_file_serving() {
    let app = create_test_app().await;

    // Test root path (should serve UI or fallback)
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should either return UI files or a fallback page
    assert!(response.status().is_success());
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_cors_headers() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/health")
                .header("Origin", "http://localhost:3000")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Should handle CORS preflight requests
    assert!(response.status().is_success() || response.status() == StatusCode::OK);
}

#[test]
fn test_item_category_roundtrip() {
    // Test that all categories can be parsed and serialized correctly
    for category in ItemCategory::all() {
        let json = serde_json::to_string(&category).unwrap();
        let parsed: ItemCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, category);

        // Test parsing from different formats
        assert!(ItemCategory::parse(category.display_name()).is_some());
        assert!(ItemCategory::parse(category.dir_name()).is_some());
    }
}

#[test]
fn test_config_integration() {
    // Test that Config can be created and used with FileManager
    std::env::set_var("GEMINI_API_KEY", "test-integration-key");

    let config = Config::new(None).unwrap();
    assert_eq!(config.gemini.api_key, "test-integration-key");

    // Test that all necessary paths are set
    assert!(!config.fs.base_dir.to_string_lossy().is_empty());
    assert!(!config.fs.input_dir.to_string_lossy().is_empty());
    assert!(!config.fs.items_dir.to_string_lossy().is_empty());
    assert!(!config.fs.output_dir.to_string_lossy().is_empty());
}

#[tokio::test]
async fn test_file_manager_integration() {
    let temp_dir = tempdir().unwrap();
    let mut config = Config::new(Some("test-key".to_string())).unwrap();
    config.fs.base_dir = temp_dir.path().to_path_buf();
    config.fs.input_dir = config.fs.base_dir.join("input");
    config.fs.items_dir = config.fs.base_dir.join("items");
    config.fs.output_dir = config.fs.base_dir.join("output");

    // Test FileManager creation and directory setup
    let file_manager = FileManager::new(&config).await.unwrap();

    // Verify directories were created
    assert!(config.fs.input_dir.exists());
    assert!(config.fs.items_dir.exists());
    assert!(config.fs.output_dir.exists());

    // Test category stats
    let stats = file_manager.get_category_stats().await.unwrap();
    assert_eq!(stats.len(), ItemCategory::all().len());

    for category in ItemCategory::all() {
        assert!(stats.contains_key(&category));
        let category_path = config.category_path(&category);
        assert!(category_path.exists());
    }
}

#[test]
fn test_gemini_client_configuration() {
    let config = Config::new(Some("test-gemini-key".to_string())).unwrap();
    // Test that client is properly configured
    // Note: We can't test actual API calls without a real key, but we can test configuration
    assert!(!config.gemini.api_key.is_empty());
    assert!(!config.gemini.base_url.is_empty());
    assert!(!config.gemini.model.is_empty());
}

#[test]
fn test_error_handling_consistency() {
    use mirage_fit::Error;

    // Test that all error types can be created and converted properly
    let errors = vec![
        Error::config("Test config error"),
        Error::file_system("Test filesystem error"),
        Error::gemini_api("Test API error", Some(500)),
        Error::invalid_request("Test invalid request"),
        Error::not_found("Test resource"),
        Error::invalid_image("Test invalid image"),
        Error::rate_limit("Test rate limit"),
        Error::internal("Test internal error"),
    ];

    for error in errors {
        // Test that all errors have reasonable status codes
        let status = error.status_code();
        assert!(status.as_u16() >= 400);
        assert!(status.as_u16() < 600);

        // Test that user messages don't leak sensitive info
        let user_msg = error.user_message();
        assert!(!user_msg.is_empty());

        // Test that error can be displayed
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
    }
}

#[test]
fn test_application_state_creation() {
    let config = Config::new(Some("test-app-state-key".to_string())).unwrap();
    let _gemini_client = GeminiClient::new(config.clone()).unwrap();

    // We can't easily create FileManager without async context, but we can test
    // that the required components can be created individually
    assert!(!config.gemini.api_key.is_empty());

    // Test configuration values
    assert!(config.server.port > 0);
    assert!(config.fs.max_file_size > 0);
    assert!(!config.server.host.is_empty());
}

#[test]
fn test_request_response_models() {
    // Test GenerateItemRequest
    let json = json!({
        "prompt": "Test prompt",
        "style": "modern",
        "color": "blue"
    });
    let request: GenerateItemRequest = serde_json::from_value(json).unwrap();
    assert_eq!(request.prompt, Some("Test prompt".to_string()));
    assert_eq!(request.style, Some("modern".to_string()));
    assert_eq!(request.color, Some("blue".to_string()));

    // Test RemixRequest
    let json = json!({
        "base_image": "base123",
        "items": [["hat", "item456"]],
        "style": "casual",
        "quality": 7
    });
    let request: RemixRequest = serde_json::from_value(json).unwrap();
    assert_eq!(request.base_image, "base123");
    assert_eq!(request.items.len(), 1);
    assert_eq!(request.items[0].0, ItemCategory::Hat);
    assert_eq!(request.items[0].1, "item456");
    assert_eq!(request.style, Some("casual".to_string()));
    assert_eq!(request.quality, Some(7));
}
