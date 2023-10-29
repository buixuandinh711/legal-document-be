mod routes {
    use crate::{
        app_config::AppState,
        models::{officier_model::PositionRole, published_doc_model, signature_model},
        routes::{verify_and_get_officer, ReqPosition},
    };
    use actix_identity::Identity;
    use actix_web::{post, web, HttpResponse, Responder};

    #[post("/list")]
    async fn get_docs(
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqPosition>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        let verify_result = verify_and_get_officer(
            &client,
            &identity,
            &req_body.division_onchain_id,
            req_body.position_index,
        )
        .await;
        if let Err(response) = verify_result {
            return response;
        }
        let (_officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match published_doc_model::get_published_docs(&client, &req_body.division_onchain_id).await
        {
            Ok(docs_list) => HttpResponse::Ok().json(docs_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/detail/{doc_content_hash}")]
    async fn get_doc_detail(
        path: web::Path<String>,
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqPosition>,
    ) -> impl Responder {
        let doc_content_hash = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let verify_result = verify_and_get_officer(
            &client,
            &identity,
            &req_body.division_onchain_id,
            req_body.position_index,
        )
        .await;
        if let Err(response) = verify_result {
            return response;
        }
        let (_officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match published_doc_model::get_published_detail(&client, &doc_content_hash).await {
            Ok(draft) => HttpResponse::Ok().json(draft),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/signatures/{draft_id}")]
    async fn get_doc_signatures(
        path: web::Path<String>,
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqPosition>,
    ) -> impl Responder {
        let doc_content_hash = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let verify_result = verify_and_get_officer(
            &client,
            &identity,
            &req_body.division_onchain_id,
            req_body.position_index,
        )
        .await;
        if let Err(response) = verify_result {
            return response;
        }
        let (_officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match signature_model::get_doc_signatures(&client, &doc_content_hash).await {
            Ok(sigs) => HttpResponse::Ok().json(sigs),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

use actix_web::web;
use routes::*;

pub fn published_doc_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/published")
            .service(get_docs)
            .service(get_doc_detail)
            .service(get_doc_signatures),
    );
}
