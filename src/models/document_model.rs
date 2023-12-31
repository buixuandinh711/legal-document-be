use std::{fs::File, io::Read};

use deadpool_postgres::Client;

use crate::{app_config::CloudStorage, utils};

use super::ModelError;

pub async fn create_document(
    db_client: &Client,
    cloud_storage: &CloudStorage,
    file: &mut File,
) -> Result<String, ModelError> {
    let (doc_hash, buffer) = preprocess_file(file).await?;
    let resource_uri = upload_to_storage(cloud_storage, &buffer, &doc_hash).await?;
    save_to_db(db_client, &doc_hash, &resource_uri).await?;

    Ok(doc_hash)
}

async fn preprocess_file(file: &mut File) -> Result<(String, Vec<u8>), ModelError> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|err| {
        ModelError::new(ModelError::InternalError, "File: read file to buffer", &err)
    })?;

    let doc_hash = get_doc_hash(&buffer)?;

    Ok((doc_hash, buffer))
}

async fn upload_to_storage(
    cloud_storage: &CloudStorage,
    file_buffer: &[u8],
    file_name: &str,
) -> Result<String, ModelError> {
    cloud_storage
        .client
        .object()
        .create(
            &cloud_storage.bucket_name,
            file_buffer.to_vec(),
            file_name,
            "application/pdf",
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "CloudStorage: upload to bucket",
                &err,
            )
        })?;

    let resource_uri = format!(
        "{}/{}/{}",
        cloud_storage.base_url, cloud_storage.bucket_name, file_name
    );

    Ok(resource_uri)
}

async fn save_to_db(client: &Client, doc_hash: &str, resource_uri: &str) -> Result<(), ModelError> {
    let statement = include_str!("../sql/documents/create_document.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_document",
            &err,
        )
    })?;

    let _ = client
        .execute(&statement, &[&doc_hash, &resource_uri])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_document",
                &err,
            )
        })?;

    Ok(())
}

fn get_doc_hash(buf: &[u8]) -> Result<String, ModelError> {
    let compressed_doc = utils::compress(&buf).map_err(|err| {
        ModelError::new(ModelError::InternalError, "Document: compress doc", &err)
    })?;
    let doc_hash = utils::keccak256(&compressed_doc);
    let doc_hash = doc_hash
        .iter()
        .fold("".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

    Ok(doc_hash)
}
