use std::collections::HashMap;

use actix_web::web::Data;
use reqwest::header;
use serde::de::DeserializeOwned;

use crate::components::databases::redis_db::RedisDB;
use crate::config;
use crate::config::CONFIG;
use crate::constants::error_codes::ErrorCodes;
use crate::constants::error_messages::Messages;
use crate::entities::app_entity::*;
use crate::errors::ApiError;

async fn get_request<T: DeserializeOwned>(url: String, key: &'static str) -> Result<T, ApiError> {
    let mut headers = header::HeaderMap::new();
    headers.insert("x-gapo-role", header::HeaderValue::from_static("service"));
    headers.insert("x-gapo-api-key", header::HeaderValue::from_static(key));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let res = client.get(&url).send().await;

    match res {
        Ok(body) => {
            if body.status() != 200 {
                return Err(ApiError::new(
                    body.status().as_u16(),
                    Messages::SYSTEM_GENERAL_ERROR.to_string(),
                    ErrorCodes::SYSTEM_GENERAL_ERROR,
                    Some(Messages::SYSTEM_GENERAL_ERROR.to_string()),
                    None,
                ));
            }

            match body.json::<T>().await {
                Ok(value) => Ok(value),
                Err(er) => {
                    return Err(ApiError::new(
                        500,
                        er.to_string(),
                        ErrorCodes::SYSTEM_GENERAL_ERROR,
                        Some(er.to_string()),
                        None,
                    ));
                }
            }
        }
        Err(_e) => {
            return Err(ApiError::new(
                400,
                Messages::SYSTEM_GENERAL_ERROR.to_string(),
                ErrorCodes::SYSTEM_GENERAL_ERROR,
                Some(Messages::SYSTEM_GENERAL_ERROR.to_string()),
                None,
            ));
        }
    }
}

pub async fn get_user(user_id: &i64, fields: &String) -> Result<UserInfo, ApiError> {
    let config = &CONFIG;
    let url = format!(
        "{}/users/{}?fields={}",
        config.user_core_api_url, user_id, fields
    );
    match get_request::<UserCoreResult>(url, config.user_core_api_key.as_str()).await {
        Ok(val) => Ok(val.data),
        Err(_er) => {
            return Err(ApiError::new(
                400,
                Messages::USER_NOT_EXIST_OR_IS_BLOCKING.to_string(),
                ErrorCodes::USER_NOT_EXIST_OR_IS_BLOCKING,
                Some(Messages::USER_NOT_EXIST_OR_IS_BLOCKING.to_string()),
                None,
            ));
        }
    }
}

#[allow(unused)]
pub async fn get_user_from_cache(
    redis: &Data<RedisDB>,
    user_id: &i64,
    fields: &String,
) -> Result<UserInfo, ApiError> {
    let user = redis.get::<String>(format!("UserCore:{}", user_id));
    if user.is_ok() {
        let user_info_str = user.unwrap();
        if let Ok(user_info) = serde_json::from_str(user_info_str.as_ref()) {
            return Ok(user_info);
        }
    } else {
        debug!("get_user_from_cache: {:?}", user);
    }

    match get_user(user_id, fields).await {
        Ok(user_info) => {
            redis.set(
                format!("UserCore:{}", user_id),
                serde_json::to_string(&user_info).unwrap(),
                config::CACHE_USER_CORE_TIME,
            );
            Ok(user_info)
        }
        Err(err) => Err(err),
    }
}

#[allow(unused)]
pub async fn get_users(ids: Vec<String>, fields: &String) -> Result<Vec<UserInfo>, ApiError> {
    let config = &CONFIG;
    let url = format!(
        "{}/users?ids={}&fields={}",
        config.user_core_api_url,
        ids.join(","),
        fields
    );
    match get_request::<UsersCoreResult>(url, config.user_core_api_key.as_str()).await {
        Ok(val) => Ok(val.data),
        Err(_er) => {
            return Err(ApiError::new(
                400,
                Messages::USER_NOT_EXIST_OR_IS_BLOCKING.to_string(),
                ErrorCodes::USER_NOT_EXIST_OR_IS_BLOCKING,
                Some(Messages::USER_NOT_EXIST_OR_IS_BLOCKING.to_string()),
                None,
            ));
        }
    }
}

#[allow(unused)]
pub async fn get_users_from_cache(
    redis: &Data<RedisDB>,
    ids: Vec<String>,
    fields: &String,
) -> Result<HashMap<String, UserInfo>, ApiError> {
    let mut users_map: HashMap<String, UserInfo> = HashMap::new();
    let mut ids_vec_not_cache: Vec<String> = vec![];

    let keys: Vec<String> = ids
        .clone()
        .into_iter()
        .map(|item| format!("UserCore:{}", item.trim()))
        .collect();
    let users_cache = redis.mget(keys.clone());

    if users_cache.is_ok() {
        let mut id: usize = 0;
        for item in users_cache.unwrap() {
            let ids_vec_clone = ids.clone();
            if item == "nil" {
                ids_vec_not_cache.push(ids_vec_clone[id].trim().to_string());
            } else {
                if let Ok(user_info) = serde_json::from_str(item.as_ref()) {
                    users_map.insert(ids_vec_clone[id].to_string(), user_info);
                }
            }

            id += 1;
        }

        if ids_vec_not_cache.len() > 0 {
            let users_info = get_users(ids_vec_not_cache, &USER_INFO_FIELDS.to_string()).await;
            if users_info.is_ok() {
                for user_info in users_info.unwrap() {
                    redis.set(
                        format!("UserCore:{}", user_info.id),
                        serde_json::to_string(&user_info).unwrap(),
                        config::CACHE_USER_CORE_TIME,
                    );

                    users_map.insert(user_info.id.to_string(), user_info);
                }
            }
        }
    }

    Ok(users_map)
}

#[allow(unused)]
pub async fn get_iam_keys() -> Result<Vec<IamKey>, ApiError> {
    let config = &CONFIG;
    let url = config.iam_api.clone();

    match get_request::<IamKeysResult>(url, config.iam_key.as_str()).await {
        Ok(h) => Ok(h.data),
        Err(err) => {
            return Err(ApiError::new(
                400,
                err.to_string(),
                ErrorCodes::UNKNOWN,
                Some(err.to_string()),
                None,
            ));
        }
    }
}
