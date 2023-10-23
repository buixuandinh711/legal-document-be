use deadpool_postgres::Client;
use serde::Serialize;

use super::ModelError;

#[derive(Serialize)]
pub struct SignatureInfo {
    id: i64,
    signer_name: String,
}

pub async fn get_draft_signatures(
    client: &Client,
    draft_id: i64,
) -> Result<Vec<SignatureInfo>, ModelError> {
    let query_stmt = include_str!("../sql/signatures/query_draft_signatures.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_draft_signatures",
            &err,
        )
    })?;
    let query_result = client
        .query(&query_stmt, &[&draft_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_draft_signatures",
                &err,
            )
        })?;

    let signatures: Vec<SignatureInfo> = query_result
        .iter()
        .map(|row| SignatureInfo {
            id: row.get(0),
            signer_name: row.get(1),
        })
        .collect();

    Ok(signatures)
}
