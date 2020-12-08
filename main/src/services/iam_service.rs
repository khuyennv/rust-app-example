use std::collections::HashMap;

use super::gapo_api_service::get_iam_keys;

#[allow(unused)]
pub async fn get_iam_keys_for_init() -> HashMap<String, String> {
    let iam_keys = match get_iam_keys().await {
        Ok(h) => h,
        Err(_er) => vec![],
    };

    let mut hash: HashMap<String, String> = HashMap::new();
    iam_keys.iter().for_each(|iam_key| {
        hash.insert(iam_key.apiKey.clone(), iam_key.source.clone());
    });

    hash
}
