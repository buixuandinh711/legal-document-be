use super::ModelError;
use deadpool_postgres::Client;
use serde::Serialize;
use std::time::SystemTime;

pub struct CreateDraftInfo {
    pub drafter_address: String,
    pub division_onchain_id: String,
    pub position_index: i16,
    pub name: String,
    pub document_no: String,
    pub document_name: String,
    pub document_type: i32,
    pub document_hash: String,
    pub file_name: String,
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

#[derive(Serialize)]
pub struct DraftDetail {
    pub id: i64,
    pub name: String,
    pub document_no: String,
    pub document_name: String,
    pub document_type: String,
    pub file_name: String,
    pub updated_at: SystemTime,
    pub doc_uri: String,
    pub drafter_username: String,
    pub drafter_name: String,
}

#[derive(Serialize)]
pub struct PublishableDraft {
    pub id: i64,
    pub name: String,
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
                &draft_info.drafter_address,
                &draft_info.division_onchain_id,
                &draft_info.position_index,
                &draft_info.name,
                &draft_info.document_no,
                &draft_info.document_name,
                &draft_info.document_type,
                &draft_info.document_hash,
                &draft_info.file_name,
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
    officer_address: &str,
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
            &[&officer_address, &division_onchain_id, &position_index],
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

pub async fn get_draft_detail(client: &Client, draft_id: i64) -> Result<DraftDetail, ModelError> {
    let statement = include_str!("../sql/drafts/query_draft_detail.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_draft_detail",
            &err,
        )
    })?;

    let query_result = client
        .query(&statement, &[&draft_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_draft_detail",
                &err,
            )
        })?;

    if query_result.is_empty() {
        return Err(ModelError::new(
            ModelError::NotFoundError,
            "get_draft_detail: query draft by id",
            &draft_id,
        ));
    }

    let query_result = &query_result[0];

    let drafts_detail = DraftDetail {
        id: query_result.get(0),
        name: query_result.get(1),
        document_no: query_result.get(2),
        document_name: query_result.get(3),
        document_type: query_result.get(4),
        file_name: query_result.get(5),
        updated_at: query_result.get(6),
        doc_uri: query_result.get(7),
        drafter_username: query_result.get(8),
        drafter_name: query_result.get(9),
    };

    Ok(drafts_detail)
}

pub async fn get_publishable_drafts(
    client: &Client,
    officer_address: &str,
    division_onchain_id: &str,
    position_index: i16,
) -> Result<Vec<PublishableDraft>, ModelError> {
    let statement = include_str!("../sql/drafts/query_publishable_draft.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_publishable_draft",
            &err,
        )
    })?;

    let query_result = client
        .query(
            &statement,
            &[&officer_address, &division_onchain_id, &position_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_publishable_draft",
                &err,
            )
        })?;

    let drafts_list: Vec<PublishableDraft> = query_result
        .iter()
        .map(|row| PublishableDraft {
            id: row.get(0),
            name: row.get(1),
        })
        .collect();

    Ok(drafts_list)
}

fn validate_draft_info(_draft_info: &CreateDraftInfo) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
