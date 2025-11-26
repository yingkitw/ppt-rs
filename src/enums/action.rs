//! Action enumeration types

use super::BaseEnum;

/// Specifies the type of a mouse action (click or hover action)
pub struct PpActionType;

impl PpActionType {
    pub const END_SHOW: BaseEnum = BaseEnum::new("END_SHOW", 6, "Slide show ends.");
    pub const FIRST_SLIDE: BaseEnum = BaseEnum::new("FIRST_SLIDE", 3, "Returns to the first slide.");
    pub const HYPERLINK: BaseEnum = BaseEnum::new("HYPERLINK", 7, "Hyperlink.");
    pub const LAST_SLIDE: BaseEnum = BaseEnum::new("LAST_SLIDE", 4, "Moves to the last slide.");
    pub const LAST_SLIDE_VIEWED: BaseEnum = BaseEnum::new("LAST_SLIDE_VIEWED", 5, "Moves to the last slide viewed.");
    pub const NAMED_SLIDE: BaseEnum = BaseEnum::new("NAMED_SLIDE", 101, "Moves to slide specified by slide number.");
    pub const NAMED_SLIDE_SHOW: BaseEnum = BaseEnum::new("NAMED_SLIDE_SHOW", 10, "Runs the slideshow.");
    pub const NEXT_SLIDE: BaseEnum = BaseEnum::new("NEXT_SLIDE", 1, "Moves to the next slide.");
    pub const NONE: BaseEnum = BaseEnum::new("NONE", 0, "No action is performed.");
    pub const OPEN_FILE: BaseEnum = BaseEnum::new("OPEN_FILE", 102, "Opens the specified file.");
    pub const OLE_VERB: BaseEnum = BaseEnum::new("OLE_VERB", 11, "OLE Verb.");
    pub const PLAY: BaseEnum = BaseEnum::new("PLAY", 12, "Begins the slideshow.");
    pub const PREVIOUS_SLIDE: BaseEnum = BaseEnum::new("PREVIOUS_SLIDE", 2, "Moves to the previous slide.");
    pub const RUN_MACRO: BaseEnum = BaseEnum::new("RUN_MACRO", 8, "Runs a macro.");
    pub const RUN_PROGRAM: BaseEnum = BaseEnum::new("RUN_PROGRAM", 9, "Runs a program.");
}

pub type PpAction = PpActionType;
