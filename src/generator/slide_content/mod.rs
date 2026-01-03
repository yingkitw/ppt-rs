//! Slide content and layout types
//!
//! This module contains types for building slide content:
//! - `BulletStyle` - Bullet point styles (numbered, lettered, roman, custom)
//! - `BulletPoint` - Individual bullet point with formatting
//! - `BulletTextFormat` - Text formatting for bullet points
//! - `SlideLayout` - Layout types (title only, title and content, etc.)
//! - `SlideContent` - Complete slide content builder
//! - `CodeBlock` - Code block with syntax highlighting

mod bullet;
mod layout;
mod code_block;
mod content;
pub mod transition;

pub use bullet::{BulletStyle, BulletPoint, BulletTextFormat};
pub use layout::SlideLayout;
pub use code_block::CodeBlock;
pub use content::SlideContent;
pub use transition::TransitionType;

