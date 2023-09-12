mod routes {
    use crate::models::user::{create_user, CreateUserInfo, UserModelError};
    use actix_identity::Identity;
    use actix_web::{
        http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use deadpool_postgres::Pool;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqLoginBody {
        username: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct ReqRegisterBody {
        username: String,
        password: String,
    }

    impl From<ReqRegisterBody> for CreateUserInfo {
        fn from(value: ReqRegisterBody) -> Self {
            CreateUserInfo {
                username: value.username,
                raw_password: value.password,
            }
        }
    }

    #[post("/login")]
    async fn login(req: HttpRequest, info: web::Json<ReqLoginBody>) -> impl Responder {
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

    #[post("/register")]
    async fn register(
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqRegisterBody>,
    ) -> impl Responder {
        let client = db_pool.get().await.unwrap();

        let user_info = CreateUserInfo::from(req_body.into_inner());

        match create_user(&client, &user_info).await {
            Ok(id) => HttpResponse::Created().body(format!("User created with id {}", id)),
            Err(err) => match err {
                UserModelError::ValidationError { msg } => HttpResponse::BadRequest().body(msg),
                UserModelError::DBPoolError { .. } => {
                    HttpResponse::InternalServerError().body("Internal server error".to_owned())
                }
                _ => HttpResponse::InternalServerError().body("Internal server error".to_owned()),
            },
        }
    }
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(register);
}
