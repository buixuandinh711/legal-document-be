mod routes {
    use crate::{
        app_config::AppState,
        models::{
            document_model, document_type_model,
            draft_model::{self, CreateDraftInfo},
            officier_model::PositionRole,
            signature_model, ModelError,
        },
        routes::{verify_and_get_officer, ReqPosition},
    };
    use actix_identity::Identity;
    use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqCreateDraftInfo {
        division_onchain_id: String,
        position_index: i16,
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

    #[post("/create")]
    async fn create_draft(
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        MultipartForm(mut form): MultipartForm<CreateDraftForm>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();
        let draft_info = form.info.into_inner();

        let verify_result = verify_and_get_officer(
            &client,
            &identity,
            &draft_info.division_onchain_id,
            draft_info.position_index,
        )
        .await;
        if let Err(response) = verify_result {
            return response;
        }
        let (officer_id, position_role) = verify_result.unwrap();

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
            drafter: officer_id,
            division_onchain_id: draft_info.division_onchain_id,
            position_index: draft_info.position_index,
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

    #[post("/list")]
    async fn get_drafts(
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
        let (officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match draft_model::get_draft_list(
            &client,
            officer_id,
            &req_body.division_onchain_id,
            req_body.position_index,
        )
        .await
        {
            Ok(drafts_list) => HttpResponse::Ok().json(drafts_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/detail/{draft_id}")]
    async fn get_draft_detail(
        path: web::Path<i64>,
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqPosition>,
    ) -> impl Responder {
        let draft_id = path.into_inner();
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

        match draft_model::get_draft_detail(&client, draft_id).await {
            Ok(draft) => HttpResponse::Ok().json(draft),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/submittable")]
    async fn get_submittable_drafts(
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
        let (officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Staff && position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match draft_model::get_submittable_drafts(
            &client,
            officer_id,
            &req_body.division_onchain_id,
            req_body.position_index,
        )
        .await
        {
            Ok(drafts_list) => HttpResponse::Ok().json(drafts_list),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/signatures/{draft_id}")]
    async fn get_draft_signatures(
        path: web::Path<i64>,
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<ReqPosition>,
    ) -> impl Responder {
        let draft_id = path.into_inner();
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

        match signature_model::get_draft_signatures(&client, draft_id).await {
            Ok(sigs) => HttpResponse::Ok().json(sigs),
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
    cfg.service(
        web::scope("/draft")
            .service(create_draft)
            .service(get_drafts)
            .service(get_draft_detail)
            .service(get_submittable_drafts)
            .service(get_draft_signatures)
    )
    .service(get_doc_types)
    .service(get_draft_detail);
}
