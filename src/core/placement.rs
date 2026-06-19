//! Shared placement state for positioned slide elements (tables, charts, images).

/// Position and optional size for a slide element, in EMU.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ElementPlacement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl ElementPlacement {
    /// Create placement at the origin with zero size.
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    /// Create placement with default chart dimensions (5" × 3.75").
    pub const fn chart_defaults() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 5_000_000,
            height: 3_750_000,
        }
    }

    /// Create placement with default image dimensions (2" square).
    pub const fn image_defaults() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1_828_800,
            height: 1_828_800,
        }
    }

    /// Set position (fluent, consuming).
    pub fn with_position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size (fluent, consuming).
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set position (mutable builder style).
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    /// Set size (mutable builder style).
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placement_defaults() {
        let chart = ElementPlacement::chart_defaults();
        assert_eq!(chart.width, 5_000_000);
        assert_eq!(chart.height, 3_750_000);

        let image = ElementPlacement::image_defaults();
        assert_eq!(image.width, 1_828_800);
    }

    #[test]
    fn test_placement_fluent() {
        let p = ElementPlacement::new()
            .with_position(100, 200)
            .with_size(300, 400);
        assert_eq!(p.x, 100);
        assert_eq!(p.y, 200);
        assert_eq!(p.width, 300);
        assert_eq!(p.height, 400);
    }
}
