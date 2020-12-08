use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct IamKey {
    pub apiKey: String,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct IamKeysResult {
    pub data: Vec<IamKey>,
}

pub const USER_INFO_FIELDS: &'static str =
    "id,display_name,full_name,cover,avatar,link_profile,status,status_verify,avatar_thumb_pattern,cover_thumb_pattern";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub id: i64,
    pub full_name: String,
    pub display_name: String,
    pub cover: String,
    pub avatar: String,
    pub link_profile: String,
    pub status: u32,
    pub status_verify: u32,
    pub avatar_thumb_pattern: String,
    pub cover_thumb_pattern: String,
}

impl UserInfo {}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCoreResult {
    pub data: UserInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsersCoreResult {
    pub data: Vec<UserInfo>,
}