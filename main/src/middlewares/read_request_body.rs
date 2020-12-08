use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use bytes::BytesMut;
use futures::future::{ok, Future, Ready};
use futures::stream::StreamExt;
use bytes::Bytes;
use regex::internal::Input;

pub struct LoggingSystem;

impl<S: 'static, B> Transform<S> for LoggingSystem
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for LoggingMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Errror;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {:?}", req.path());
        println!("Hi from start. You requested: {:?}", req.uri());
        println!("Hi from start. You requested: {:?}", req.connection_info());

        let mut svc = self.service.clone();

        Box::pin(async move {
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}