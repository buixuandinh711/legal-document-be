use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;

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
                "{} (${},${},${},${},${},${},${})",
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
                "{}, (${},${},${},${},${},${},${})",
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

    let result = client.query_one(&statement, &params).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: execute create_review_task",
            &err,
        )
    })?;

    Ok(())
}

fn validate_review_task_info(_tasks_info: &[CreateReviewTaskInfo]) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
