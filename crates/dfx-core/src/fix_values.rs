#![allow(non_snake_case)]

use crate::field_map::Tag;

pub struct BeginString {}
impl BeginString {
    pub const FIXT11: &'static str = "FIXT.1.1";
    pub const FIX50SP2: &'static str = "FIX.5.0SP2";
    pub const FIX50SP1: &'static str = "FIX.5.0SP1";
    pub const FIX50: &'static str = "FIX.5.0";
    pub const FIX44: &'static str = "FIX.4.4";
    pub const FIX43: &'static str = "FIX.4.3";
    pub const FIX42: &'static str = "FIX.4.2";
    pub const FIX41: &'static str = "FIX.4.1";
    pub const FIX40: &'static str = "FIX.4.0";
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionRejectReason {
    tag: Tag,
    reason: String,
}

impl SessionRejectReason {
    pub fn INVALID_TAG_NUMBER() -> SessionRejectReason { SessionRejectReason { tag: 0, reason: "Invalid tag number".to_string() } }
    pub fn REQUIRED_TAG_MISSING() -> SessionRejectReason { SessionRejectReason { tag: 1, reason: "Required tag missing".to_string() } }
    pub fn TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE() -> SessionRejectReason { SessionRejectReason { tag: 2, reason: "Tag not defined for this message type".to_string() } }
    pub fn UNDEFINED_TAG() -> SessionRejectReason { SessionRejectReason { tag: 3, reason: "Undefined Tag".to_string() } }
    pub fn TAG_SPECIFIED_WITHOUT_A_VALUE() -> SessionRejectReason { SessionRejectReason { tag: 4, reason: "Tag specified without a value".to_string() } }
    pub fn VALUE_IS_INCORRECT() -> SessionRejectReason { SessionRejectReason { tag: 5, reason: "Value is incorrect (out of range) for this tag".to_string() } }
    pub fn INCORRECT_DATA_FORMAT_FOR_VALUE() -> SessionRejectReason { SessionRejectReason { tag: 6, reason: "Incorrect data format for value".to_string() } }
    pub fn DECRYPTION_PROBLEM() -> SessionRejectReason { SessionRejectReason { tag: 7, reason: "Decryption problem".to_string() } }
    pub fn SIGNATURE_PROBLEM() -> SessionRejectReason { SessionRejectReason { tag: 8, reason: "Signature problem".to_string() } }
    pub fn COMPID_PROBLEM() -> SessionRejectReason { SessionRejectReason { tag: 9, reason: "CompID problem".to_string() } }
    pub fn SENDING_TIME_ACCURACY_PROBLEM() -> SessionRejectReason { SessionRejectReason { tag: 10, reason: "SendingTime accuracy problem".to_string() } }
    pub fn INVALID_MSGTYPE() -> SessionRejectReason { SessionRejectReason { tag: 11, reason: "Invalid MsgType".to_string() } }
    pub fn XML_VALIDATION_ERROR() -> SessionRejectReason { SessionRejectReason { tag: 12, reason: "XML validation error".to_string() } }
    pub fn TAG_APPEARS_MORE_THAN_ONCE() -> SessionRejectReason { SessionRejectReason { tag: 13, reason: "Tag appears more than once".to_string() } }
    pub fn TAG_SPECIFIED_OUT_OF_REQUIRED_ORDER() -> SessionRejectReason { SessionRejectReason { tag: 14, reason: "Tag specified out of required order".to_string() } }
    pub fn REPEATING_GROUP_FIELDS_OUT_OF_ORDER() -> SessionRejectReason { SessionRejectReason { tag: 15, reason: "Repeating group fields out of order".to_string() } }
    pub fn INCORRECT_NUM_IN_GROUP_COUNT_FOR_REPEATING_GROUP() -> SessionRejectReason { SessionRejectReason { tag: 16, reason: "Incorrect NumInGroup count for repeating group".to_string() } }
    pub fn NON_DATA_VALUE_INCLUDES_FIELD_DELIMITER() -> SessionRejectReason { SessionRejectReason { tag: 17, reason: "Non-data value includes field delimiter".to_string() } }
    pub fn OTHER(reason: String) -> SessionRejectReason { SessionRejectReason { tag: 99, reason } }

    pub fn tag(&self) -> Tag {
        self.tag
    }

    pub fn reason(&self) -> &str {
        self.reason.as_ref()
    }

    pub fn description(&self) -> String {
        format!("{}", self.reason)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BusinessRejectReason {
    index: usize,
    reason: String,
}

impl BusinessRejectReason {
    pub fn UNKNOWN_MESSAGE_TYPE() -> BusinessRejectReason { BusinessRejectReason { index: 3, reason: "Unsupported Message Type".to_string() } }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn reason(&self) -> &str {
        self.reason.as_ref()
    }

    pub fn description(&self) -> String {
        format!("{}", self.reason)
    }
}
