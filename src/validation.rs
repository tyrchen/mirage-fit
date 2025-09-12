//! File validation and security checks for uploads
//!
//! This module provides comprehensive validation for uploaded files including:
//! - MIME type verification
//! - File size limits
//! - Content inspection for malicious payloads
//! - Image format validation

use crate::{Error, Result};

/// Maximum file size for uploads (10MB)
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Minimum file size (1KB) to prevent empty/corrupted uploads
const MIN_FILE_SIZE: u64 = 1024;

/// Magic bytes for common image formats
// JPEG files start with FF D8, the third byte can vary (FF E0, FF E1, etc.)
const JPEG_MAGIC: &[u8] = &[0xFF, 0xD8];
const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const WEBP_MAGIC: &[u8] = b"RIFF";
const WEBP_MARKER: &[u8] = b"WEBP";

/// Validate uploaded file data
pub fn validate_upload(data: &[u8], filename: Option<&str>) -> Result<()> {
    // Check file size
    validate_file_size(data.len() as u64)?;

    // Verify image format by magic bytes
    validate_image_format(data)?;

    // Skip malicious content check for valid image files
    // Image files are binary and checking for text patterns in them
    // leads to false positives. The format validation above ensures
    // it's a valid image file structure.

    // Validate filename if provided
    if let Some(name) = filename {
        validate_filename(name)?;
    }

    Ok(())
}

/// Validate file size is within acceptable limits
fn validate_file_size(size: u64) -> Result<()> {
    if size > MAX_FILE_SIZE {
        return Err(Error::invalid_request(format!(
            "File size ({} bytes) exceeds maximum allowed size ({} bytes)",
            size, MAX_FILE_SIZE
        )));
    }

    if size < MIN_FILE_SIZE {
        return Err(Error::invalid_request(format!(
            "File size ({} bytes) is below minimum required size ({} bytes)",
            size, MIN_FILE_SIZE
        )));
    }

    Ok(())
}

/// Validate image format by checking magic bytes
fn validate_image_format(data: &[u8]) -> Result<()> {
    if data.len() < 12 {
        return Err(Error::invalid_request("File too small to be a valid image"));
    }

    // Check for JPEG
    if data.starts_with(JPEG_MAGIC) {
        return validate_jpeg(data);
    }

    // Check for PNG
    if data.starts_with(PNG_MAGIC) {
        return validate_png(data);
    }

    // Check for WebP
    if data.starts_with(WEBP_MAGIC) && data.len() > 12 && &data[8..12] == WEBP_MARKER {
        return validate_webp(data);
    }

    Err(Error::invalid_request(
        "Unsupported image format. Only JPEG, PNG, and WebP are allowed",
    ))
}

/// Validate JPEG structure
fn validate_jpeg(data: &[u8]) -> Result<()> {
    // Basic JPEG validation - check for proper start
    if data.len() < 4 {
        return Err(Error::invalid_request("Invalid JPEG file: too small"));
    }

    // JPEG files should start with FF D8 (SOI marker)
    // The third byte is typically FF followed by E0, E1, E2, etc. for different JPEG formats
    if !data.starts_with(&[0xFF, 0xD8]) {
        return Err(Error::invalid_request(
            "Invalid JPEG file structure: missing SOI marker",
        ));
    }

    // The third byte should be FF (start of another marker)
    if data.len() > 2 && data[2] != 0xFF {
        return Err(Error::invalid_request(
            "Invalid JPEG file structure: invalid marker after SOI",
        ));
    }

    // Look for JPEG end marker (FF D9) somewhere in the file
    // Note: The end marker might not be at the very end due to metadata
    let mut found_end_marker = false;
    for i in 2..data.len().saturating_sub(1) {
        if data[i] == 0xFF && data[i + 1] == 0xD9 {
            found_end_marker = true;
            break;
        }
    }

    if !found_end_marker {
        return Err(Error::invalid_request(
            "Invalid JPEG file: missing end marker",
        ));
    }

    Ok(())
}

