use crate::constants::error_codes::ErrorCodes;
use crate::constants::error_messages::Messages;
use crate::errors::ApiError;

pub enum XGapoRole {
    Service,
    User,
}

impl XGapoRole {
    #[allow(unused)]
    pub fn from_str(s: &str) -> Result<XGapoRole, ApiError> {
        match s.to_lowercase().as_ref() {
            "service" => Result::Ok(XGapoRole::Service),
            "user" => Result::Ok(XGapoRole::User),
            _ => Err(ApiError::new(
                403,
                Messages::USER_NOT_PERMISSION.to_string(),
                ErrorCodes::GAPO_ROLE_NOT_SUPPORT,
                Some(Messages::USER_NOT_PERMISSION.to_string()),
                None,
            )),
        }
    }
}
