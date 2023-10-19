mod routes {
    use crate::{
        app_config::AppState,
        models::{
            document_model, document_type_model,
            draft_model::{self, CreateDraftInfo},
            officier_model::{self, PositionRole},
            ModelError,
        },
    };
    use actix_identity::Identity;
    use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqCreateDraftInfo {
        division_id: i64,
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
        if identity.is_none() {
            return HttpResponse::Unauthorized().finish();
        }
        let officer_id = identity.unwrap().id();
        if officer_id.is_err() {
            return HttpResponse::InternalServerError().body("Unable to get identity info");
        }
        let officer_id: i64 = officer_id.unwrap().parse().unwrap();
        let client = app_state.db_pool.get().await.unwrap();
        let draft_info = form.info.into_inner();

        let position_role = officier_model::validate_and_get_role(
            &client,
            officer_id,
            draft_info.division_id,
            draft_info.position_index,
        )
        .await;
        if position_role.is_err() {
            return HttpResponse::NotFound().body("Position not found");
        }

        if let PositionRole::Staff | PositionRole::Manager = position_role.unwrap() {
            let cloud_storage = &app_state.cloud_storage;
            let file = form.doc.file.as_file_mut();
            let doc_hash = document_model::create_document(&client, cloud_storage, file).await;
            if doc_hash.is_err() {
                return HttpResponse::InternalServerError().body("Failed to upload document");
            }
            let doc_hash = doc_hash.unwrap();

            let draft_info = CreateDraftInfo {
                drafter: officer_id,
                division_id: draft_info.division_id,
                position_index: draft_info.position_index,
                name: draft_info.name,
                document_no: draft_info.document_no,
                document_name: draft_info.document_name,
                document_type: draft_info.document_type,
                document_hash: doc_hash,
            };

            match draft_model::create_draft(&client, &draft_info).await {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(err) => match err {
                    ModelError::ValidationError => {
                        HttpResponse::BadRequest().body("Invild draft info")
                    }
                    _ => HttpResponse::InternalServerError().body("Failed to save draft"),
                },
            }
        } else {
            HttpResponse::Unauthorized().body("Invalid position")
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
    cfg.service(web::scope("/draft").service(create_draft))
        .service(get_doc_types);
}
