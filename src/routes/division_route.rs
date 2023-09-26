mod routes {
    // use crate::models::division_model::{self, CreateDivisionInfo};
    // use actix_web::{post, web, HttpResponse, Responder};
    // use deadpool_postgres::Pool;
    // use serde::Deserialize;
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn division_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/division"));
}
