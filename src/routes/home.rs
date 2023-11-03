mod routes {
    use actix_multipart::form::{tempfile::TempFile, MultipartForm};
    use actix_web::{get, post, web::Data, Error, HttpResponse, Responder};

    use crate::{
        app_config::AppState, middlewares::auth::AuthenticatedOfficer, models::document_model,
    };

    #[derive(Debug, MultipartForm)]
    struct UploadForm {
        #[multipart(rename = "file")]
        file: TempFile,
    }

    #[post("/")]
    async fn upload_file(
        app_state: Data<AppState>,
        MultipartForm(mut form): MultipartForm<UploadForm>,
    ) -> Result<impl Responder, Error> {
        let cloud_storage = &app_state.cloud_storage;
        let client = app_state.db_pool.get().await.unwrap();

        let file = form.file.file.as_file_mut();

        document_model::create_document(&client, cloud_storage, file)
            .await
            .unwrap();

        Ok(HttpResponse::Ok().body("Upload successfully"))
    }

    #[get("/")]
    pub async fn home(officer: AuthenticatedOfficer) -> impl Responder {
        HttpResponse::Ok().json(officer)
    }
}

use actix_web::web::{self};
use routes::*;

// this function could be located in a different module
pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
}
