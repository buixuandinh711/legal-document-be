use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

use super::ModelError;

#[derive(Debug)]
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

impl ToSql for PositionRole {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match ty {
            &Type::INT2 => {
                let value: i16 = match self {
                    PositionRole::Revoked => 0,
                    PositionRole::DivisionAdmin => 1,
                    PositionRole::Manager => 2,
                    PositionRole::Staff => 3,
                };
                value.to_sql(ty, out)
            }
            ty => Err(Box::new(ModelError::new(
                ModelError::InternalError,
                "ToSql: PositionRole to SQL",
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

impl TryFrom<&u8> for PositionRole {
    type Error = ModelError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PositionRole::Revoked),
            1 => Ok(PositionRole::DivisionAdmin),
            2 => Ok(PositionRole::Manager),
            3 => Ok(PositionRole::Staff),
            val => Err(ModelError::new(
                ModelError::InternalError,
                "ToPositionRole: invalid source value",
                val,
            )),
        }
    }
}

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "positions")]
pub struct Position {
    pub onchain_officer_id: i64,
    pub division_id: i64,
    pub position_index: i16,
    pub name: String,
    pub role: PositionRole,
}

pub struct CreatePositionInfo {
    pub officer_address: String,
    pub division_onchain_id: String,
    pub position_index: i16,
    pub name: String,
    pub role: u8,
}

pub async fn create_position(
    client: &Client,
    position_info: &CreatePositionInfo,
) -> Result<(), ModelError> {
    let statement = include_str!("../sql/position/create_position.sql");
    let statement = client.prepare(&statement).await.map_err(|err| {
        ModelError::new(
            ModelError::InternalError,
            "DbPool: prepare create_position",
            &err,
        )
    })?;

    let _ = client
        .execute(
            &statement,
            &[
                &position_info.officer_address,
                &position_info.division_onchain_id,
                &position_info.position_index,
                &position_info.name,
                &PositionRole::try_from(&position_info.role)?,
            ],
        )
        .await
        .map_err(|err| {
            ModelError::new(
                ModelError::InternalError,
                "DbPool: execute create_position",
                &err,
            )
        })?;

    Ok(())
}
