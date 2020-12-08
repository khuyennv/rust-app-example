use crate::components::databases::redis_db::RedisDB;
use crate::config::CONFIG;
use crate::middlewares::before_action_middleware;
use crate::routes;
use crate::services::iam_service::get_iam_keys_for_init;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{web, App, HttpServer};
use async_std::task;
use proptest::std_facade::hash_map::RandomState;
use proptest::std_facade::HashMap;
use sentry::ClientInitGuard;
use std::str::FromStr;
use std::sync::Mutex;

pub struct Application {}

impl Application {
    pub fn init() {
        if cfg!(test) {
            dotenv::from_filename(".env.test").ok();
        } else {
            dotenv::from_filename(".env").ok();
        }
        // env::set_var("RUST_BACKTRACE", "1");
        let config = &CONFIG;

        std::env::set_var("RUST_LOG", config.rust_log.clone());
    }

    pub fn init_sentry(sentry_url: String) -> ClientInitGuard {
        // init sentry
        let dsn = sentry::types::Dsn::from_str(&sentry_url.as_str()).unwrap();
        let opt = sentry::ClientOptions {
            release: sentry::release_name!(),
            dsn: Some(dsn),
            debug: true,
            ..Default::default()
        };
        sentry::init(opt)
    }

    #[allow(unused)]
    pub fn env_is(env: String) -> bool {
        let config = &CONFIG;

        config.env == env
    }

    pub fn config_app() -> Box<dyn Fn(&mut ServiceConfig)> {
        Box::new(move |cfg: &mut ServiceConfig| {
            routes::init_routes(cfg);
            Application::config_database(cfg);
        })
    }

    pub fn config_database(cfg: &mut actix_web::web::ServiceConfig) {
        let config = &CONFIG;
        let redis = RedisDB::connect(config.redis_uri.clone());

        cfg.data(redis.clone());
    }

    pub async fn get_iam_keys() -> Data<Mutex<HashMap<String, String>>> {
        let hash_map = get_iam_keys_for_init().await;

        web::Data::new(Mutex::new(hash_map))
    }
}

pub struct Server {}

impl Server {
    pub async fn run() -> std::io::Result<()> {
        let config = &CONFIG;

        Application::init();
        let dsn = sentry::types::Dsn::from_str(&config.sentry_url.as_str()).unwrap();
        let opt = sentry::ClientOptions {
            release: sentry::release_name!(),
            dsn: Some(dsn),
            debug: true,
            ..Default::default()
        };
        let _sentry = sentry::init(opt);
        //
        // let _sentry = Application::init_sentry(config.sentry_url.clone());
        let iam_keys = Application::get_iam_keys().await;

        // start server
        HttpServer::new(move || {
            App::new()
                .configure(Application::config_app())
                .app_data(iam_keys.clone())
                .wrap(before_action_middleware::BeforeAction)
        })
            .bind(config.server.as_str())
            .unwrap()
            .keep_alive(0)
            .client_shutdown(1000)
            .client_timeout(1000)
            .workers(1)
            .run()
            .await
    }
}
