use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use bytes::BytesMut;
use deadpool_postgres::Client;
use derive_more::{Display, Error as DeriveError};
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Display, DeriveError)]
pub enum OfficerModelError {
    OfficerNotFound,
    ValidationError,
    InternalError,
}

impl OfficerModelError {
    fn new(err: Self, msg: &str) -> Self {
        log::error!("{}", msg);
        err
    }
}

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
            _ => Err(Box::new(OfficerModelError::new(
                OfficerModelError::InternalError,
                "Failed to convert from sql to OfficerStatus",
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

impl ToSql for OfficerStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match *ty {
            Type::INT2 => {
                let value: i16 = match self {
                    OfficerStatus::NotCreated => 0,
                    OfficerStatus::Active => 1,
                    OfficerStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            _ => Err(Box::new(OfficerModelError::new(
                OfficerModelError::InternalError,
                "Failed to convert OfficerStatus to SQL type",
            ))),
        }
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        *ty == Type::INT2
    }

    to_sql_checked!();
}

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "officers")]
pub struct Officer {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub onchain_address: String,
    pub private_key: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub status: OfficerStatus,
    pub transaction_hash: String,
    pub finalized: bool,
}

pub struct CreateOfficerInfo {
    pub username: String,
    pub password: String,
    pub onchain_address: String,
    pub private_key: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub transaction_hash: String,
}

pub async fn create_officer(
    client: &Client,
    officer_info: &CreateOfficerInfo,
) -> Result<(), OfficerModelError> {
    if !validate_creattion_info(&officer_info) {
        return Err(OfficerModelError::new(
            OfficerModelError::ValidationError,
            "Invalid CreateOfficerInfo",
        ));
    }

    let statement = include_str!("../sql/officer/create_officer.sql");
    let statement = client.prepare(&statement).await.map_err(|_| {
        OfficerModelError::new(
            OfficerModelError::InternalError,
            "Failed to prepare statement",
        )
    })?;

    let hashed_password =
        bcrypt::hash(&officer_info.password, bcrypt::DEFAULT_COST).map_err(|_| {
            OfficerModelError::new(OfficerModelError::InternalError, "Failed to hash password")
        })?;

    let encrypted_pk = encrypt_private_key(&officer_info.private_key, &officer_info.password)?;

    let _ = client
        .execute(
            &statement,
            &[
                &officer_info.username,
                &hashed_password,
                &officer_info.onchain_address,
                &encrypted_pk,
                &officer_info.name,
                &officer_info.date_of_birth,
                &officer_info.sex,
                &OfficerStatus::Active,
                &officer_info.transaction_hash,
                &false,
            ],
        )
        .await
        .map_err(|_| {
            OfficerModelError::new(
                OfficerModelError::InternalError,
                "Failed to execute insert officer",
            )
        })?;

    Ok(())
}

pub async fn finalized_officer_creattion(
    client: &Client,
    transaction_hash: &str,
) -> Result<(), OfficerModelError> {
    let statement = include_str!("../sql/officer/update_finalized.sql");
    let statement = client.prepare(&statement).await.map_err(|_| {
        OfficerModelError::new(
            OfficerModelError::InternalError,
            "Failed to prepare update_finalized statement",
        )
    })?;

    let _ = client
        .execute(&statement, &[&transaction_hash])
        .await
        .map_err(|_| {
            OfficerModelError::new(
                OfficerModelError::InternalError,
                "Failed to execute update_finalized",
            )
        })?;

    Ok(())
}

fn validate_creattion_info(info: &CreateOfficerInfo) -> bool {
    // TODO
    info;
    true
}

fn encrypt_private_key(private_key: &str, password: &str) -> Result<String, OfficerModelError> {
    let hashed_password = ethers::utils::keccak256(password.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&hashed_password);
    let cipher = Aes256Gcm::new(&key);

    let nonce: &[u8] = &[42; 12];
    let nonce = Nonce::from_slice(nonce);

    let encrypted_pk = cipher.encrypt(nonce, private_key.as_bytes()).map_err(|_| {
        OfficerModelError::new(
            OfficerModelError::InternalError,
            "Failed to encrypt private key",
        )
    })?;

    let hex_string = encrypted_pk
        .iter()
        .fold("0x".to_owned(), |acc, byte| acc + &format!("{:02x}", byte));

    Ok(hex_string)
}
