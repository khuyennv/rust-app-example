//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use std::env;

use serde::Deserialize;

pub const ACTOR_FOR_EVERY_WORKER: usize = 2;
pub const WORKER: usize = 1;
pub const RATE_LIMIT_DETECT_DUPLICATE_TIME: usize = 2; // second
pub const CACHE_USER_CORE_TIME: usize = 30 * 60; //second

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub env: String,
    pub rust_log: String,
    pub server: String,
    pub actor_for_every_worker: usize,
    pub worker: usize,
    pub rate_limit_detect_duplicate_time: usize,
    pub sentry_url: String,
    pub rabbitmq_uri: String,
    pub user_core_api_url: String,
    pub user_core_api_key: String,
    pub iam_api: String,
    pub iam_key: String,
    pub redis_uri: String,
}

impl Config {
    #[allow(unused)]
    pub fn env(&self, val: String) -> bool {
        if self.env == val {
            return true;
        }

        false
    }
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    /** Config Config **/
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
pub fn get_config() -> Config {
    if cfg!(test) {
        dotenv::from_filename(".env.test").ok();
    } else {
        dotenv::from_filename(".env").ok();
    }

    //set env
    let env = env::var("ENV").unwrap_or_else(|_| "dev".to_string());

    // Set log level
    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "trace".to_string());
    std::env::set_var("RUST_LOG", &rust_log);
    if !cfg!(test) {
        env_logger::builder().format_module_path(false).init();
    }

    let server = env::var("SERVER").unwrap_or_else(|_| "127.0.0.1:5000".to_string());
    let rabbitmq_uri = env::var("RABBITMQ_URI").unwrap();
    let sentry_url = env::var("SENTRY_URI").unwrap();
    let iam_api = env::var("IAM_API").unwrap();
    let iam_key = env::var("IAM_KEY").unwrap();

    let user_core_api_url = env::var("USER_CORE_API_URL").unwrap();
    let user_core_api_key = env::var("USER_CORE_API_KEY").unwrap();
    let redis_uri = env::var("REDIS_URI").unwrap();

    Config {
        env,
        rust_log,
        server,
        actor_for_every_worker: ACTOR_FOR_EVERY_WORKER,
        worker: WORKER,
        rate_limit_detect_duplicate_time: RATE_LIMIT_DETECT_DUPLICATE_TIME,
        sentry_url,
        rabbitmq_uri,
        user_core_api_url,
        user_core_api_key,
        iam_api,
        iam_key,
        redis_uri,
    }
}
