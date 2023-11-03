use actix_identity::Identity;
use actix_web::HttpResponse;
use deadpool_postgres::Client;
use serde::Deserialize;

use crate::models::officier_model::{self, PositionRole};

pub mod draft_route;
pub mod home;
pub mod officer_route;
pub mod published_doc_route;
pub mod task_route;

#[derive(Deserialize)]
pub struct ReqPosition {
    division_onchain_id: String,
    position_index: i16,
}

pub async fn verify_and_get_officer(
    client: &Client,
    identity: &Option<Identity>,
    division_onchain_id: &str,
    position_index: i16,
) -> Result<(i64, PositionRole), HttpResponse> {
    if identity.is_none() {
        return Err(HttpResponse::Unauthorized().body("Identity not exist"));
    }
    let officer_id = identity.as_ref().unwrap().id();
    if officer_id.is_err() {
        return Err(HttpResponse::InternalServerError().body("Unable to get identity info"));
    }
    let officer_id: i64 = officer_id.unwrap().parse().unwrap();

    let position_role = officier_model::validate_and_get_role(
        &client,
        "officer_id",
        division_onchain_id,
        position_index,
    )
    .await;
    if position_role.is_err() {
        return Err(HttpResponse::NotFound().body("Position not found"));
    }

    Ok((officer_id, position_role.unwrap()))
}
