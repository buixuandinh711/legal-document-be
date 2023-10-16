mod routes {
    use actix_identity::Identity;
    use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
    use serde::Deserialize;

    use crate::{
        app_config::AppState,
        models::{
            officier_model::{self, AuthOfficerInfo, CreateOfficerInfo},
            ModelError,
        },
    };

    #[derive(Deserialize, Debug)]
    struct CreateOfficerBody {
        pub username: String,
        pub password: String,
        pub onchain_address: String,
        pub private_key: String,
    }

    impl From<CreateOfficerBody> for CreateOfficerInfo {
        fn from(body: CreateOfficerBody) -> CreateOfficerInfo {
            CreateOfficerInfo {
                username: body.username,
                password: body.password,
                onchain_address: body.onchain_address,
                private_key: body.private_key,
            }
        }
    }

    #[derive(Deserialize, Debug)]
    struct ReqLoginBody {
        username: String,
        password: String,
    }

    impl From<ReqLoginBody> for AuthOfficerInfo {
        fn from(value: ReqLoginBody) -> Self {
            AuthOfficerInfo {
                username: value.username,
                password: value.password,
            }
        }
    }

    #[post("/create")]
    async fn create_officer(
        app_state: web::Data<AppState>,
        req_body: web::Json<CreateOfficerBody>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();
        let officer_info = req_body.into_inner();
        let officer_info = CreateOfficerInfo::from(officer_info);

        match officier_model::create_officer(&client, &officer_info).await {
            Ok(_) => HttpResponse::Created().body("Officer created"),
            Err(err) => match err {
                ModelError::ValidationError => {
                    HttpResponse::BadRequest().body("Invalid officer info".to_owned())
                }
                _ => HttpResponse::InternalServerError().body("Internal server error".to_owned()),
            },
        }
    }

    #[post("/login")]
    async fn login(
        req: HttpRequest,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqLoginBody>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        let req_body = req_body.into_inner();

        let auth_info = AuthOfficerInfo::from(req_body);

        match officier_model::authenticate_officer(&client, &auth_info).await {
            Ok(auth_result) => {
                if let Some((officer_id, officer_info)) = auth_result {
                    match Identity::login(&req.extensions(), officer_id.to_string()) {
                        Ok(_) => HttpResponse::Ok().json(officer_info),
                        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
                    }
                } else {
                    HttpResponse::Unauthorized().body("Invalid password")
                }
            }
            Err(err) => match err {
                ModelError::AuthError => HttpResponse::Unauthorized().body(""),
                ModelError::NotFoundError => HttpResponse::NotFound().body(""),
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            },
        }
    }

    #[post("/login-cookie")]
    async fn login_cookie(
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
    ) -> impl Responder {
        match identity {
            Some(identity) => match identity.id() {
                Ok(officer_id) => {
                    let client = app_state.db_pool.get().await.unwrap();
                    let officer_id: i64 = officer_id.parse().unwrap();
                    let officer_info =
                        officier_model::validate_and_get_info(&client, officer_id).await;
                    match officer_info {
                        Ok(officer_info) => HttpResponse::Ok().json(officer_info),
                        Err(err) => match err {
                            ModelError::AuthError => HttpResponse::Unauthorized().body(""),
                            ModelError::NotFoundError => HttpResponse::NotFound().body(""),
                            _ => HttpResponse::InternalServerError().body("Internal server error"),
                        },
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Internal server error");
                }
            },
            None => return HttpResponse::Unauthorized().body("Invalid auth cookie"),
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
}

use actix_web::web;
use routes::*;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/officer").service(create_officer))
        .service(login)
        .service(login_cookie)
        .service(logout);
}
