use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{FromSql, Type};

#[derive(Debug, Display, Error)]
pub enum OfficialModelError {
    #[display(fmt = "Convert from sql error: {}", msg)]
    FromSqlError { msg: &'static str },
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum OfficialStatus {
    NotCreated,
    Active,
    Deactivated,
}

impl<'a> FromSql<'a> for OfficialStatus {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match i16::from_sql(ty, raw)? {
            0 => Ok(OfficialStatus::NotCreated),
            1 => Ok(OfficialStatus::Active),
            2 => Ok(OfficialStatus::Deactivated),
            _ => Err(Box::new(OfficialModelError::FromSqlError {
                msg: "officials.status value not satisfied",
            })),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        if *ty == Type::INT2 {
            return true;
        }
        false
    }
}

#[derive(Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "officials")]
pub struct Official {
    pub id: i32,
    pub address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub status: OfficialStatus,
}
