use super::ModelError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use deadpool_postgres::Client;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "officers")]
pub struct Officer {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub onchain_address: String,
    pub private_key: String,
}

pub struct CreateOfficerInfo {
    pub username: String,
    pub password: String,
    pub onchain_address: String,
    pub private_key: String,
}

pub async fn create_officer(
    client: &Client,
    officer_info: &CreateOfficerInfo,
) -> Result<(), ModelError> {
    if !validate_creattion_info(&officer_info) {
        return Err(ModelError::new(
            ModelError::ValidationError,
            "Invalid CreateOfficerInfo",
        ));
    }

    let statement = include_str!("../sql/officer/create_officer.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|_| ModelError::new(ModelError::InternalError, "Failed to prepare statement"))?;

    let hashed_password = bcrypt::hash(&officer_info.password, bcrypt::DEFAULT_COST)
        .map_err(|_| ModelError::new(ModelError::InternalError, "Failed to hash password"))?;

    let encrypted_pk = encrypt_private_key(&officer_info.private_key, &officer_info.password)?;

    let _ = client
        .execute(
            &statement,
            &[
                &officer_info.username,
                &hashed_password,
                &officer_info.onchain_address,
                &encrypted_pk,
            ],
        )
        .await
        .map_err(|e| {
            println!("{}", e.to_string());
            ModelError::new(
                ModelError::InternalError,
                "Failed to execute insert officer",
            )
        })?;

    Ok(())
}

fn validate_creattion_info(info: &CreateOfficerInfo) -> bool {
    // TODO
    info;
    true
}

fn encrypt_private_key(private_key: &str, password: &str) -> Result<String, ModelError> {
    let hashed_password = ethers::utils::keccak256(password.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&hashed_password);
    let cipher = Aes256Gcm::new(&key);

    let nonce: &[u8] = &[42; 12];
    let nonce = Nonce::from_slice(nonce);

    let encrypted_pk = cipher
        .encrypt(nonce, private_key.as_bytes())
        .map_err(|_| ModelError::new(ModelError::InternalError, "Failed to encrypt private key"))?;

    let hex_string = encrypted_pk
        .iter()
        .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

    Ok(hex_string)
}
