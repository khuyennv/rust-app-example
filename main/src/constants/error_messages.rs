pub struct Messages {}

impl Messages {
    pub const INVALID_REQUEST: &'static str = "Yêu cầu không đúng, mời bạn thử lại!";
    pub const SYSTEM_GENERAL_ERROR: &'static str = "Có lỗi xẩy ra mời bạn thử lại.";
    pub const USER_NOT_PERMISSION: &'static str = "Bạn không có quyền thực hiện chức năng này.";
    pub const USER_NOT_EXIST_OR_IS_BLOCKING: &'static str = "Người dùng không tồn tại hoặc bị khóa";
}
