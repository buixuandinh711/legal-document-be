use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

use super::ModelError;

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
            _ => Err(Box::new(ModelError::new(
                ModelError::InternalError,
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
            _ => Err(Box::new(ModelError::new(
                ModelError::InternalError,
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
#[pg_mapper(table = "onchain_officers")]
pub struct Officer {
    pub id: i32,
    pub onchain_address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub status: OfficerStatus,
}

pub struct CreateOnchainOfficerInfo {
    pub onchain_address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
}

pub async fn create_onchain_officer(
    client: &Client,
    officer_info: &CreateOnchainOfficerInfo,
) -> Result<(), ModelError> {
    let statement = include_str!("../sql/onchain_officer/create_onchain_officer.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            &format!(
                "DbPool: Failed to prepare create_onchain_officer\nContent: {}",
                err.to_string()
            ),
        )
    })?;

    let _ = client
        .execute(
            &statement,
            &[
                &officer_info.onchain_address,
                &officer_info.name,
                &officer_info.date_of_birth,
                &officer_info.sex,
                &OfficerStatus::Active,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                &format!(
                    "DbPool: Failed to execute create_onchain_officer\nContent: {}",
                    err.to_string()
                ),
            )
        })?;

    Ok(())
}
