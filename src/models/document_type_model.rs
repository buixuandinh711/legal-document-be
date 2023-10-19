use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use serde::Serialize;

use super::ModelError;

#[derive(PostgresMapper, Debug, Serialize)]
#[pg_mapper(table = "document_types")]
pub struct DocumentType {
    pub id: i32,
    pub name: String,
}

pub async fn get_document_types(client: &Client) -> Result<Vec<DocumentType>, ModelError> {
    let query_stmt = include_str!("../sql/document_types/query_document_types.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_document_types",
            &err,
        )
    })?;
    let query_result = client.query(&query_stmt, &[]).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: execute query_document_types",
            &err,
        )
    })?;

    let doc_types: Vec<DocumentType> = query_result
        .iter()
        .map(|row| DocumentType::from_row_ref(row).unwrap())
        .collect();

    Ok(doc_types)
}
