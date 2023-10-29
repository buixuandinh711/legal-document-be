use deadpool_postgres::Client;
use serde::Serialize;

use super::ModelError;

#[derive(Serialize)]
pub struct SignatureInfo {
    id: i64,
    signer_name: String,
}

#[derive(Serialize)]
pub struct DocSignature {
    signer_name: String,
    position_name: String,
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

pub async fn get_doc_signatures(
    client: &Client,
    doc_content_hash: &str,
) -> Result<Vec<DocSignature>, ModelError> {
    let query_stmt = include_str!("../sql/signatures/query_doc_signatures.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_doc_signatures",
            &err,
        )
    })?;
    let query_result = client
        .query(&query_stmt, &[&doc_content_hash])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_doc_signatures",
                &err,
            )
        })?;

    let signatures: Vec<DocSignature> = query_result
        .iter()
        .map(|row| DocSignature {
            signer_name: row.get(0),
            position_name: row.get(1),
        })
        .collect();

    Ok(signatures)
}
