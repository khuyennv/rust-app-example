// src/api_error.rs
use actix_web::{error::ResponseError, http::StatusCode, web, Error};
use sentry::protocol::{Event, Level};
use sentry::{configure_scope, types::Uuid};
use sentry_backtrace::Stacktrace;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestInfo {
    pub headers: Option<HashMap<String, String>>,
    pub tags: Option<HashMap<String, String>>,
    pub body: Option<HashMap<String, String>>,
    pub uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub http_code: u16,
    pub message: String,
    pub code: u16,
    pub cause: Option<String>,
    pub backtrace: Option<Stacktrace>,
    pub info: Option<RequestInfo>,
}

impl ApiError {
    #[allow(unused)]
    pub fn new(
        http_code: u16,
        message: String,
        code: u16,
        cause: Option<String>,
        backtrace: Option<Stacktrace>,
    ) -> ApiError {
        ApiError {
            http_code,
            message,
            code,
            cause,
            backtrace,
            info: None,
        }
    }

    #[allow(unused)]
    pub fn one(
        http_code: u16,
        message: String,
        code: u16,
        cause: Option<String>,
        backtrace: Option<Stacktrace>,
        info: Option<RequestInfo>,
    ) -> ApiError {
        ApiError {
            http_code,
            message,
            code,
            cause,
            backtrace,
            info,
        }
    }
}

impl Default for ApiError {
    fn default() -> ApiError {
        ApiError {
            http_code: 200,
            message: "Successfully!".to_string(),
            code: 0,
            cause: None,
            backtrace: None,
            info: None,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let a = match to_string_pretty(self) {
            Ok(val) => val,
            Err(er) => er.to_string(),
        };

        write!(f, "{}", a)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl ResponseError for ApiError {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> web::HttpResponse {
        let err_json = json!({
            "message": self.message,
            "http_code": self.http_code,
            "code": self.code
        });

        self.sent_to_sentry();

        let code = match StatusCode::from_u16(self.http_code) {
            Ok(val) => val,
            Err(_err) => StatusCode::OK,
        };

        web::HttpResponse::build(code).json(err_json)
    }
}

impl ApiError {
    fn sent_to_sentry(&self) {
        let trace_max_level = 5;
        if self.http_code > 400 {
            let uuid = Uuid::new_v4();
            let mut stacktrace = Stacktrace::default();

            // Get 10 frames of end
            if let Some(bt) = self.backtrace.clone() {
                let len = bt.frames.len();
                let start = {
                    if len > trace_max_level {
                        len - trace_max_level
                    } else {
                        0
                    }                };

                stacktrace.frames = bt.frames[start..len].to_vec();
            }

            // Add more info to log
            if let Some(request_info) = &self.info {
                configure_scope(move |scope| {
                    for (key, value) in &request_info.headers.clone().unwrap() {
                        scope.set_tag(key, value);
                    }

                    scope.set_tag("uri", &request_info.uri.clone().unwrap());
                });
            }

            let event = Event {
                event_id: uuid,
                message: Some(self.message.clone()),
                level: Level::Info,
                stacktrace: Some(stacktrace),
                ..Default::default()
            };

            sentry::capture_event(event.clone());
        }
    }
}
