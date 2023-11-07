mod routes {
    use crate::{
        app_config::AppState,
        middlewares::auth::AuthenticatedOfficer,
        models::{officier_model::PositionRole, review_task_model, signature_model},
    };
    use actix_web::{get, post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqSignerPosition {
        signer_address: String,
        position_index: i16,
    }

    #[derive(Deserialize, Debug)]
    struct CreateReviewTaskBody {
        draft_id: i64,
        assignees: Vec<ReqSignerPosition>,
    }

    #[derive(Deserialize, Debug)]
    struct SignReviewDraftBody {
        signature: String,
    }

    #[post("/review-tasks")]
    async fn create_review_task(
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
        req_body: web::Json<CreateReviewTaskBody>,
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

        let tasks_info: Vec<_> = req_body
            .assignees
            .into_iter()
            .map(|assignee| review_task_model::CreateReviewTaskInfo {
                draft_id: req_body.draft_id,
                assigner_address: address.clone(),
                assigner_division_id: division_id.clone(),
                assigner_position_index: position_index.clone(),
                assignee_address: assignee.signer_address,
                assignee_division_id: division_id.clone(),
                assignee_position_index: assignee.position_index,
            })
            .collect();

        match review_task_model::create_review_task(&client, &tasks_info).await {
            Ok(_) => HttpResponse::Created().body("Review task created"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/created-review-tasks")]
    async fn get_created_review_tasks(
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
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match review_task_model::get_created_review_tasks(
            &client,
            &address,
            &division_id,
            position_index,
        )
        .await
        {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/assigned-review-tasks")]
    async fn get_assigned_review_tasks(
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
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match review_task_model::get_assigned_review_tasks(
            &client,
            &address,
            &division_id,
            position_index,
        )
        .await
        {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[get("/assigned-review-tasks/{id}")]
    async fn get_assigned_review_task_detail(
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
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        match review_task_model::get_assigned_review_task_detail(
            &client,
            &address,
            &division_id,
            position_index,
            task_id,
        )
        .await
        {
            Ok(task) => HttpResponse::Ok().json(task),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    #[post("/review-tasks/sign/{id}")]
    async fn sign_reviewed_draft(
        path: web::Path<i64>,
        app_state: web::Data<AppState>,
        authenticated_officer: AuthenticatedOfficer,
        req_body: web::Json<SignReviewDraftBody>,
    ) -> impl Responder {
        log::info!("Called fasdfsdfs");
        let client = app_state.db_pool.get().await.unwrap();
        let task_id = path.into_inner();

        let task_detail = review_task_model::get_review_task_detail(&client, &task_id).await;
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
            &task_detail.draft_id,
            &address,
            &division_id,
            &position_index,
            &req_body.signature,
        )
        .await
        {
            return HttpResponse::InternalServerError().finish();
        }

        match review_task_model::update_review_task_signed(&client, &task_id).await {
            Ok(_) => HttpResponse::Created().body("Review task draft signed"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

use actix_web::web;
use routes::*;

pub fn review_task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_created_review_tasks)
        .service(get_assigned_review_tasks)
        .service(get_assigned_review_task_detail)
        .service(sign_reviewed_draft)
        .service(create_review_task);
}
