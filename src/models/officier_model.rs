use super::ModelError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use deadpool_postgres::Client;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{FromSql, Type};

#[derive(PartialEq, Debug)]
pub enum OfficerStatus {
    NotCreated,
    Active,
    Deactivated,
}

impl<'a> FromSql<'a> for OfficerStatus {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match i16::from_sql(ty, raw)? {
            0 => Ok(OfficerStatus::NotCreated),
            1 => Ok(OfficerStatus::Active),
            2 => Ok(OfficerStatus::Deactivated),
            val => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "FromSql: invalid value",
                &val,
            ))),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        if *ty == Type::INT2 {
            return true;
        }
        false
    }
}

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

pub struct AuthOfficerInfo {
    pub username: String,
    pub password: String,
}

pub async fn create_officer(
    client: &Client,
    officer_info: &CreateOfficerInfo,
) -> Result<(), ModelError> {
    if !validate_creattion_info(&officer_info) {
        return Err(ModelError::new(
            ModelError::ValidationError,
            "Validation: invalid create officer info",
            &"",
        ));
    }

    let statement = include_str!("../sql/officers/create_officer.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_officer",
            &err,
        )
    })?;

    let hashed_password = bcrypt::hash(&officer_info.password, bcrypt::DEFAULT_COST)
        .map_err(|err| ModelError::new(ModelError::InternalError, "Bcrypt: hash password", &err))?;

    let lowercase_address = officer_info.onchain_address.to_lowercase();
    let lowercase_pk = officer_info.private_key.to_lowercase();
    let encrypted_pk = encrypt_private_key(&lowercase_pk, &officer_info.password)?;

    let _ = client
        .execute(
            &statement,
            &[
                &officer_info.username,
                &hashed_password,
                &lowercase_address,
                &encrypted_pk,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_officer",
                &err,
            )
        })?;

    Ok(())
}

pub async fn authenticate_officer(
    client: &Client,
    auth_info: &AuthOfficerInfo,
) -> Result<Option<String>, ModelError> {
    let query_password_stmt = include_str!("../sql/officers/query_password.sql");
    let query_password_stmt = client.prepare(query_password_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_password",
            &err,
        )
    })?;
    let mut query_password_result = client
        .query(&query_password_stmt, &[&auth_info.username])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_password",
                &err,
            )
        })?;

    if query_password_result.is_empty() {
        return Err(ModelError::new(
            ModelError::NotFoundError,
            "NotFound: get password",
            &auth_info.username,
        ));
    }
    let query_password_result = query_password_result.remove(0);

    let onchain_address: String = query_password_result.get(2);
    validate_onchain_status(client, &onchain_address).await?;

    let hashed_password: String = query_password_result.get(1);
    let is_authenticated =
        bcrypt::verify(&auth_info.password, &hashed_password).map_err(|err| {
            ModelError::new(ModelError::InternalError, "Bcrypt: verify password", &err)
        })?;

    if is_authenticated {
        let officer_id: i64 = query_password_result.get(0);
        return Ok(Some(officer_id.to_string()));
    } else {
        return Ok(None);
    }
}

fn validate_creattion_info(info: &CreateOfficerInfo) -> bool {
    // TODO
    {
        info
    };
    true
}

fn encrypt_private_key(private_key: &str, password: &str) -> Result<String, ModelError> {
    let hashed_password = ethers_core::utils::keccak256(password.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&hashed_password);
    let cipher = Aes256Gcm::new(&key);

    let nonce: &[u8] = &[42; 12];
    let nonce = Nonce::from_slice(nonce);

    let encrypted_pk = cipher
        .encrypt(nonce, private_key.as_bytes())
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "AesGcm: encrypt private key",
                &err,
            )
        })?;

    let hex_string = encrypted_pk
        .iter()
        .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

    Ok(hex_string)
}

async fn validate_onchain_status(client: &Client, onchain_address: &str) -> Result<(), ModelError> {
    let query_finalize_stmt = include_str!("../sql/officers/query_finalization.sql");
    let query_finalize_stmt = client.prepare(query_finalize_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_finalization",
            &err,
        )
    })?;

    let query_finalize_result = client
        .query(&query_finalize_stmt, &[&onchain_address])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_finalization",
                &err,
            )
        })?;

    if query_finalize_result.is_empty() {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: officer not finalized",
            &onchain_address,
        ));
    }

    let status: OfficerStatus = query_finalize_result[0].get(0);
    if status != OfficerStatus::Active {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: invalid officer onchain status",
            &(status as u8),
        ));
    }

    Ok(())
}
