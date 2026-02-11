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
pub mod comments;
pub mod sections;
pub mod digital_signature;
pub mod ink_annotations;
pub mod slide_show_settings;
pub mod print_settings;
pub mod table_merge;
pub mod embedded_fonts;
pub mod presentation_settings;

pub use bullet::{BulletStyle, BulletPoint, BulletTextFormat};
pub use layout::SlideLayout;
pub use code_block::CodeBlock;
pub use content::SlideContent;
pub use transition::TransitionType;
pub use comments::{Comment, CommentAuthor, CommentAuthorList, SlideComments};
pub use sections::{SlideSection, SectionManager};
pub use digital_signature::{DigitalSignature, SignerInfo, HashAlgorithm, SignatureCommitment};
pub use ink_annotations::{InkAnnotations, InkStroke, InkPen, InkPoint, PenTip};
pub use slide_show_settings::{SlideShowSettings, ShowType, PenColor, SlideRange};
pub use print_settings::{PrintSettings, HandoutLayout, PrintColorMode, PrintWhat, Orientation};
pub use table_merge::{TableMergeMap, MergeRegion, CellMergeState};
pub use embedded_fonts::{EmbeddedFontList, EmbeddedFont, FontStyle, FontCharset};
pub use presentation_settings::PresentationSettings;

