//! Utility functions and types for length conversions

/// Base class for length values in English Metric Units (EMUs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(i32);

impl Length {
    const EMUS_PER_INCH: i32 = 914400;
    const EMUS_PER_CENTIPOINT: i32 = 127;
    const EMUS_PER_CM: i32 = 360000;
    const EMUS_PER_MM: i32 = 36000;
    const EMUS_PER_PT: i32 = 12700;

    /// Create a Length from EMUs
    pub fn new(emu: i32) -> Self {
        Length(emu)
    }

    /// Get length in inches
    pub fn inches(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_INCH as f64
    }

    /// Get length in centipoints (hundredths of a point)
    pub fn centipoints(&self) -> i32 {
        self.0 / Self::EMUS_PER_CENTIPOINT
    }

    /// Get length in centimeters
    pub fn cm(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_CM as f64
    }

    /// Get length in EMUs
    pub fn emu(&self) -> i32 {
        self.0
    }

    /// Get length in millimeters
    pub fn mm(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_MM as f64
    }

    /// Get length in points
    pub fn pt(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_PT as f64
    }
}

impl From<i32> for Length {
    fn from(emu: i32) -> Self {
        Length(emu)
    }
}

impl From<Length> for i32 {
    fn from(length: Length) -> Self {
        length.0
    }
}

/// Create a Length from inches
pub fn inches(value: f64) -> Length {
    Length((value * Length::EMUS_PER_INCH as f64) as i32)
}

/// Create a Length from centipoints
pub fn centipoints(value: i32) -> Length {
    Length(value * Length::EMUS_PER_CENTIPOINT)
}

/// Create a Length from centimeters
pub fn cm(value: f64) -> Length {
    Length((value * Length::EMUS_PER_CM as f64) as i32)
}

/// Create a Length from English Metric Units
pub fn emu(value: i32) -> Length {
    Length(value)
}

/// Create a Length from millimeters
pub fn mm(value: f64) -> Length {
    Length((value * Length::EMUS_PER_MM as f64) as i32)
}

/// Create a Length from points
pub fn pt(value: f64) -> Length {
    Length((value * Length::EMUS_PER_PT as f64) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversions() {
        let len = inches(1.0);
        assert_eq!(len.emu(), 914400);
        assert_eq!(len.inches(), 1.0);
    }

    #[test]
    fn test_cm_conversion() {
        let len = cm(2.54);
        assert!((len.inches() - 1.0).abs() < 0.01);
    }
}
