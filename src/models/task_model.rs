use std::time::SystemTime;

use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;
use serde::Serialize;

use super::ModelError;

pub struct CreateReviewTaskInfo {
    pub draft_id: i64,
    pub assigner_address: String,
    pub assigner_division_id: String,
    pub assigner_position_index: i16,
    pub assignee_address: String,
    pub assignee_division_id: String,
    pub assignee_position_index: i16,
}

#[derive(Serialize)]
pub struct CreatedReviewTaskItem {
    id: i64,
    draft_name: String,
    assignee: String,
    created_at: SystemTime,
    status: i16,
}

pub async fn create_review_task(
    client: &Client,
    tasks_info: &[CreateReviewTaskInfo],
) -> Result<(), ModelError> {
    if validate_review_task_info(&tasks_info).is_err() {
        return Err(ModelError::new(
            ModelError::ValidationError,
            "Validation: invalid create draft info",
            &"",
        ));
    }

    let mut statement = include_str!("../sql/tasks/create_review_task.sql").to_owned();
    let mut params = Vec::<&(dyn ToSql + Sync)>::new();
    let mut i = 1;

    for row in tasks_info.iter() {
        if i == 1 {
            statement = format!(
                "{} (${},${},${},${},${},${},${}, 0)",
                statement,
                i,
                i + 1,
                i + 2,
                i + 3,
                i + 4,
                i + 5,
                i + 6
            );
        } else {
            statement = format!(
                "{}, (${},${},${},${},${},${},${}, 0)",
                statement,
                i,
                i + 1,
                i + 2,
                i + 3,
                i + 4,
                i + 5,
                i + 6
            );
        }

        params.push(&row.draft_id);
        params.push(&row.assigner_address);
        params.push(&row.assigner_division_id);
        params.push(&row.assigner_position_index);
        params.push(&row.assignee_address);
        params.push(&row.assignee_division_id);
        params.push(&row.assignee_position_index);

        i = i + 7;
    }

    statement += ";";

    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_review_task",
            &err,
        )
    })?;

    let _result = client.execute(&statement, &params).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: execute create_review_task",
            &err,
        )
    })?;

    Ok(())
}

pub async fn get_created_review_tasks(
    client: &Client,
    assigner_address: &str,
    assigner_div_id: &str,
    assigner_pos_index: i16,
) -> Result<Vec<CreatedReviewTaskItem>, ModelError> {
    let statement = include_str!("../sql/tasks/query_created_review_tasks.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_created_review_tasks",
            &err,
        )
    })?;

    let query_result = client
        .query(
            &statement,
            &[&assigner_address, &assigner_div_id, &assigner_pos_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_created_review_tasks",
                &err,
            )
        })?;

    let tasks: Vec<CreatedReviewTaskItem> = query_result
        .iter()
        .map(|row| CreatedReviewTaskItem {
            id: row.get(0),
            draft_name: row.get(1),
            assignee: row.get::<usize, String>(2) + row.get(3),
            created_at: row.get(4),
            status: row.get(5),
        })
        .collect();

    Ok(tasks)
}

fn validate_review_task_info(_tasks_info: &[CreateReviewTaskInfo]) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
