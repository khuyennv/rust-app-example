use crate::components::databases::redis_db::RedisDB;
use crate::constants::error_codes::*;
use crate::constants::error_messages::*;
use crate::errors::*;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use codegen::validate_request;

#[get("/")]
pub async fn index(redis: Data<RedisDB>, req: HttpRequest) -> Result<impl Responder, ApiError> {
    // let ret = redis.get::<String>("khuyentest1".to_string());
    // println!("ret1:{:?}", ret);
    // let ret = redis.get::<Vec<u8>>("khuyentest1".to_string());
    // println!("ret2:{:?}", ret);

    // let ret = redis.set_nx("khuyentest12".to_string(), 10.to_string(), 3343);
    // println!("req: {:?}", req);
    // info!("{}", "dung");

    if let Some(hdr) = req.headers().get(header::CONTENT_TYPE) {
        Ok(HttpResponse::Ok().body("Hello world"))
    } else {
        Ok(HttpResponse::Ok().body("bye the world"))
    }
}
