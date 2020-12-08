use std::{collections::HashMap, io, sync::Mutex, task::{Context, Poll}};
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;

use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest,
    dev::ServiceResponse,
    Error,
    error,
    error::ErrorBadGateway,
    http::header::{HeaderName, HeaderValue},
    http::StatusCode,
    web,
    web::Data,
};
use futures::future::{Future, ok, Ready};
use sentry::configure_scope;
use serde_json::json;

use crate::services::iam_service::get_iam_keys_for_init;
use crate::errors::ApiError;
use crate::constants::error_codes::ErrorCodes;
use crate::constants::error_messages::Messages;
use sentry_backtrace::current_stacktrace;
use crate::utils::request_info_utils::{get_request_info_from_service_request};

pub struct BeforeAction;

impl<S: 'static, B> Transform<S> for BeforeAction
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = BeforeActionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(BeforeActionMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct BeforeActionMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for BeforeActionMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let role = match req.headers().get("x-gapo-role") {
            Some(value) => value.to_str().unwrap().to_string(),
            _ => "".to_string(),
        };

        // set default sentry
        let headers = req.headers();
        let uri = format!("{}?{}", req.path(), req.query_string());

        configure_scope(move |scope| {
            for (key, value) in headers.iter() {
                scope.set_tag(key.as_str(), value.to_str().unwrap_or("").to_string());
            }

            scope.set_tag("uri", uri);
        });

        let mut svc = self.service.clone();

        Box::pin(async move {
            let mut valid_request = "reject";

            if role == "service" {
                let iam_keys_data = req
                    .app_data::<Data<Mutex<HashMap<String, String>>>>()
                    .expect("get iam key from app_data failse");

                let mut iam_keys = iam_keys_data.lock().unwrap();

                let key = match req.headers().get("x-gapo-api-key") {
                    Some(value) => value.to_str().unwrap().to_string(),
                    _ => "".to_string(),
                };

                if key != "" {
                    if let None = iam_keys.get(&key) {
                        // info!("recall iam!");
                        let hashs = get_iam_keys_for_init().await;

                        if let Some(_v) = hashs.get(&key) {
                            valid_request = "accepted";
                        }

                        for (k, v) in hashs.iter() {
                            &iam_keys.insert(k.to_string(), v.to_string());
                        }
                    } else {
                        valid_request = "accepted";
                    }
                }
            } else {
                valid_request = "accepted";
            }

            if valid_request != "accepted" {
                return Err(ApiError {
                    http_code: 403,
                    message:  Messages::INVALID_REQUEST.to_string(),
                    code: ErrorCodes::INVALID_REQUEST,
                    cause: Some("x-gapo-key-api, hoac x-gapo-role không đúng".to_string()),
                    backtrace: current_stacktrace(),
                    info: get_request_info_from_service_request(&req)
                }.into())
            }

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
