use deadpool_postgres::Client;
use serde::Serialize;

use super::ModelError;

#[derive(Serialize)]
struct Position {
    position_index: i16,
    position_name: String,
}

#[derive(Serialize)]
pub struct SignerPosition {
    signer_address: String,
    signer_name: String,
    positions: Vec<Position>,
}

pub struct CreateDraftTaskInfo {
    pub name: String,
    pub assigner_address: String,
    pub assigner_division_id: String,
    pub assigner_position_index: i16,
    pub assignee_address: String,
    pub assignee_division_id: String,
    pub assignee_position_index: i16,
}

pub async fn get_division_drafters(
    client: &Client,
    division_id: &str,
    officer_address: &str,
    position_index: &i16,
) -> Result<Vec<SignerPosition>, ModelError> {
    let query_stmt = include_str!("../sql/draft_tasks/query_division_drafter.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_division_drafter",
            &err,
        )
    })?;
    let query_result = client
        .query(
            &query_stmt,
            &[&division_id, &officer_address, &position_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_division_drafter",
                &err,
            )
        })?;

    let mut signers: Vec<SignerPosition> = Vec::new();

    for row in query_result {
        let signer_address: String = row.get(0);
        if let Some(signer) = signers
            .iter_mut()
            .find(|signer| signer.signer_address == signer_address)
        {
            signer.positions.push(Position {
                position_index: row.get(2),
                position_name: row.get(3),
            });
        } else {
            signers.push(SignerPosition {
                signer_address,
                signer_name: row.get(1),
                positions: vec![Position {
                    position_index: row.get(2),
                    position_name: row.get(3),
                }],
            });
        }
    }

    Ok(signers)
}

pub async fn create_draft_task(
    client: &Client,
    tasks_info: &CreateDraftTaskInfo,
) -> Result<(), ModelError> {
    if validate_task_info(&tasks_info).is_err() {
        return Err(ModelError::new(
            ModelError::ValidationError,
            "Validation: invalid create task info",
            &"",
        ));
    }

    let statement = include_str!("../sql/draft_tasks/create_draft_task.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_draft_task",
            &err,
        )
    })?;

    let _result = client
        .execute(
            &statement,
            &[
                &tasks_info.name,
                &tasks_info.assigner_address,
                &tasks_info.assigner_division_id,
                &tasks_info.assigner_position_index,
                &tasks_info.assignee_address,
                &tasks_info.assignee_division_id,
                &tasks_info.assignee_position_index,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_draft_task",
                &err,
            )
        })?;

    Ok(())
}

fn validate_task_info(_tasks_info: &CreateDraftTaskInfo) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
