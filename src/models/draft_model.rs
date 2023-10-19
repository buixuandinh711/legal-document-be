use super::ModelError;
use deadpool_postgres::Client;
use serde::Serialize;
use std::time::SystemTime;

pub struct CreateDraftInfo {
    pub drafter: i64,
    pub division_onchain_id: String,
    pub position_index: i16,
    pub name: String,
    pub document_no: String,
    pub document_name: String,
    pub document_type: i32,
    pub document_hash: String,
}

#[derive(Serialize)]
pub struct DraftsListItem {
    pub id: i64,
    pub name: String,
    pub drafter_username: String,
    pub drafter_name: String,
    pub document_name: String,
    pub updated_at: SystemTime,
}

pub async fn create_draft(
    client: &Client,
    draft_info: &CreateDraftInfo,
) -> Result<i64, ModelError> {
    if validate_draft_info(&draft_info).is_err() {
        return Err(ModelError::new(
            ModelError::ValidationError,
            "Validation: invalid create draft info",
            &"",
        ));
    }

    let statement = include_str!("../sql/drafts/create_draft.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_draft",
            &err,
        )
    })?;

    let result = client
        .query_one(
            &statement,
            &[
                &draft_info.drafter,
                &draft_info.division_onchain_id,
                &draft_info.position_index,
                &draft_info.name,
                &draft_info.document_no,
                &draft_info.document_name,
                &draft_info.document_type,
                &draft_info.document_hash,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_draft",
                &err,
            )
        })?;

    let draft_id: i64 = result.get(0);

    Ok(draft_id)
}

pub async fn get_draft_list(
    client: &Client,
    officer_id: i64,
    division_onchain_id: &str,
    position_index: i16,
) -> Result<Vec<DraftsListItem>, ModelError> {
    let statement = include_str!("../sql/drafts/query_draft_list.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_draft_list",
            &err,
        )
    })?;

    let query_result = client
        .query(
            &statement,
            &[&officer_id, &division_onchain_id, &position_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_draft_list",
                &err,
            )
        })?;

    let drafts_list: Vec<DraftsListItem> = query_result
        .iter()
        .map(|row| DraftsListItem {
            id: row.get(0),
            drafter_username: row.get(1),
            drafter_name: row.get(2),
            name: row.get(3),
            document_name: row.get(4),
            updated_at: row.get(5),
        })
        .collect();

    Ok(drafts_list)
}

fn validate_draft_info(_draft_info: &CreateDraftInfo) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
