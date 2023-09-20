mod routes {
    use actix_identity::Identity;
    use actix_web::{
        http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
    };
    use deadpool_postgres::Pool;
    use serde::Deserialize;

    use crate::models::official::{self, CreateOfficalInfo, OfficialModelError};

    #[derive(Deserialize, Debug)]
    struct ReqLoginBody {
        username: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct ReqRegisterBody {
        pub address: String,
        pub name: String,
        pub date_of_birth: String,
        pub sex: String,
        pub password: String,
        pub private_key: String,
    }

    impl From<ReqRegisterBody> for CreateOfficalInfo {
        fn from(value: ReqRegisterBody) -> Self {
            CreateOfficalInfo {
                address: value.address,
                name: value.name,
                date_of_birth: value.date_of_birth,
                sex: value.sex,
                password: value.password,
                private_key: value.private_key,
            }
        }
    }

    #[post("/login")]
    async fn login(
        req: HttpRequest,
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqLoginBody>,
    ) -> impl Responder {
        "To do".to_owned()
    }

    #[post("/logout")]
    async fn logout(id: Identity) -> impl Responder {
        "To do".to_owned()
    }

    #[post("/register")]
    async fn register(
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqRegisterBody>,
    ) -> impl Responder {
        let client = db_pool.get().await.unwrap();
        let official_info = CreateOfficalInfo::from(req_body.into_inner());

        match official::create_offcial(&client, &official_info).await {
            Ok(_) => HttpResponse::Created().body("Register successfully"),
            Err(err) => match err {
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
