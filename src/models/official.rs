use std::error::Error;

use bytes::BytesMut;
use deadpool_postgres::Client;
use derive_more::{Display, Error as DeriveError};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Debug, Display, DeriveError)]
pub enum OfficialModelError {
    #[display(fmt = "Failed to convert from SQL type")]
    FromSqlError,
    #[display(fmt = "Failed to convert to SQL type")]
    ToSqlError,
    #[display(fmt = "Database pool error: {}", msg)]
    DBPoolError { msg: &'static str },
    #[display(fmt = "Other error: {}", msg)]
    OtherError { msg: &'static str },
}

#[derive(PartialEq, Debug)]
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
            _ => Err(Box::new(OfficialModelError::FromSqlError)),
        }
    }

    fn accepts(ty: &tokio_postgres::types::Type) -> bool {
        if *ty == Type::INT2 {
            return true;
        }
        false
    }
}

impl ToSql for OfficialStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match *ty {
            Type::INT2 => {
                let value: i16 = match self {
                    OfficialStatus::NotCreated => 0,
                    OfficialStatus::Active => 1,
                    OfficialStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            _ => Err(Box::new(OfficialModelError::ToSqlError)),
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
#[pg_mapper(table = "officials")]
pub struct Official {
    pub id: i32,
    pub address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub status: OfficialStatus,
    pub password: String,
    pub private_key: String,
}

pub async fn get_official(client: &Client, address: &str) -> Result<Official, OfficialModelError> {
    let statement = "SELECT * FROM officials WHERE address = $1";
    let statement =
        client
            .prepare(&statement)
            .await
            .map_err(|_| OfficialModelError::DBPoolError {
                msg: "Failed to prepare statement",
            })?;

    let query = client
        .query_one(&statement, &[&address])
        .await
        .map_err(|_| OfficialModelError::DBPoolError {
            msg: "Failed to query",
        })?;
    let official = Official::from_row(query).unwrap();

    Ok(official)
}

pub struct CreateOfficalInfo {
    pub address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
    pub password: String,
    pub private_key: String,
}

pub async fn create_offcial(
    client: &Client,
    official_info: &CreateOfficalInfo,
) -> Result<(), OfficialModelError> {
    let statement = include_str!("../sql/create_official.sql");
    let statement =
        client
            .prepare(&statement)
            .await
            .map_err(|_| OfficialModelError::DBPoolError {
                msg: "Failed to prepare statement",
            })?;

    let hashed_password =
        bcrypt::hash(&official_info.password, bcrypt::DEFAULT_COST).map_err(|_| {
            OfficialModelError::OtherError {
                msg: "Failed to hash password",
            }
        })?;

    let _ = client
        .execute(
            &statement,
            &[
                &official_info.address,
                &official_info.name,
                &official_info.date_of_birth,
                &official_info.sex,
                &OfficialStatus::Active,
                &hashed_password,
                &official_info.private_key,
            ],
        )
        .await
        .map_err(|_| OfficialModelError::DBPoolError {
            msg: "Failed to execute insert official",
        })?;

    Ok(())
}
