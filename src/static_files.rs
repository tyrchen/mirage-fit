//! Static file serving with embedded UI assets
//!
//! This module embeds the built UI files into the binary and serves them
//! efficiently using rust-embed.

use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

/// Embedded UI assets from the build directory
#[derive(RustEmbed)]
#[folder = "ui/dist"]
#[include = "*"]
pub struct UiAssets;

/// Serve embedded static files
pub async fn serve_static(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    // Default to index.html for root path
    let path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        path
    };

    // Try to serve the file
    match UiAssets::get(path) {
        Some(content) => {
            let body = Body::from(content.data);
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::CACHE_CONTROL, "public, max-age=3600")
                .body(body)
                .unwrap()
        }
        None => {
            // If file not found and it's not an API call, serve index.html for client-side routing
            if !path.starts_with("api/") {
                serve_index().await
            } else {
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
        }
    }
}

/// Serve the index.html file for client-side routing
async fn serve_index() -> Response {
    match UiAssets::get("index.html") {
        Some(content) => {
            let body = Body::from(content.data);

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .header(header::CACHE_CONTROL, "no-cache")
                .body(body)
                .unwrap()
        }
        None => {
            // If index.html is not found, return a fallback page
            let fallback = r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Mirage Fit</title>
                    <style>
                        body {
                            font-family: system-ui, -apple-system, sans-serif;
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100vh;
                            margin: 0;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            color: white;
                        }
                        .container {
                            text-align: center;
                            padding: 2rem;
                        }
                        h1 {
                            font-size: 3rem;
                            margin-bottom: 1rem;
                        }
                        p {
                            font-size: 1.2rem;
                            opacity: 0.9;
                        }
                        .api-link {
                            display: inline-block;
                            margin-top: 2rem;
                            padding: 0.75rem 1.5rem;
                            background: white;
                            color: #764ba2;
                            text-decoration: none;
                            border-radius: 0.5rem;
                            font-weight: 600;
                            transition: transform 0.2s;
                        }
                        .api-link:hover {
                            transform: translateY(-2px);
                        }
                    </style>
                </head>
                <body>
                    <div class="container">
                        <h1>Mirage Fit</h1>
                        <p>Fashion Remix Powered by AI</p>
                        <p style="margin-top: 2rem; font-size: 1rem;">
                            The UI is not built yet. Please run <code style="background: rgba(255,255,255,0.2); padding: 0.25rem 0.5rem; border-radius: 0.25rem;">npm run build</code> in the ui directory.
                        </p>
                        <a href="/api/health" class="api-link">Check API Status</a>
                    </div>
                </body>
                </html>
            "#;

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(fallback))
                .unwrap()
        }
    }
}

/// Check if UI assets are available
pub fn ui_assets_available() -> bool {
    UiAssets::get("index.html").is_some()
}
