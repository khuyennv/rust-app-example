pub struct ErrorCodes {}

impl ErrorCodes {
    pub const UNKNOWN: u16 = 900;
    pub const INVALID_REQUEST: u16 = 901;
    // General error
    pub const SYSTEM_GENERAL_ERROR: u16 = 1000;
    // System, User 10xx
    pub const GAPO_ROLE_NOT_SUPPORT: u16 = 1001;
    pub const USER_NOT_PERMISSION: u16 = 1002;
    pub const USER_NOT_EXIST_OR_IS_BLOCKING: u16 = 1003;
    // pub const PAGE_NOT_EXIST_OR_IS_BLOCKING: u16 = 1004;
    // pub const HASHTAG_NOT_EXIST_OR_IS_BLOCKING: u16 = 1008;
    // pub const GROUP_NOT_EXIST_OR_IS_BLOCKING: u16 = 1009;
}
