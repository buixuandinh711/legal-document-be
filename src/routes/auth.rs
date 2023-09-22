mod routes {
    use actix_identity::Identity;
    use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
    use deadpool_postgres::Pool;
    use serde::Deserialize;

    use crate::models::official::{
        self, CreateOfficalInfo, OfficialModelError, VerifyOfficialInfo,
    };

    #[derive(Deserialize, Debug)]
    struct ReqLoginBody {
        username: String,
        password: String,
    }

    impl From<&ReqLoginBody> for VerifyOfficialInfo {
        fn from(value: &ReqLoginBody) -> Self {
            VerifyOfficialInfo {
                username: value.username.clone(),
                password: value.password.clone(),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    struct ReqRegisterBody {
        pub username: String,
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
                username: value.username,
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
        identity: Option<Identity>,
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqLoginBody>,
    ) -> impl Responder {
        if let Some(identity) = identity {
            match identity.id() {
                Ok(official_id) => {
                    return HttpResponse::Ok()
                        .body(format!("Already logged in, hello {}", official_id));
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Internal server error");
                }
            }
        }

        let client = db_pool.get().await.unwrap();

        let req_body = req_body.into_inner();

        let verify_info = VerifyOfficialInfo::from(&req_body);

        match official::verify_official(&client, verify_info).await {
            Ok(is_verified) => {
                if is_verified {
                    Identity::login(&req.extensions(), req_body.username.clone()).unwrap();
                    HttpResponse::Ok().body("Login successfully")
                } else {
                    HttpResponse::Unauthorized().body("Invalid password")
                }
            }
            Err(err) => match err {
                OfficialModelError::OfficialNotFound => {
                    HttpResponse::NotFound().body("Official not found")
                }
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            },
        }
    }

    #[post("/logout")]
    async fn logout(identity: Option<Identity>) -> impl Responder {
        match identity {
            Some(identity) => {
                identity.logout();
                HttpResponse::Ok().body("Log out successfully")
            }
            None => HttpResponse::Unauthorized().body("Unauthorized account"),
        }
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
