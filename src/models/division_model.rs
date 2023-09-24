use bytes::BytesMut;
use deadpool_postgres::Client;
use derive_more::{Display, Error as DeriveError};
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Display, DeriveError)]
pub enum DivisionModelError {
    #[display(fmt = "Failed to convert from SQL type")]
    FromSqlError,
    #[display(fmt = "Failed to convert to SQL type")]
    ToSqlError,
    #[display(fmt = "Official not found")]
    OfficialNotFound,
    #[display(fmt = "Database pool error: {}", msg)]
    DBPoolError { msg: &'static str },
    #[display(fmt = "Other error: {}", msg)]
    OtherError { msg: &'static str },
}

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
        match i16::from_sql(ty, raw)? {
            0 => Ok(DivisionStatus::NotCreated),
            1 => Ok(DivisionStatus::Active),
            2 => Ok(DivisionStatus::Deactivated),
            _ => Err(Box::new(DivisionModelError::FromSqlError)),
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
        match *ty {
            Type::INT2 => {
                let value: i16 = match self {
                    DivisionStatus::NotCreated => 0,
                    DivisionStatus::Active => 1,
                    DivisionStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            _ => Err(Box::new(DivisionModelError::ToSqlError)),
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
    pub supervisory_id: i64,
}

pub async fn create_division(
    client: &Client,
    division_info: &CreateDivisionInfo,
) -> Result<(), DivisionModelError> {
    let statement = include_str!("../sql/create_division.sql");
    let statement =
        client
            .prepare(&statement)
            .await
            .map_err(|_| DivisionModelError::DBPoolError {
                msg: "Failed to prepare statement",
            })?;

    let _ = client
        .execute(
            &statement,
            &[
                &division_info.onchain_id,
                &division_info.name,
                &division_info.supervisory_id,
                &DivisionStatus::Active,
            ],
        )
        .await
        .unwrap();

    Ok(())
}
