mod routes {
    use crate::{
        app_config::AppState,
        middlewares::auth::AuthenticatedOfficer,
        models::{officier_model::PositionRole, published_doc_model, signature_model},
    };
    use actix_web::{get, web, HttpResponse, Responder};

    #[get("/publisheds")]
    async fn get_docs(
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address: _,
            division_id,
            position_index: _,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match published_doc_model::get_published_docs(&client, &division_id).await {
            Ok(docs_list) => HttpResponse::Ok().json(docs_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/publisheds/{doc_content_hash}")]
    async fn get_doc_detail(
        path: web::Path<String>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let doc_content_hash = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address: _,
            division_id: _,
            position_index: _,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match published_doc_model::get_published_detail(&client, &doc_content_hash).await {
            Ok(draft) => HttpResponse::Ok().json(draft),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/signatures/{draft_id}")]
    async fn get_doc_signatures(
        path: web::Path<String>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let doc_content_hash = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address: _,
            division_id: _,
            position_index: _,
            position_role,
        } = authenticated_officer;

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
    cfg.service(get_docs)
        .service(get_doc_detail)
        .service(web::scope("/publisheds").service(get_doc_signatures));
}
