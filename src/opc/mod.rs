//! OPC (Open Packaging Convention) package handling

pub mod constants;
pub mod package;
pub mod packuri;
pub mod shared;

pub use package::Package;
pub use packuri::PackUri;
