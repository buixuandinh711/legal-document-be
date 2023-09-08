mod routes {

    use actix_identity::Identity;
    use actix_web::{
        http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct LoginInfo {
        username: String,
        password: String,
    }

    #[post("/login")]
    async fn login(req: HttpRequest, info: web::Json<LoginInfo>) -> impl Responder {
        if info.username.eq("dinhbx") && info.password.eq("dinhbx123") {
            Identity::login(&req.extensions(), info.username.clone()).unwrap();
            return HttpResponse::Ok().body("Login successfully");
        }
        HttpResponse::Unauthorized().body("Login failed")
    }

    #[post("/logout")]
    async fn logout(id: Identity) -> impl Responder {
        id.logout();

        web::Redirect::to("/").using_status_code(StatusCode::FOUND)
    }
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout);
}
