//! Server setup and routing for Mirage Fit
//!
//! This module creates the Axum application with all routes, middleware,
//! and static file serving configuration.

use crate::{
    config::Config,
    file_manager::FileManager,
    gemini::GeminiClient,
    handlers::{
        generate_category_item, generate_remix, get_categories, get_category_items, get_outputs,
        health_check, not_found, serve_image, upload_photo, AppState,
    },
    Result,
};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    response::Html,
    routing::{get, post},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level};

/// Create the main application with all routes and middleware
pub async fn create_app(config: Config, file_manager: FileManager) -> Result<Router> {
    info!("Creating Mirage Fit application");

    // Create Gemini client
    let gemini_client = GeminiClient::new(config.clone())?;

    // Create shared application state
    let app_state = Arc::new(AppState {
        config: config.clone(),
        file_manager,
        gemini_client,
    });

    // Create API routes
    let api_routes = Router::new()
        // Health and status
        .route("/health", get(health_check))
        // Categories and items
        .route("/categories", get(get_categories))
        .route("/items/{category}", get(get_category_items))
        .route("/items/{category}", post(generate_category_item))
        // Upload and remix
        .route("/upload", post(upload_photo))
        .route("/remix", post(generate_remix))
        .route("/outputs", get(get_outputs))
        // Image serving
        .route("/images/{type}/{*path}", get(serve_image))
        // Catch-all for API routes
        .fallback(not_found)
        .with_state(app_state.clone());

    // Create main application router
    let app = Router::new()
        // Mount API routes
        .nest("/api", api_routes)
        // Serve documentation/info page at root
        .route("/", get(serve_root))
        .route("/docs", get(serve_docs))
        // Global fallback
        .fallback(not_found)
        // Add middleware layers
        .layer(DefaultBodyLimit::max(
            config.server.max_request_size as usize,
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<axum::body::Body>| {
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(create_cors_layer(&config));

    info!("Application created successfully");
    Ok(app)
}

/// Create CORS layer based on configuration
fn create_cors_layer(config: &Config) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::ACCEPT,
            header::ACCEPT_LANGUAGE,
            header::AUTHORIZATION,
            header::CONTENT_LANGUAGE,
            header::CONTENT_TYPE,
        ])
        .max_age(Duration::from_secs(3600));

    // Configure allowed origins
    if config.server.cors_origins.contains(&"*".to_string()) {
        cors = cors.allow_origin(tower_http::cors::Any);
    } else {
        for origin in &config.server.cors_origins {
            if let Ok(header_value) = origin.parse::<HeaderValue>() {
                cors = cors.allow_origin(header_value);
            }
        }
    }

    cors
}

/// Serve root documentation page
async fn serve_root() -> Html<&'static str> {
    Html(include_str!("../docs/index.html"))
}

