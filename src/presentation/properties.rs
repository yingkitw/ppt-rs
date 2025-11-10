//! Presentation properties management
//!
//! This module handles all presentation properties including core, app, and custom properties.
//! Follows DRY principle by centralizing property access.

use crate::opc::{CoreProperties, AppProperties, CustomProperties};
use crate::presentation::traits::PropertyAccessor;

/// Unified presentation properties manager
pub struct PropertiesManager {
    core: CoreProperties,
    app: AppProperties,
    custom: CustomProperties,
}

impl PropertiesManager {
    /// Create new properties manager
    pub fn new() -> Self {
        Self {
            core: CoreProperties::new(),
            app: AppProperties::new(),
            custom: CustomProperties::new(),
        }
    }

    /// Get core properties
    pub fn core(&self) -> &CoreProperties {
        &self.core
    }

    /// Get mutable core properties
    pub fn core_mut(&mut self) -> &mut CoreProperties {
        &mut self.core
    }

    /// Get app properties
    pub fn app(&self) -> &AppProperties {
        &self.app
    }

    /// Get mutable app properties
    pub fn app_mut(&mut self) -> &mut AppProperties {
        &mut self.app
    }

    /// Get custom properties
    pub fn custom(&self) -> &CustomProperties {
        &self.custom
    }

    /// Get mutable custom properties
    pub fn custom_mut(&mut self) -> &mut CustomProperties {
        &mut self.custom
    }
}

impl Default for PropertiesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyAccessor<CoreProperties> for PropertiesManager {
    fn get(&self) -> &CoreProperties {
        &self.core
    }

    fn get_mut(&mut self) -> &mut CoreProperties {
        &mut self.core
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_manager_new() {
        let manager = PropertiesManager::new();
        assert_eq!(manager.core().title, None);
    }

    #[test]
    fn test_properties_manager_core_access() {
        let mut manager = PropertiesManager::new();
        manager.core_mut().title = Some("Test".to_string());
        assert_eq!(manager.core().title, Some("Test".to_string()));
    }

    #[test]
    fn test_properties_manager_default() {
        let manager = PropertiesManager::default();
        assert_eq!(manager.core().title, None);
    }
}
