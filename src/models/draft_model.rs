use super::ModelError;
use deadpool_postgres::Client;
use std::time::SystemTime;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "drafts")]
pub struct Draft {
    pub id: i64,
    pub drafter: i64,
    pub division_id: String,
    pub position_index: i16,
    pub name: String,
    pub document_name: String,
    pub document_no: String,
    pub document_hash: String,
    pub updated_at: SystemTime,
}

pub struct CreateDraftInfo {
    pub drafter: i64,
    pub division_id: i64,
    pub position_index: i16,
    pub name: String,
    pub document_name: String,
    pub document_no: String,
    pub document_hash: String,
}

pub async fn create_draft(client: &Client, draft_info: &CreateDraftInfo) -> Result<(), ModelError> {
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

    let _ = client
        .execute(
            &statement,
            &[
                &draft_info.drafter,
                &draft_info.division_id,
                &draft_info.position_index,
                &draft_info.name,
                &draft_info.document_name,
                &draft_info.document_no,
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

    Ok(())
}

fn validate_draft_info(_draft_info: &CreateDraftInfo) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
