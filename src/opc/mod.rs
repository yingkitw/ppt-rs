//! OPC (Open Packaging Convention) package handling

pub mod package;
pub mod compress;

pub use package::Package;
pub use compress::{
    compress_pptx, compress_pptx_in_memory, analyze_pptx,
    CompressionOptions, CompressionLevel, CompressionResult,
    PptxAnalysis
};
