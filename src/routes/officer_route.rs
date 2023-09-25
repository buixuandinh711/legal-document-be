mod routes {
    use actix_web::{post, web, HttpResponse, Responder};
    use deadpool_postgres::Pool;
    use serde::Deserialize;

    use crate::models::{
        officier_model::{self, CreateOfficerInfo},
        ModelError,
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

    #[post("/create")]
    async fn create_officer(
        db_pool: web::Data<Pool>,
        req_body: web::Json<CreateOfficerBody>,
    ) -> impl Responder {
        let client = db_pool.get().await.unwrap();
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
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/officer").service(create_officer));
}
