//! Layout constants for PPTX generation
//!
//! All positioning and sizing values are in EMU (English Metric Units).
//! EMU is the standard unit for Office Open XML format.
//!
//! Conversions:
//! - 1 inch = 914400 EMU
//! - 1 point (pt) = 12700 EMU
//! - 1 centimeter = 360000 EMU
//! - 1 millimeter = 36000 EMU

/// Standard slide dimensions in EMU
/// 10 inches × 7.5 inches (standard PowerPoint slide size)
pub const SLIDE_WIDTH: u32 = 9144000;  // 10 inches
pub const SLIDE_HEIGHT: u32 = 6858000; // 7.5 inches

// ============================================================================
// Title Shape Positioning
// ============================================================================

/// Title shape X position (left margin)
/// 0.5 inches from left edge
pub const TITLE_X: u32 = 457200;  // 0.5 inches

/// Title shape Y position (top margin)
/// ~0.3 inches from top edge
pub const TITLE_Y: u32 = 274638;  // ~0.3 inches

/// Title shape width
/// 9 inches (slide width minus 1 inch margins)
pub const TITLE_WIDTH: u32 = 8230200;  // 9 inches

/// Title shape height (standard)
/// 1.25 inches
pub const TITLE_HEIGHT: u32 = 1143000;  // 1.25 inches

/// Title shape height (big content layout)
/// 1 inch
pub const TITLE_HEIGHT_BIG: u32 = 914400;  // 1 inch

/// Title font size in EMU
/// 44pt (4400 EMU)
pub const TITLE_FONT_SIZE: u32 = 4400;  // 44pt

/// Centered title Y position
/// ~3 inches from top (centered vertically)
pub const CENTERED_TITLE_Y: u32 = 2743200;  // ~3 inches

/// Centered title height
/// 1.5 inches
pub const CENTERED_TITLE_HEIGHT: u32 = 1371600;  // 1.5 inches

// ============================================================================
// Content Shape Positioning
// ============================================================================

/// Content shape X position (left margin)
/// 0.5 inches from left edge (same as title)
pub const CONTENT_X: u32 = 457200;  // 0.5 inches

/// Content shape Y position (start position)
/// ~1.67 inches from top (below title)
pub const CONTENT_Y_START: u32 = 1600200;  // ~1.67 inches

/// Content shape Y position (alternative start)
/// ~1.3 inches from top (for big content layout)
pub const CONTENT_Y_START_BIG: u32 = 1189200;  // ~1.3 inches

/// Content shape width
/// 9 inches (same as title)
pub const CONTENT_WIDTH: u32 = 8230200;  // 9 inches

/// Content shape height (per item)
/// 1 inch spacing between items
pub const CONTENT_HEIGHT: u32 = 4572000;  // 5 inches (for content area)

/// Content shape height (big content layout)
/// ~6.2 inches (fills most of slide)
pub const CONTENT_HEIGHT_BIG: u32 = 5668800;  // ~6.2 inches

/// Vertical increment between content items
/// 1 inch spacing
pub const CONTENT_Y_INCREMENT: u32 = 914400;  // 1 inch

/// Content font size in EMU
/// 28pt (2800 EMU)
pub const CONTENT_FONT_SIZE: u32 = 2800;  // 28pt

// ============================================================================
// Two-Column Layout Positioning
// ============================================================================

/// Two-column layout: Title Y position
/// Same as standard title
pub const TWO_COL_TITLE_Y: u32 = 274638;  // ~0.3 inches

/// Two-column layout: Title height
/// 1 inch
pub const TWO_COL_TITLE_HEIGHT: u32 = 914400;  // 1 inch

/// Two-column layout: Content Y position
/// ~1.3 inches from top
pub const TWO_COL_CONTENT_Y: u32 = 1189200;  // ~1.3 inches

// ============================================================================
// Notes Slide Positioning
// ============================================================================

/// Notes slide content X position
/// 0.75 inches from left
pub const NOTES_X: u32 = 685800;  // 0.75 inches

/// Notes slide content Y position
/// 1.25 inches from top
pub const NOTES_Y: u32 = 1143000;  // 1.25 inches

// ============================================================================
// Text Properties
// ============================================================================

/// Default language code
pub const DEFAULT_LANG: &str = "en-US";

/// Default major font (for headings)
pub const DEFAULT_FONT_MAJOR: &str = "Calibri";

/// Default minor font (for body text)
pub const DEFAULT_FONT_MINOR: &str = "Calibri";

// ============================================================================
// Relationship IDs
// ============================================================================

/// Slide ID base (must be > 255 to avoid conflicts with reserved IDs)
pub const SLIDE_ID_BASE: u32 = 255;

/// Relationship ID base for slides (rId1 is reserved for slide master)
pub const SLIDE_RID_BASE: u32 = 2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emu_conversions() {
        // 1 inch = 914400 EMU
        assert_eq!(TITLE_X, 457200);  // 0.5 inches
        assert_eq!(TITLE_WIDTH, 8230200);  // 9 inches
        
        // 1 point = 12700 EMU
        // 44pt = 44 * 12700 = 558800 EMU (but we use 4400 which is in 100ths)
        // Actually: 44pt = 44 * 100 = 4400 (in the format used)
        assert_eq!(TITLE_FONT_SIZE, 4400);
    }

    #[test]
    fn test_slide_dimensions() {
        // Standard PowerPoint slide: 10" × 7.5"
        assert_eq!(SLIDE_WIDTH, 9144000);   // 10 inches
        assert_eq!(SLIDE_HEIGHT, 6858000);  // 7.5 inches
    }

    #[test]
    fn test_positioning_consistency() {
        // Title and content should have same X position (left margin)
        assert_eq!(TITLE_X, CONTENT_X);
        
        // Content should start below title
        const { assert!(CONTENT_Y_START > TITLE_Y + TITLE_HEIGHT) };
    }
}

