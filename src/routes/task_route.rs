mod routes {
    use crate::{
        app_config::AppState,
        models::{officier_model::PositionRole, task_model},
        routes::verify_and_get_officer,
    };
    use actix_identity::Identity;
    use actix_web::{post, web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct ReqSignerPosition {
        signer_address: String,
        position_index: i16,
    }

    #[derive(Deserialize, Debug)]
    struct CreateReviewTaskBody {
        draft_id: i64,
        asssigner_div_id: String,
        assigner_pos_index: i16,
        assignees: Vec<ReqSignerPosition>,
    }

    #[post("/create")]
    async fn create_review_task(
        identity: Option<Identity>,
        app_state: web::Data<AppState>,
        req_body: web::Json<CreateReviewTaskBody>,
    ) -> impl Responder {
        let client = app_state.db_pool.get().await.unwrap();
        let req_body = req_body.into_inner();

        let verify_result = verify_and_get_officer(
            &client,
            &identity,
            &req_body.asssigner_div_id,
            req_body.assigner_pos_index,
        )
        .await;
        if let Err(response) = verify_result {
            return response;
        }
        let (_officer_id, position_role) = verify_result.unwrap();

        if position_role != PositionRole::Manager {
            return HttpResponse::Unauthorized().body("Invalid position");
        }

        let assigner_address = "".to_owned();

        let tasks_info: Vec<_> = req_body
            .assignees
            .into_iter()
            .map(|assignee| task_model::CreateReviewTaskInfo {
                draft_id: req_body.draft_id,
                assigner_address: assigner_address.clone(),
                assigner_division_id: req_body.asssigner_div_id.clone(),
                assigner_position_index: req_body.assigner_pos_index,
                assignee_address: assignee.signer_address,
                assignee_division_id: req_body.asssigner_div_id.clone(),
                assignee_position_index: assignee.position_index,
            })
            .collect();

        match task_model::create_review_task(&client, &tasks_info).await {
            Ok(_) => HttpResponse::Created().body("Review task created"),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

use actix_web::web;
use routes::*;

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/review").service(create_review_task));
}
