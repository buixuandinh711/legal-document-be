use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ReqLoginBody {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct ReqRegisterBody {
    pub username: String,
    pub password: String,
}

impl error::ResponseError for UserModelError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

mod routes {
    use crate::{
        models::user::{create_user, CreateUserInfo},
        routes::auth::{ReqLoginBody, ReqRegisterBody},
    };
    use actix_identity::Identity;
    use actix_web::{
        http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use deadpool_postgres::Pool;

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
    pub async fn register(
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqRegisterBody>,
    ) -> actix_web::Result<impl Responder> {
        let client = db_pool.get().await.unwrap();

        let user_info = CreateUserInfo::from(req_body.into_inner());

        let user_id = create_user(&client, &user_info).await.unwrap();

        Ok(format!("User created {user_id}"))
    }
}

use actix_web::{error, http::StatusCode, web, HttpResponse};
use routes::*;

use crate::models::user::UserModelError;

// this function could be located in a different module
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout).service(register);
}
