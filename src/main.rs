//! Mirage Fit - Image remix application using Gemini AI
//!
//! This is the main binary entry point for the Mirage Fit application.
//! It sets up the web server, initializes the file system, and starts the API.

use anyhow::Result;
use clap::Parser;
use mirage_fit::{config::Config, file_manager::FileManager, server::create_app};
use std::net::SocketAddr;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Command line arguments for Mirage Fit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to bind the server to
    #[arg(short, long, default_value = "3000", env = "MIRAGE_FIT_PORT")]
    port: u16,

    /// Host to bind the server to
    #[arg(long, default_value = "127.0.0.1", env = "MIRAGE_FIT_HOST")]
    host: String,

    /// Gemini API key (can also be set via GEMINI_API_KEY environment variable)
    #[arg(long, env = "GEMINI_API_KEY")]
    gemini_api_key: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info", env = "RUST_LOG")]
    log_level: String,

    /// Generate default item images on startup
    #[arg(long, default_value = "false")]
    generate_defaults: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing
    init_tracing(&args.log_level)?;

    info!("Starting Mirage Fit server...");

    // Create configuration
    let config = Config::new(args.gemini_api_key)?;

    // Initialize file manager and create directory structure
    let file_manager = FileManager::new(&config).await?;

    // Generate default item images if requested
    if args.generate_defaults {
        info!("Generating default item images...");
        file_manager.generate_default_items(&config).await?;
    }

    // Create the application
    let app = create_app(config, file_manager).await?;

    // Bind to socket address
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    info!("Server listening on http://{}", addr);
    info!("API documentation available at http://{}/docs", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Initialize tracing/logging subsystem
fn init_tracing(log_level: &str) -> Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(log_level))?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(env_filter)
        .init();

    if log_level == "trace" || log_level == "debug" {
        warn!("Debug logging enabled - this may impact performance");
    }

    Ok(())
}
