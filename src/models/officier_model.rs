use crate::utils;

use super::ModelError;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};
use deadpool_postgres::Client;
use serde::Serialize;
use serde_repr::Serialize_repr;
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

#[derive(Debug, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum PositionRole {
    Revoked,
    DivisionAdmin,
    Manager,
    Staff,
}

impl<'a> FromSql<'a> for PositionRole {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = i16::from_sql(ty, raw).map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "FromSql: convert to PositionRole",
                &err,
            )
        })?;
        match val {
            0 => Ok(PositionRole::Revoked),
            1 => Ok(PositionRole::DivisionAdmin),
            2 => Ok(PositionRole::Manager),
            3 => Ok(PositionRole::Staff),
            val => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "FromSql: from SQL to PositionRole",
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

#[derive(PartialEq, Debug)]
pub enum DivisionStatus {
    NotCreated,
    Active,
    Deactivated,
}

impl<'a> FromSql<'a> for DivisionStatus {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = i16::from_sql(ty, raw).map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "FromSql: convert to PositionRole",
                &err,
            )
        })?;
        match val {
            0 => Ok(DivisionStatus::NotCreated),
            1 => Ok(DivisionStatus::Active),
            2 => Ok(DivisionStatus::Deactivated),
            val => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "FromSql: from SQL to PositionRole",
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

#[derive(Serialize)]
pub struct OfficerPosition {
    division_onchain_id: String,
    division_name: String,
    position_index: i16,
    position_name: String,
    position_role: PositionRole,
}

#[derive(Serialize)]
pub struct OfficerInfo {
    address: String,
    name: String,
    positions: Vec<OfficerPosition>,
}

#[derive(Serialize)]
pub struct OfficerPrivateKey {
    onchain_address: String,
    private_key: String,
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
    let query_password_result = client
        .query(&query_password_stmt, &[&auth_info.username])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_password",
                &err,
            )
        })?;

    let query_password_result = query_password_result.first().ok_or_else(|| {
        ModelError::new(
            ModelError::NotFoundError,
            "NotFound: get password",
            &auth_info.username,
        )
    })?;

    let hashed_password: String = query_password_result.get(1);
    let is_authenticated =
        bcrypt::verify(&auth_info.password, &hashed_password).map_err(|err| {
            ModelError::new(ModelError::InternalError, "Bcrypt: verify password", &err)
        })?;

    if is_authenticated {
        let officer_address: String = query_password_result.get(0);
        return Ok(Some(officer_address));
    } else {
        return Ok(None);
    }
}

pub async fn validate_and_get_info(
    client: &Client,
    officer_address: &str,
) -> Result<OfficerInfo, ModelError> {
    let query_officer_stmt = include_str!("../sql/officers/query_officer_info.sql");
    let query_finalize_stmt = client.prepare(query_officer_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_officer_info",
            &err,
        )
    })?;

    let query_officer_result = client
        .query(&query_finalize_stmt, &[&officer_address])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_officer_info",
                &err,
            )
        })?;

    if query_officer_result.is_empty() {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: query officer info",
            &officer_address,
        ));
    }

    let positions: Vec<OfficerPosition> = query_officer_result
        .iter()
        .map(|row| OfficerPosition {
            position_index: row.get(2),
            position_name: row.get(3),
            position_role: row.get(4),
            division_onchain_id: row.get(5),
            division_name: row.get(6),
        })
        .collect();

    let officer_status: OfficerStatus = query_officer_result[0].get(1);
    if officer_status != OfficerStatus::Active {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: officer not active",
            &(officer_status as u8),
        ));
    }

    let onchain_officer = OfficerInfo {
        address: officer_address.to_owned(),
        name: query_officer_result[0].get(0),
        positions,
    };

    Ok(onchain_officer)
}

pub async fn validate_and_get_role(
    client: &Client,
    officer_address: &str,
    division_onchain_id: &str,
    position_index: i16,
) -> Result<PositionRole, ModelError> {
    let query_officer_stmt = include_str!("../sql/officers/query_validation_info.sql");
    let query_finalize_stmt = client.prepare(query_officer_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_validation_info",
            &err,
        )
    })?;

    let query_officer_result = client
        .query_one(
            &query_finalize_stmt,
            &[&officer_address, &division_onchain_id, &position_index],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_validation_info",
                &err,
            )
        })?;

    let officer_status: OfficerStatus = query_officer_result.try_get(0).map_err(|err| {
        ModelError::new(ModelError::InternalError, "Auth: officer not found", &err)
    })?;
    if officer_status != OfficerStatus::Active {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: officer not active",
            &(officer_status as u8),
        ));
    }

    let division_status: DivisionStatus = query_officer_result.try_get(1).map_err(|err| {
        ModelError::new(ModelError::InternalError, "Auth: division not found", &err)
    })?;
    if division_status != DivisionStatus::Active {
        return Err(ModelError::new(
            ModelError::AuthError,
            "Auth: division not active",
            &(officer_status as u8),
        ));
    }

    let position_role: PositionRole = query_officer_result.try_get(2).map_err(|err| {
        ModelError::new(ModelError::InternalError, "Auth: position not found", &err)
    })?;

    Ok(position_role)
}

pub async fn get_private_key(
    client: &Client,
    officer_id: i64,
) -> Result<OfficerPrivateKey, ModelError> {
    let query_stmt = include_str!("../sql/officers/query_private_key.sql");
    let query_stmt = client.prepare(query_stmt).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare query_private_key",
            &err,
        )
    })?;

    let query_result = client
        .query_one(&query_stmt, &[&officer_id])
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute query_private_key",
                &err,
            )
        })?;

    let key_info = OfficerPrivateKey {
        onchain_address: query_result.get(0),
        private_key: query_result.get(1),
    };

    Ok(key_info)
}

fn validate_creattion_info(_info: &CreateOfficerInfo) -> bool {
    // TODO
    true
}

fn encrypt_private_key(private_key: &str, password: &str) -> Result<String, ModelError> {
    let hashed_password = utils::keccak256(password.as_bytes());
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