/// Serve API documentation
async fn serve_docs() -> Html<&'static str> {
    Html(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mirage Fit API Documentation</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 { color: #2c3e50; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        h2 { color: #34495e; margin-top: 30px; }
        h3 { color: #7f8c8d; }
        .endpoint {
            background: #ecf0f1;
            padding: 15px;
            margin: 10px 0;
            border-left: 4px solid #3498db;
            border-radius: 4px;
        }
        .method {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 3px;
            font-weight: bold;
            color: white;
            margin-right: 10px;
        }
        .get { background-color: #27ae60; }
        .post { background-color: #e74c3c; }
        .put { background-color: #f39c12; }
        .delete { background-color: #8e44ad; }
        code {
            background: #2c3e50;
            color: #ecf0f1;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'Monaco', 'Consolas', monospace;
        }
        pre {
            background: #2c3e50;
            color: #ecf0f1;
            padding: 15px;
            border-radius: 5px;
            overflow-x: auto;
        }
        .categories { display: flex; flex-wrap: wrap; gap: 10px; margin: 10px 0; }
        .category {
            background: #3498db;
            color: white;
            padding: 5px 10px;
            border-radius: 15px;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎨 Mirage Fit API Documentation</h1>

        <p>Welcome to the Mirage Fit API - an AI-powered image remix application using Gemini AI for fashion and style transformation.</p>

        <h2>📋 Base Information</h2>
        <ul>
            <li><strong>Base URL:</strong> <code>http://localhost:3000</code></li>
            <li><strong>API Version:</strong> v1</li>
            <li><strong>Content Type:</strong> <code>application/json</code> (except file uploads)</li>
        </ul>

        <h2>🏥 Health & Status</h2>
        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/health</code>
            <p>Check API health and Gemini service availability.</p>
        </div>

        <h2>🗂️ Categories & Items</h2>

        <h3>Available Categories</h3>
        <div class="categories">
            <span class="category">帽子 (Hat)</span>
            <span class="category">眼镜 (Glasses)</span>
            <span class="category">鞋子 (Shoes)</span>
            <span class="category">上衣 (Top)</span>
            <span class="category">裤子/裙子 (Bottom/Skirt)</span>
            <span class="category">袜子 (Socks)</span>
            <span class="category">手套 (Gloves)</span>
            <span class="category">围巾 (Scarf)</span>
            <span class="category">包包 (Bag)</span>
            <span class="category">饰品 (Accessory)</span>
            <span class="category">其他 (Other)</span>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/categories</code>
            <p>Get all available item categories with counts.</p>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/items/:category</code>
            <p>Get all items for a specific category. Replace <code>:category</code> with one of the categories above.</p>
        </div>

        <div class="endpoint">
            <span class="method post">POST</span>
            <code>/api/items/:category</code>
            <p>Generate a new item for the specified category.</p>
            <pre>{
  "prompt": "vintage style hat",
  "style": "retro",
  "color": "brown"
}</pre>
        </div>

        <h2>📤 Upload & Processing</h2>

        <div class="endpoint">
            <span class="method post">POST</span>
            <code>/api/upload</code>
            <p>Upload a user photo. Send as multipart/form-data with field name 'image' or 'file'.</p>
        </div>

        <div class="endpoint">
            <span class="method post">POST</span>
            <code>/api/remix</code>
            <p>Generate a remix image by combining a user photo with item images.</p>
            <pre>{
  "base_image": "blake3_hash_of_user_photo",
  "items": {
    "帽子": "blake3_hash_of_hat_item",
    "鞋子": "blake3_hash_of_shoes_item"
  },
  "style_prompt": "casual outdoor style",
  "quality": 8
}</pre>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/outputs</code>
            <p>Get all generated remix images.</p>
        </div>

        <h2>🖼️ Image Access</h2>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/images/input/:filename</code>
            <p>Access uploaded user photos.</p>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/images/items/:category/:filename</code>
            <p>Access generated item images by category.</p>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/api/images/output/:filename</code>
            <p>Access generated remix output images.</p>
        </div>

        <h2>📁 File Organization</h2>
        <p>The application organizes files in the following structure:</p>
        <pre>~/.mirage-fit/
├── input/           # User uploaded photos
├── items/           # Generated item images by category
│   ├── 帽子/
│   ├── 眼镜/
│   ├── 鞋子/
│   ├── 上衣/
│   ├── 裤子_裙子/
│   ├── 袜子/
│   ├── 手套/
│   ├── 围巾/
│   ├── 包包/
│   ├── 饰品/
│   └── 其他/
└── output/          # Generated remix images (Blake3 hash names)</pre>

        <h2>🔧 Configuration</h2>
        <p>Configure the application using environment variables:</p>
        <ul>
            <li><code>GEMINI_API_KEY</code> - Your Google Gemini API key (required)</li>
            <li><code>MIRAGE_FIT_PORT</code> - Server port (default: 3000)</li>
            <li><code>MIRAGE_FIT_HOST</code> - Server host (default: 127.0.0.1)</li>
            <li><code>RUST_LOG</code> - Log level (default: info)</li>
        </ul>

        <h2>🚀 Getting Started</h2>
        <ol>
            <li>Set your Gemini API key: <code>export GEMINI_API_KEY="your-key-here"</code></li>
            <li>Start the server: <code>./mirage-fit</code></li>
            <li>Generate default items: <code>./mirage-fit --generate-defaults</code></li>
            <li>Upload a photo via <code>/api/upload</code></li>
            <li>Create a remix via <code>/api/remix</code></li>
        </ol>

        <p style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #bdc3c7; color: #7f8c8d; text-align: center;">
            Mirage Fit v0.1.0 - AI-Powered Fashion Remix Application
        </p>
    </div>
</body>
</html>
    "#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    async fn create_test_app() -> Router {
        let config = Config::default();
        let file_manager = FileManager::new(&config).await.unwrap();
        create_app(config, file_manager).await.unwrap()
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_docs_endpoint() {
        let app = create_test_app().await;

        let response = app
            .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_categories_endpoint() {
        let app = create_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/categories")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = create_test_app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
