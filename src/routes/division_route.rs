mod routes {
    use actix_identity::Identity;
    use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
    use deadpool_postgres::Pool;
    use serde::Deserialize;

    use crate::models::division_model::{self, CreateDivisionInfo};

    #[derive(Deserialize, Debug)]
    struct ReqCreateBody {
        pub onchain_id: String,
        pub name: String,
        pub supervisory_id: i64,
    }

    impl From<&ReqCreateBody> for CreateDivisionInfo {
        fn from(value: &ReqCreateBody) -> Self {
            CreateDivisionInfo {
                onchain_id: value.onchain_id.clone(),
                name: value.name.clone(),
                supervisory_id: value.supervisory_id.clone(),
            }
        }
    }

    #[post("/create")]
    async fn create(
        db_pool: web::Data<Pool>,
        req_body: web::Json<ReqCreateBody>,
    ) -> impl Responder {
        let client = db_pool.get().await.unwrap();

        let req_body = req_body.into_inner();
        let division_info = CreateDivisionInfo::from(&req_body);

        let create_result = division_model::create_division(&client, &division_info).await;

        match create_result {
            Ok(_) => HttpResponse::Created().body("Create division successfully"),
            Err(err) => match err {
                _ => {
                    println!("{err}");
                    HttpResponse::InternalServerError().body("Internal server error".to_owned())
                },
            },
        }
    }
}

use actix_web::web;
use routes::*;

// this function could be located in a different module
pub fn division_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/division").service(create));
}
