use std::time::SystemTime;

use deadpool_postgres::Client;
use serde::Serialize;
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

#[derive(Serialize)]
pub struct CreatedReviewTaskItem {
    id: i64,
    draft_name: String,
    assignee: String,
    assignee_position: String,
    created_at: SystemTime,
    status: i16,
}

#[derive(Serialize)]
pub struct AssignedReviewTaskItem {
    id: i64,
    draft_name: String,
    assigner: String,
    assigner_position: String,
    assigned_at: SystemTime,
    status: i16,
}

#[derive(Serialize)]
pub struct AssignedReviewTaskDetail {
    id: i64,
    draft_id: i64,
    draft_name: String,
    assigner: String,
    assigner_position: String,
    assigned_at: SystemTime,
    status: i16,
}

#[derive(Debug, Serialize)]
pub struct ReviewTaskDetail {
    pub draft_id: i64,
    pub assigner_address: String,
    pub assigner_division_id: String,
    pub assigner_position_index: i16,
    pub assignee_address: String,
    pub assignee_division_id: String,
    pub assignee_position_index: i16,
    pub created_at: SystemTime,
    pub status: i16,
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

    let mut statement = include_str!("../sql/review_tasks/create_review_task.sql").to_owned();
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
    let statement = include_str!("../sql/review_tasks/query_created_review_tasks.sql");
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
            assignee: row.get(2),
            assignee_position: row.get(3),
            created_at: row.get(4),
            status: row.get(5),
        })
        .collect();

    Ok(tasks)
}

pub async fn get_assigned_review_tasks(
    client: &Client,
    assignee_address: &str,
    assignee_div_id: &str,
    assignee_pos_index: i16,
) -> Result<Vec<AssignedReviewTaskItem>, ModelError> {
    let statement = include_str!("../sql/review_tasks/query_assigned_review_tasks.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_assigned_review_tasks",
            &err,
        )
    })?;

    let query_result = client
        .query(
            &statement,
            &[&assignee_address, &assignee_div_id, &assignee_pos_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_assigned_review_tasks",
                &err,
            )
        })?;

    let tasks: Vec<AssignedReviewTaskItem> = query_result
        .iter()
        .map(|row| AssignedReviewTaskItem {
            id: row.get(0),
            draft_name: row.get(1),
            assigner: row.get(2),
            assigner_position: row.get(3),
            assigned_at: row.get(4),
            status: row.get(5),
        })
        .collect();

    Ok(tasks)
}

pub async fn get_assigned_review_task_detail(
    client: &Client,
    assignee_address: &str,
    assignee_div_id: &str,
    assignee_pos_index: i16,
    task_id: i64,
) -> Result<AssignedReviewTaskDetail, ModelError> {
    let statement = include_str!("../sql/review_tasks/query_assigned_review_task_detail.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_assigned_review_task_detail",
            &err,
        )
    })?;

    let query_result = client
        .query_one(
            &statement,
            &[
                &assignee_address,
                &assignee_div_id,
                &assignee_pos_index,
                &task_id,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_assigned_review_task_detail",
                &err,
            )
        })?;

    let task = AssignedReviewTaskDetail {
        id: query_result.get(0),
        draft_id: query_result.get(1),
        draft_name: query_result.get(2),
        assigner: query_result.get(3),
        assigner_position: query_result.get(4),
        assigned_at: query_result.get(5),
        status: query_result.get(6),
    };

    Ok(task)
}

pub async fn get_review_task_detail(
    client: &Client,
    task_id: &i64,
) -> Result<ReviewTaskDetail, ModelError> {
    let statement = include_str!("../sql/review_tasks/query_review_task_detail.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_review_task_detail",
            &err,
        )
    })?;

    let query_result = client
        .query_one(&statement, &[&task_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_review_task_detail",
                &err,
            )
        })?;

    let task = ReviewTaskDetail {
        draft_id: query_result.get(0),
        assigner_address: query_result.get(1),
        assigner_division_id: query_result.get(2),
        assigner_position_index: query_result.get(3),
        assignee_address: query_result.get(4),
        assignee_division_id: query_result.get(5),
        assignee_position_index: query_result.get(6),
        created_at: query_result.get(7),
        status: query_result.get(8),
    };

    Ok(task)
}

pub async fn update_review_task_signed(client: &Client, task_id: &i64) -> Result<(), ModelError> {
    let statement = include_str!("../sql/review_tasks/update_review_task_signed.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare update_review_task_signed",
            &err,
        )
    })?;

    let execute_result = client
        .execute(&statement, &[&task_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute update_review_task_signed",
                &err,
            )
        })?;

    match execute_result {
        1 => Ok(()),
        val => Err(ModelError::new(
            ModelError::InternalError,
            "update_review_task_signed: invalid result number",
            &val,
        )),
    }
}

fn validate_review_task_info(_tasks_info: &[CreateReviewTaskInfo]) -> Result<(), ModelError> {
    // TODO: add validation logic
    Ok(())
}
