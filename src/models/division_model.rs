use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

use super::ModelError;

#[derive(Debug)]
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
                "FromSql: convert to DivisionStatus",
                &err,
            )
        })?;
        match val {
            0 => Ok(DivisionStatus::NotCreated),
            1 => Ok(DivisionStatus::Active),
            2 => Ok(DivisionStatus::Deactivated),
            val => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "FromSql: from SQL to DivisionStatus",
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

impl ToSql for DivisionStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match ty {
            &Type::INT2 => {
                let value: i16 = match self {
                    DivisionStatus::NotCreated => 0,
                    DivisionStatus::Active => 1,
                    DivisionStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            ty => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "ToSql: DivisionStatus to SQL",
                ty,
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
#[pg_mapper(table = "divisions")]
pub struct Division {
    pub id: i64,
    pub onchain_id: String,
    pub name: String,
    pub supervisory_id: i64,
    pub status: DivisionStatus,
}

pub struct CreateDivisionInfo {
    pub onchain_id: String,
    pub name: String,
    pub onchain_supervisory_id: String,
}

pub async fn create_division(
    client: &Client,
    division_info: &CreateDivisionInfo,
) -> Result<(), ModelError> {

    let statement = include_str!("../sql/division/create_division.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_division",
            &err,
        )
    })?;

    let _ = client
        .execute(
            &statement,
            &[
                &division_info.onchain_id,
                &division_info.name,
                &division_info.onchain_supervisory_id,
                &DivisionStatus::Active,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_division",
                &err,
            )
        })?;

    Ok(())
}