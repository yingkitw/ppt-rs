//! Traits for presentation operations
//!
//! This module defines common traits for presentation functionality,
//! following KISS and DRY principles.

use crate::error::Result;

/// Trait for objects with dimensions (width and height)
pub trait Dimensioned {
    /// Get width in EMU (English Metric Units)
    fn width(&self) -> Option<u32>;
    
    /// Get height in EMU
    fn height(&self) -> Option<u32>;
    
    /// Set width in EMU
    fn set_width(&mut self, width: u32) -> Result<()>;
    
    /// Set height in EMU
    fn set_height(&mut self, height: u32) -> Result<()>;
}

/// Trait for objects with properties (getter/setter pairs)
pub trait PropertyAccessor<T> {
    /// Get the property
    fn get(&self) -> &T;
    
    /// Get mutable reference to the property
    fn get_mut(&mut self) -> &mut T;
}

/// Trait for objects that can be saved
pub trait Saveable {
    /// Save the object
    fn save(&mut self) -> Result<()>;
}

/// Trait for objects that can be opened
pub trait Openable {
    /// Open from a source
    fn open() -> Result<Self>
    where
        Self: Sized;
}

/// Trait for objects with metadata
pub trait Metadata {
    /// Get title
    fn title(&self) -> Option<&str>;
    
    /// Set title
    fn set_title(&mut self, title: impl Into<String>);
    
    /// Get author
    fn author(&self) -> Option<&str>;
    
    /// Set author
    fn set_author(&mut self, author: impl Into<String>);
}

/// Trait for objects with a collection of items
pub trait Collection<T> {
    /// Get item count
    fn len(&self) -> usize;
    
    /// Check if empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Add an item
    fn add(&mut self, item: T) -> Result<usize>;
    
    /// Remove an item by index
    fn remove(&mut self, index: usize) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_is_empty() {
        struct TestCollection {
            items: Vec<i32>,
        }
        
        impl Collection<i32> for TestCollection {
            fn len(&self) -> usize {
                self.items.len()
            }
            
            fn add(&mut self, item: i32) -> Result<usize> {
                self.items.push(item);
                Ok(self.items.len() - 1)
            }
            
            fn remove(&mut self, index: usize) -> Result<bool> {
                if index < self.items.len() {
                    self.items.remove(index);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
        
        let mut col = TestCollection { items: vec![] };
        assert!(col.is_empty());
        col.add(1).unwrap();
        assert!(!col.is_empty());
    }
}
