mod routes {
    use actix_web::{get, Responder};

    #[get("/")]
    pub async fn home() -> impl Responder {
        String::from("Hello")
    }
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}
