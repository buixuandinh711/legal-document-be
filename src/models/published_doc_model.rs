use deadpool_postgres::Client;
use serde::Serialize;

use super::ModelError;

#[derive(Serialize)]
pub struct PublishedDoc {
    pub content_hash: String,
    pub number: String,
    pub name: String,
    pub publisher: String,
    pub published_date: i32,
}
#[derive(Serialize)]
pub struct PublishedDocDetail {
    pub content_hash: String,
    pub number: String,
    pub name: String,
    pub doc_type: String,
    pub publisher: String,
    pub published_date: i32,
    pub resource_uri: Option<String>,
}

pub async fn get_published_docs(
    client: &Client,
    division_onchain_id: &str,
) -> Result<Vec<PublishedDoc>, ModelError> {
    let statement = include_str!("../sql/onchain_documents/query_published_docs.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_published_docs",
            &err,
        )
    })?;

    let query_result = client
        .query(&statement, &[&division_onchain_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_published_docs",
                &err,
            )
        })?;

    let docs_list: Vec<PublishedDoc> = query_result
        .iter()
        .map(|row| PublishedDoc {
            content_hash: row.get(0),
            number: row.get(1),
            name: row.get(2),
            publisher: "".to_owned() + row.get(3) + row.get(4),
            published_date: row.get(5),
        })
        .collect();

    Ok(docs_list)
}

pub async fn get_published_detail(
    client: &Client,
    document_content_hash: &str,
) -> Result<PublishedDocDetail, ModelError> {
    let statement = include_str!("../sql/onchain_documents/query_published_doc_detail.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_published_doc_detail",
            &err,
        )
    })?;

    let query_result = client
        .query(&statement, &[&document_content_hash])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_published_doc_detail",
                &err,
            )
        })?;

    if query_result.is_empty() {
        return Err(ModelError::new(
            ModelError::NotFoundError,
            "get_published_detail: query draft by id",
            &document_content_hash,
        ));
    }
    let query_result = &query_result[0];

    let doc_detail = PublishedDocDetail {
        content_hash: query_result.get(0),
        number: query_result.get(1),
        name: query_result.get(2),
        doc_type: query_result.get(3),
        published_date: query_result.get(4),
        publisher: "".to_owned() + query_result.get(5) + " - " + query_result.get(6),
        resource_uri: query_result.get(7),
    };
    Ok(doc_detail)
}
