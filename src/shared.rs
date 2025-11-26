//! Objects shared by pptx modules

use std::rc::Rc;
use std::cell::RefCell;

/// Base class for XML element proxy classes
pub struct ElementProxy {
    element: Rc<RefCell<dyn std::any::Any>>,
}

impl ElementProxy {
    /// Create a new ElementProxy wrapping an XML element
    pub fn new(element: Rc<RefCell<dyn std::any::Any>>) -> Self {
        ElementProxy { element }
    }

    /// Get the wrapped XML element
    pub fn element(&self) -> Rc<RefCell<dyn std::any::Any>> {
        Rc::clone(&self.element)
    }
}

impl PartialEq for ElementProxy {
    fn eq(&self, other: &Self) -> bool {
        // Compare by reference equality
        Rc::ptr_eq(&self.element, &other.element)
    }
}

impl Eq for ElementProxy {}

/// Provides access to ancestor objects and part
pub struct ParentedElementProxy {
    element: Rc<RefCell<dyn std::any::Any>>,
    parent: Rc<RefCell<dyn std::any::Any>>,
}

impl ParentedElementProxy {
    /// Create a new ParentedElementProxy
    pub fn new(
        element: Rc<RefCell<dyn std::any::Any>>,
        parent: Rc<RefCell<dyn std::any::Any>>,
    ) -> Self {
        ParentedElementProxy { element, parent }
    }

    /// Get the parent proxy object
    pub fn parent(&self) -> Rc<RefCell<dyn std::any::Any>> {
        Rc::clone(&self.parent)
    }

    /// Get the XML element
    pub fn element(&self) -> Rc<RefCell<dyn std::any::Any>> {
        Rc::clone(&self.element)
    }
}

/// Provides common members for proxy-objects that wrap a part's root element
pub struct PartElementProxy {
    element: Rc<RefCell<dyn std::any::Any>>,
    part: Rc<RefCell<dyn std::any::Any>>,
}

impl PartElementProxy {
    /// Create a new PartElementProxy
    pub fn new(
        element: Rc<RefCell<dyn std::any::Any>>,
        part: Rc<RefCell<dyn std::any::Any>>,
    ) -> Self {
        PartElementProxy { element, part }
    }

    /// Get the XML element
    pub fn element(&self) -> Rc<RefCell<dyn std::any::Any>> {
        Rc::clone(&self.element)
    }

    /// Get the part
    pub fn part(&self) -> Rc<RefCell<dyn std::any::Any>> {
        Rc::clone(&self.part)
    }
}
