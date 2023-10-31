use deadpool_postgres::Client;
use serde::Serialize;

use super::ModelError;

#[derive(Serialize)]
pub struct DraftSignature {
    signer_name: String,
    position_name: String,
    signature: String,
}

#[derive(Serialize)]
pub struct DocSignature {
    signer_name: String,
    position_name: String,
}

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

pub async fn get_draft_signatures(
    client: &Client,
    draft_id: i64,
) -> Result<Vec<DraftSignature>, ModelError> {
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

    let signatures: Vec<DraftSignature> = query_result
        .iter()
        .map(|row| DraftSignature {
            signer_name: row.get(0),
            position_name: row.get(1),
            signature: row.get(2),
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

pub async fn get_signer_not_signed(
    client: &Client,
    draft_id: i64,
) -> Result<Vec<SignerPosition>, ModelError> {
    let query_stmt = include_str!("../sql/signatures/query_draft_signer.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_draft_signer",
            &err,
        )
    })?;
    let query_result = client
        .query(&query_stmt, &[&draft_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_draft_signer",
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
