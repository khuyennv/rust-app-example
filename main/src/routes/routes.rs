use crate::controllers::index_controller;

pub fn init_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(index_controller::index);
    cfg.service(index_controller::index_test);
}
