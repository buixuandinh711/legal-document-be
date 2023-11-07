mod routes {
    use crate::{
        app_config::AppState,
        middlewares::auth::AuthenticatedOfficer,
        models::{draft_task_model, officier_model::PositionRole, signature_model},
    };
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct CreateDraftTaskBody {
        task_name: String,
        drafter_address: String,
        drafter_position_index: i16,
    }

    #[derive(Deserialize, Debug)]
    struct SubmittedDraftTaskBody {
        draft_id: i64,
        signature: String,
    }

    #[get("/draft-tasks/drafters")]
    async fn get_division_drafters(
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

        if position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position role");
        }

        match draft_task_model::get_division_drafters(
            &client,
            &division_id,
            &address,
            &position_index,
        )
        .await
        {
            Ok(positions) => HttpResponse::Ok().json(positions),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/draft-tasks")]
    async fn create_draft_task(
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
        req_body: web::Json<CreateDraftTaskBody>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();
        let req_body = req_body.into_inner();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        let task_info: draft_task_model::CreateDraftTaskInfo =
            draft_task_model::CreateDraftTaskInfo {
                name: req_body.task_name,
                assigner_address: address.clone(),
                assigner_division_id: division_id.clone(),
                assigner_position_index: position_index.clone(),
                assignee_address: req_body.drafter_address,
                assignee_division_id: division_id.clone(),
                assignee_position_index: req_body.drafter_position_index,
            };

        match draft_task_model::create_draft_task(&client, &task_info).await {
            Ok(task_id) => HttpResponse::Created().json(task_id),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/draft-tasks/created")]
    async fn get_created_draft_tasks(
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

        if position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position role");
        }

        match draft_task_model::get_created_draft_tasks(
            &client,
            &address,
            &division_id,
            &position_index,
        )
        .await
        {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/draft-tasks/assigned")]
    async fn get_assigned_draft_tasks(
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

        if position_role != PositionRole::Manager && position_role != PositionRole::Staff {
            return HttpResponse::Unauthorized().body("Invalid position role");
        }

        match draft_task_model::get_assigned_draft_tasks(
            &client,
            &address,
            &division_id,
            &position_index,
        )
        .await
        {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("draft-tasks/assigned/{id}")]
    async fn get_assigned_draft_task_detail(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
    ) -> impl Responder {
        let task_id = path.into_inner();
        let client = app_state.db_pool.get().await.unwrap();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if position_role != PositionRole::Manager && position_role != PositionRole::Staff {
            return HttpResponse::Unauthorized().body("Invalid position role");
        }

        let task_detail = draft_task_model::get_draft_task_detail(&client, &task_id).await;
        if task_detail.is_err() {
            return HttpResponse::InternalServerError().body("Failed to query task detail");
        }
        let task_detail = task_detail.unwrap();

        if !(task_detail.assignee_address == address
            && task_detail.assignee_division_id == division_id
            && task_detail.assignee_position_index == position_index)
        {
            return HttpResponse::Unauthorized().body("Not the task's assignee");
        }

        match draft_task_model::get_assigned_draft_task_detail(&client, &task_id).await {
            Ok(task) => HttpResponse::Ok().json(task),
            Err(_) => {
                HttpResponse::InternalServerError().body("Failed to query assigned task detail")
            }
        }
    }

    #[post("/draft-tasks/submit/{id}")]
    async fn submit_draft_task(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
        req_body: web::Json<SubmittedDraftTaskBody>,
    ) -> impl Responder {
        log::info!("Called fasdfsdfs");
        let client = app_state.db_pool.get().await.unwrap();
        let task_id = path.into_inner();

        let task_detail = draft_task_model::get_draft_task_detail(&client, &task_id).await;
        if task_detail.is_err() {
            return HttpResponse::InternalServerError().body("Failed to get task detail");
        }
        let task_detail = task_detail.unwrap();

        let AuthenticatedOfficer {
            address,
            division_id,
            position_index,
            position_role,
        } = authenticated_officer;

        if !(task_detail.assignee_address == address
            && task_detail.assignee_division_id == division_id
            && task_detail.assignee_position_index == position_index)
        {
            return HttpResponse::Unauthorized().body("Not the task assignee");
        }

        if position_role != PositionRole::Manager && position_role != PositionRole::Staff {
            return HttpResponse::Unauthorized().body("Invalid position role");
        }

        let req_body = req_body.into_inner();

        if let Err(_) = signature_model::create_draft_signature(
            &client,
            &req_body.draft_id,
            &address,
            &division_id,
            &position_index,
            &req_body.signature,
        )
        .await
        {
            return HttpResponse::InternalServerError().body("Failed to create draft signature");
        }

        match draft_task_model::update_draft_task_submitted(&client, &task_id, &req_body.draft_id)
            .await
        {
            Ok(_) => HttpResponse::Created().body("Draft task draft submitted"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

use actix_web::web;
use routes::*;

pub fn draft_task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_division_drafters)
        .service(create_draft_task)
        .service(get_created_draft_tasks)
        .service(get_assigned_draft_tasks)
        .service(get_assigned_draft_task_detail)
        .service(submit_draft_task);
}
