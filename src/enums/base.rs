//! Base enumeration types

use std::collections::HashMap;

/// Base enumeration type with MS API values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaseEnum {
    pub name: &'static str,
    pub value: i32,
    pub doc: &'static str,
}

impl BaseEnum {
    /// Create a new BaseEnum
    pub const fn new(name: &'static str, value: i32, doc: &'static str) -> Self {
        BaseEnum { name, value, doc }
    }
}

impl std::fmt::Display for BaseEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.value)
    }
}

/// Enumeration type that maps to XML attribute values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaseXmlEnum {
    pub name: &'static str,
    pub value: i32,
    pub xml_value: Option<&'static str>,
    pub doc: &'static str,
}

impl BaseXmlEnum {
    /// Create a new BaseXmlEnum
    pub const fn new(
        name: &'static str,
        value: i32,
        xml_value: Option<&'static str>,
        doc: &'static str,
    ) -> Self {
        BaseXmlEnum {
            name,
            value,
            xml_value,
            doc,
        }
    }

    /// Get enumeration member from XML value
    pub fn from_xml(xml_value: &str, members: &[BaseXmlEnum]) -> Result<BaseXmlEnum, String> {
        if xml_value.is_empty() {
            return Err("Empty XML value".to_string());
        }

        members
            .iter()
            .find(|m| m.xml_value == Some(xml_value))
            .copied()
            .ok_or_else(|| format!("No XML mapping for {}", xml_value))
    }

    /// Get XML value for enumeration member
    pub fn to_xml(&self) -> Result<&'static str, String> {
        self.xml_value
            .ok_or_else(|| format!("{} has no XML representation", self.name))
    }
}

impl std::fmt::Display for BaseXmlEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.value)
    }
}

/// Registry for enum members
pub struct EnumRegistry {
    members: HashMap<String, BaseXmlEnum>,
}

impl EnumRegistry {
    /// Create a new EnumRegistry
    pub fn new() -> Self {
        EnumRegistry {
            members: HashMap::new(),
        }
    }

    /// Register an enum member
    pub fn register(&mut self, name: String, member: BaseXmlEnum) {
        self.members.insert(name, member);
    }

    /// Get an enum member by name
    pub fn get(&self, name: &str) -> Option<BaseXmlEnum> {
        self.members.get(name).copied()
    }
}

impl Default for EnumRegistry {
    fn default() -> Self {
        Self::new()
    }
}
