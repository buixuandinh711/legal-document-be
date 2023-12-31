mod routes {
    use crate::{
        app_config::AppState,
        middlewares::auth::AuthenticatedOfficer,
        models::{
            document_model, document_type_model,
            draft_model::{self, CreateDraftInfo},
            officier_model::PositionRole,
            signature_model, ModelError,
        },
    };
    use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqCreateDraftInfo {
        name: String,
        document_no: String,
        document_name: String,
        document_type: i32,
    }

    #[derive(Debug, MultipartForm)]
    struct CreateDraftForm {
        doc: TempFile,
        info: Json<ReqCreateDraftInfo>,
    }

    #[post("/drafts")]
    async fn create_draft(
        app_state: web::Data<AppState>,
        MultipartForm(mut form): MultipartForm<CreateDraftForm>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();
        let draft_info = form.info.into_inner();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        let cloud_storage = &app_state.cloud_storage;
        let file = form.doc.file.as_file_mut();
        let doc_hash = document_model::create_document(&client, cloud_storage, file).await;
        if doc_hash.is_err() {
            return HttpResponse::InternalServerError().body("Failed to upload document");
        }
        let doc_hash = doc_hash.unwrap();
        let file_name = match form.doc.file_name {
            Some(name) => name,
            None => "draft.pdf".to_owned(),
        };

        let draft_info = CreateDraftInfo {
            drafter_address: address,
            division_onchain_id: division_id,
            position_index: position_index,
            name: draft_info.name,
            document_no: draft_info.document_no,
            document_name: draft_info.document_name,
            document_type: draft_info.document_type,
            document_hash: doc_hash,
            file_name: file_name,
        };

        match draft_model::create_draft(&client, &draft_info).await {
            Ok(draft_id) => HttpResponse::Created().json(draft_id),
            Err(err) => match err {
                ModelError::ValidationError => HttpResponse::BadRequest().body("Invild draft info"),
                _ => HttpResponse::InternalServerError().body("Failed to save draft"),
            },
        }
    }

    #[get("/drafts")]
    async fn get_drafts(
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match draft_model::get_draft_list(&client, &address, &division_id, position_index).await {
            Ok(drafts_list) => HttpResponse::Ok().json(drafts_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/drafts/{draft_id}")]
    async fn get_draft_detail(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let draft_id = path.into_inner();
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

        match draft_model::get_draft_detail(&client, draft_id).await {
            Ok(draft) => HttpResponse::Ok().json(draft),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/publishable-drafts")]
    async fn get_publishable_drafts(
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match draft_model::get_publishable_drafts(&client, &address, &division_id, position_index)
            .await
        {
            Ok(drafts_list) => HttpResponse::Ok().json(drafts_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/drafts/{draft_id}/signatures")]
    async fn get_draft_signatures(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let draft_id = path.into_inner();
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

        match signature_model::get_draft_signatures(&client, draft_id).await {
            Ok(sigs) => HttpResponse::Ok().json(sigs),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/drafts/{draft_id}/not-signed")]
    async fn get_singers_not_signed(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let draft_id = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address: _,
            division_id: _,
            position_index: _,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match signature_model::get_signer_not_signed(&client, draft_id).await {
            Ok(positions) => HttpResponse::Ok().json(positions),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/doc-types")]
    async fn get_doc_types(app_state: web::Data<AppState>) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();

        match document_type_model::get_document_types(&client).await {
            Ok(doc_types) => HttpResponse::Ok().json(doc_types),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

use actix_web::web;
use routes::*;

pub fn draft_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_draft)
        .service(get_drafts)
        .service(get_draft_detail)
        .service(get_publishable_drafts)
        .service(get_draft_signatures)
        .service(get_singers_not_signed)
        .service(get_doc_types);
}
