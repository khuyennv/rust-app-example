use actix_web::HttpRequest;
use std::collections::HashMap;
use crate::errors::RequestInfo;
use actix_web::dev::ServiceRequest;

#[allow(unused)]
pub fn get_request_info(req: &HttpRequest) -> Option<RequestInfo> {
    let headers: HashMap<String, String> = HashMap::new();

    // for (key, value) in req.headers().iter() {
    //     headers.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
    // }
    
    let tags: HashMap<String, String> = HashMap::new();
    let body: HashMap<String, String> = HashMap::new();
    let uri: String = req.uri().to_string();

    Some(RequestInfo { headers: Some(headers), tags: Some(tags), uri: Some(uri), body: Some(body)})
}

pub fn get_request_info_from_service_request(req: &ServiceRequest) -> Option<RequestInfo> {
    let headers: HashMap<String, String> = HashMap::new();

    let tags: HashMap<String, String> = HashMap::new();
    let body: HashMap<String, String> = HashMap::new();
    let uri: String = req.uri().to_string();

    Some(RequestInfo { headers: Some(headers), tags: Some(tags), uri: Some(uri), body: Some(body)})
}