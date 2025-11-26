//! Language enumeration types

use super::BaseEnum;

/// Specifies the language identifier
pub struct MsoLanguageID;

impl MsoLanguageID {
    pub const ENGLISH_US: BaseEnum = BaseEnum::new("ENGLISH_US", 1033, "English (United States).");
    pub const ENGLISH_UK: BaseEnum = BaseEnum::new("ENGLISH_UK", 2057, "English (United Kingdom).");
    pub const SPANISH: BaseEnum = BaseEnum::new("SPANISH", 1034, "Spanish.");
    pub const FRENCH: BaseEnum = BaseEnum::new("FRENCH", 1036, "French.");
    pub const GERMAN: BaseEnum = BaseEnum::new("GERMAN", 1031, "German.");
    pub const ITALIAN: BaseEnum = BaseEnum::new("ITALIAN", 1040, "Italian.");
    pub const PORTUGUESE: BaseEnum = BaseEnum::new("PORTUGUESE", 2070, "Portuguese.");
    pub const DUTCH: BaseEnum = BaseEnum::new("DUTCH", 1043, "Dutch.");
    pub const RUSSIAN: BaseEnum = BaseEnum::new("RUSSIAN", 1049, "Russian.");
    pub const CHINESE_SIMPLIFIED: BaseEnum = BaseEnum::new("CHINESE_SIMPLIFIED", 2052, "Chinese (Simplified).");
    pub const CHINESE_TRADITIONAL: BaseEnum = BaseEnum::new("CHINESE_TRADITIONAL", 1028, "Chinese (Traditional).");
    pub const JAPANESE: BaseEnum = BaseEnum::new("JAPANESE", 1041, "Japanese.");
    pub const KOREAN: BaseEnum = BaseEnum::new("KOREAN", 1042, "Korean.");
}
