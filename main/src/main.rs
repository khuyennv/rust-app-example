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