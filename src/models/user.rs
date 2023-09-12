use bcrypt::{hash, DEFAULT_COST};
use deadpool_postgres::Client;
use derive_more::{Display, Error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::routes::auth::ReqRegisterBody;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "user")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

pub struct CreateUserInfo {
    pub username: String,
    pub raw_password: String,
}

impl From<ReqRegisterBody> for CreateUserInfo {
    fn from(value: ReqRegisterBody) -> Self {
        CreateUserInfo {
            username: value.username,
            raw_password: value.password,
        }
    }
}

#[derive(Debug, Display, Error)]
pub enum UserModelError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "Database pool error: {}", msg)]
    DBPoolError { msg: String },
    #[display(fmt = "{}", msg)]
    OtherError { msg: String },
}

pub async fn create_user(
    client: &Client,
    user_info: &CreateUserInfo,
) -> Result<i32, UserModelError> {
    let statement = include_str!("../sql/create-user.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|_| UserModelError::DBPoolError {
            msg: "Failed to prepare statement".to_owned(),
        })?;

    validate_user_info(user_info)?;
    let hashed_pwd =
        hash(&user_info.raw_password, DEFAULT_COST).map_err(|_| UserModelError::OtherError {
            msg: "Failed to hash password".to_owned(),
        })?;

    let user_id = client
        .query(&statement, &[&user_info.username, &hashed_pwd])
        .await
        .map_err(|_| UserModelError::DBPoolError {
            msg: "Failed to query".to_owned(),
        })?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i32>>()
        .pop()
        .unwrap();

    Ok(user_id)
}

fn validate_user_info(user_info: &CreateUserInfo) -> Result<(), UserModelError> {
    let username_regex = Regex::new(r"^[\w]{6,20}$").unwrap();
    if !username_regex.is_match(&user_info.username) {
        return Err(UserModelError::ValidationError {
            field: "username".to_owned(),
        });
    }

    let password_regex = Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
    if !password_regex.is_match(&user_info.raw_password) {
        return Err(UserModelError::ValidationError {
            field: "password".to_owned(),
        });
    }

    Ok(())
}