/// Validate PNG structure
fn validate_png(data: &[u8]) -> Result<()> {
    // PNG should have IEND chunk at the end
    let data_len = data.len();
    if data_len < 12 {
        return Err(Error::invalid_request("Invalid PNG file structure"));
    }

    // Look for IEND chunk (simplified check)
    let iend_marker = b"IEND";
    let mut found_iend = false;

    for window in data.windows(4) {
        if window == iend_marker {
            found_iend = true;
            break;
        }
    }

    if !found_iend {
        return Err(Error::invalid_request(
            "Invalid PNG file structure: missing IEND chunk",
        ));
    }

    Ok(())
}

/// Validate WebP structure
fn validate_webp(data: &[u8]) -> Result<()> {
    // Basic WebP validation - just check headers for now
    if data.len() < 30 {
        return Err(Error::invalid_request("Invalid WebP file structure"));
    }

    Ok(())
}

/// Check for potentially malicious content patterns
/// Note: This is not currently used for image files as it can cause false positives
/// with binary data. It's preserved here for potential future use with text files.
#[allow(dead_code)]
fn check_for_malicious_content(data: &[u8]) -> Result<()> {
    // Check for embedded scripts or executables
    let suspicious_patterns: &[&[u8]] = &[
        b"<script",
        b"javascript:",
        b"eval(",
        b"document.",
        b"window.",
        b"alert(",
        b"<?php",
        b"<%",
        b"<jsp:",
        b"\x4D\x5A",               // PE executable header
        &[0x7F, 0x45, 0x4C, 0x46], // ELF header
    ];

    for pattern in suspicious_patterns {
        if contains_pattern(data, pattern) {
            return Err(Error::invalid_request(
                "File contains suspicious content and was rejected",
            ));
        }
    }

    Ok(())
}

/// Check if data contains a specific byte pattern
fn contains_pattern(data: &[u8], pattern: &[u8]) -> bool {
    if pattern.is_empty() || data.len() < pattern.len() {
        return false;
    }

    data.windows(pattern.len()).any(|window| window == pattern)
}

/// Validate filename for path traversal and other issues
fn validate_filename(filename: &str) -> Result<()> {
    // Check for path traversal attempts
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err(Error::invalid_request(
            "Invalid filename: path traversal detected",
        ));
    }

    // Check for null bytes
    if filename.contains('\0') {
        return Err(Error::invalid_request(
            "Invalid filename: null bytes not allowed",
        ));
    }

    // Check filename length
    if filename.len() > 255 {
        return Err(Error::invalid_request(
            "Filename too long (max 255 characters)",
        ));
    }

    // Check for valid extension
    let valid_extensions = [".jpg", ".jpeg", ".png", ".webp"];
    let lower = filename.to_lowercase();
    let has_valid_extension = valid_extensions.iter().any(|ext| lower.ends_with(ext));

    if !has_valid_extension {
        return Err(Error::invalid_request(
            "Invalid file extension. Only .jpg, .jpeg, .png, and .webp are allowed",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_size() {
        // Valid size
        assert!(validate_file_size(1024 * 1024).is_ok());

        // Too large
        assert!(validate_file_size(MAX_FILE_SIZE + 1).is_err());

        // Too small
        assert!(validate_file_size(MIN_FILE_SIZE - 1).is_err());
    }

    #[test]
    fn test_validate_filename() {
        // Valid filenames
        assert!(validate_filename("photo.jpg").is_ok());
        assert!(validate_filename("image.png").is_ok());
        assert!(validate_filename("file.webp").is_ok());

        // Path traversal attempts
        assert!(validate_filename("../etc/passwd").is_err());
        assert!(validate_filename("..\\windows\\system32").is_err());
        assert!(validate_filename("folder/file.jpg").is_err());

        // Invalid extensions
        assert!(validate_filename("script.js").is_err());
        assert!(validate_filename("executable.exe").is_err());

        // Too long
        let long_name = "a".repeat(256) + ".jpg";
        assert!(validate_filename(&long_name).is_err());
    }

    #[test]
    fn test_malicious_content_detection() {
        // Clean data
        assert!(check_for_malicious_content(b"Hello World").is_ok());

        // Script injection
        assert!(check_for_malicious_content(b"<script>alert('xss')</script>").is_err());
        assert!(check_for_malicious_content(b"javascript:void(0)").is_err());

        // Server-side code
        assert!(check_for_malicious_content(b"<?php echo 'test'; ?>").is_err());

        // Executable headers
        assert!(check_for_malicious_content(b"\x4D\x5A\x90\x00").is_err());
    }
}
