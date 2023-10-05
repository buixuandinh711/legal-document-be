use cloud_storage::Client;
use flate2::{write::GzEncoder, Compression};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_pg_mapper_derive::PostgresMapper;

use super::ModelError;

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "divisions")]
pub struct Document {
    pub id: i64,
    pub hash: String,
    pub resource_uri: String,
}

async fn create_document(cloud_client: &Client, file: &mut File) -> Result<(), ModelError> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.map_err(|err| {
        ModelError::new(ModelError::InternalError, "File: read file to buffer", &err)
    })?;
    let doc_hash = ethers::utils::keccak256(&buffer);
    let doc_hash = doc_hash
        .iter()
        .fold("".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

    let file_name = format!("{}.gzip", doc_hash);

    upload_to_storage(cloud_client, &buffer, &file_name).await?;

    Ok(())
}

async fn upload_to_storage(
    cloud_client: &Client,
    file_buffer: &[u8],
    file_name: &str,
) -> Result<(), ModelError> {
    let bucket_name = dotenv::var("CLOUD_STORAGE_BUCKET").map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "LoadEnv: load environment variable",
            &err,
        )
    })?;

    cloud_client
        .object()
        .create(
            &bucket_name,
            file_buffer.to_vec(),
            file_name,
            "application/gzip",
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "CloudStorage: upload to bucket",
                &err,
            )
        })?;

    Ok(())
}
