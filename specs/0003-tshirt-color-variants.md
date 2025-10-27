# Feature: T-Shirt Color Variants for Default Item Generation

## Overview

Enhance the default item generation to create multiple t-shirt/top items in different colors, providing users with a diverse selection of clothing options when they first start using Mirage Fit. Currently, the application generates only 2 items per category without color specification, resulting in limited variety for tops.

## Requirements

### Functional Requirements

- **REQ-1**: Generate t-shirts/tops in multiple predefined colors during default item generation
- **REQ-2**: Support common t-shirt colors: white, black, red, blue, green, yellow, gray, navy
- **REQ-3**: Generate 1-2 items per color variant (configurable)
- **REQ-4**: Maintain existing default generation behavior for other categories (hats, shoes, etc.)
- **REQ-5**: Respect rate limiting with staggered delays between generations

### Non-Functional Requirements

- **Performance**: Total generation time should remain reasonable (< 2 minutes for all items)
- **Quality**: Generated images should be photorealistic and professional
- **Maintainability**: Color list should be easily configurable
- **Rate Limiting**: Respect Gemini API rate limits with appropriate delays

## Architecture

### High-Level Design

The feature modifies the `generate_default_items` function in `file_manager.rs` to:

1. Define a list of t-shirt/top colors as a constant
2. For the `Top` category specifically, generate items with color specifications
3. For all other categories, maintain existing behavior (no color specification)

### Implementation Details

**Location**: `src/file_manager.rs`

**Changes**:
```rust
// Add color variants constant at module level
const TSHIRT_COLORS: &[&str] = &["white", "black", "red", "blue", "green", "yellow", "gray", "navy"];

// Modify generate_default_items function to:
// 1. Check if category is ItemCategory::Top
// 2. If yes, iterate through TSHIRT_COLORS and generate 1 item per color
// 3. If no, use existing logic (generate items_per_category with no color)
```

**Modified Function**: `FileManager::generate_default_items`

**Logic Flow**:
```
FOR each category in ItemCategory::all():
    IF category == ItemCategory::Top:
        FOR each color in TSHIRT_COLORS:
            Generate 1 item with specified color
            Apply staggered delay
    ELSE:
        Generate items_per_category items with no color (existing logic)
        Apply staggered delay
```

## Implementation Steps

1. Add `TSHIRT_COLORS` constant at the top of `file_manager.rs` module
2. Modify `generate_default_items` function to detect `ItemCategory::Top`
3. Implement color-specific generation loop for tops
4. Maintain existing logic for other categories
5. Adjust delay calculations to account for increased total items
6. Test with `--generate-defaults` flag

## Testing Strategy

### Manual Testing

**Test Case 1: Default Generation with Color Variants**
- Steps:
  1. Delete existing `~/.mirage-fit/items/tops/` directory
  2. Run `cargo run -- --generate-defaults`
  3. Check `~/.mirage-fit/items/tops/` directory
- Expected: See 8 t-shirt images in different colors (white, black, red, blue, green, yellow, gray, navy)

**Test Case 2: Existing Categories Unchanged**
- Steps:
  1. Delete all item directories
  2. Run `cargo run -- --generate-defaults`
  3. Check all category directories
- Expected: Tops has 8 items (colors), other categories have 2 items each (no color)

**Test Case 3: Rate Limiting**
- Steps:
  1. Run generation with verbose logging: `RUST_LOG=debug cargo run -- --generate-defaults`
  2. Monitor logs for delays
- Expected: See staggered delays, no rate limit errors from Gemini API

### Integration Testing

- Existing integration tests should continue to pass
- No new integration tests required (this is a default generation feature)

## Acceptance Criteria

- [x] T-shirts/tops are generated in 8 different colors during default generation
- [x] Each color variant is clearly specified in the generation prompt
- [x] Other item categories (hats, shoes, etc.) maintain existing 2-item generation
- [x] No Gemini API rate limit errors occur during generation
- [x] Generated t-shirt images are photorealistic and professional quality
- [x] All existing tests pass
- [x] Code formatted with `cargo fmt`
- [x] No lint warnings from `cargo clippy`

## Configuration

**Default Colors**: white, black, red, blue, green, yellow, gray, navy

**Items Per Color**: 1 (total of 8 t-shirts)

**Future Enhancement**: Could expose `TSHIRT_COLORS` as a configuration option in `Config` struct

## Benefits

1. **User Experience**: Users have immediate access to a diverse wardrobe upon first use
2. **Showcase**: Better demonstrates the app's fashion remix capabilities
3. **Practical**: Common t-shirt colors cover most user preferences
4. **Scalable**: Easy to add/remove colors by modifying the constant

## Risks & Mitigations

**Risk 1: Increased generation time**
- Mitigation: Generate only 1 item per color (8 total vs previous 2)
- Impact: ~30 seconds additional time (manageable for initial setup)

**Risk 2: API rate limiting**
- Mitigation: Maintain staggered delays (200ms per item)
- Existing delay mechanism already handles rate limiting

**Risk 3: Storage space**
- Mitigation: 8 images vs 2 images is minimal increase (~6 additional images)
- Blake3 deduplication prevents storage waste
