#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

use crate::app::{Server};

mod app;
mod components;
mod config;
mod constants;
mod controllers;
mod entities;
mod errors;
mod middlewares;
mod routes;
mod services;
mod test;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Server::run().await
}

mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use actix_web::test::read_response;

    #[actix_rt::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_async))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        let req1 = test::TestRequest::with_header("content-type", "text/plain").to_request();

        let body = read_response(&mut app, req1).await;
        println!("{:?}", resp);
        assert_eq!(b"Hello world!\r\n", body.as_ref());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_index_post() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_async))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_client_error());
    }
}