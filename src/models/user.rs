use bcrypt::{hash, verify, DEFAULT_COST};
use deadpool_postgres::Client;
use derive_more::{Display, Error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

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

pub struct LoginUserInfo {
    pub username: String,
    pub raw_password: String,
}

#[derive(Debug, Display, Error)]
pub enum UserModelError {
    #[display(fmt = "Validation error: {}", msg)]
    ValidationError { msg: String },
    #[display(fmt = "Database pool error: {}", msg)]
    DBPoolError { msg: String },
    #[display(fmt = "Not found error: {}", msg)]
    NotFoundError { msg: String },
    #[display(fmt = "Other error: {}", msg)]
    OtherError { msg: String },
}

pub async fn create_user(
    client: &Client,
    user_info: &CreateUserInfo,
) -> Result<i32, UserModelError> {
    validate_register_info(user_info)?;

    let statement = include_str!("../sql/create-user.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|_| UserModelError::DBPoolError {
            msg: "Failed to prepare statement".to_owned(),
        })?;

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

pub async fn verify_user(
    client: &Client,
    user_info: &LoginUserInfo,
) -> Result<bool, UserModelError> {
    validate_login_info(user_info)?;

    let statement = include_str!("../sql/get-user.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|_| UserModelError::DBPoolError {
            msg: "Failed to prepare statement".to_owned(),
        })?;

    let query_password = client
        .query(&statement, &[&user_info.username])
        .await
        .map_err(|_| UserModelError::DBPoolError {
            msg: "Failed to query".to_owned(),
        })?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<String>>()
        .pop();

    match query_password {
        Some(hashed_password) => {
            let is_verified = verify(&user_info.raw_password, &hashed_password).map_err(|_| {
                UserModelError::OtherError {
                    msg: "Hash error".to_owned(),
                }
            })?;
            Ok(is_verified)
        }
        None => Err(UserModelError::NotFoundError {
            msg: "username not exist".to_owned(),
        }),
    }
}

fn validate_register_info(user_info: &CreateUserInfo) -> Result<(), UserModelError> {
    let username_regex = Regex::new(r"^[\w]{6,20}$").unwrap();
    if !username_regex.is_match(&user_info.username) {
        return Err(UserModelError::ValidationError {
            msg: "Invalid username".to_owned(),
        });
    }

    let password_regex = Regex::new(r"^[A-Za-z\d#$@!%&*?]{8,30}$").unwrap();
    if !password_regex.is_match(&user_info.raw_password)
        || !user_info.raw_password.chars().any(|c| c.is_alphabetic()) // rust regex not support lookaround yet.
        || !user_info.raw_password.chars().any(|c| c.is_numeric())
        || !user_info
            .raw_password
            .chars()
            .any(|c| "#$@!%&*?".contains(c))
    {
        return Err(UserModelError::ValidationError {
            msg: "Invalid password".to_owned(),
        });
    }

    Ok(())
}

fn validate_login_info(user_info: &LoginUserInfo) -> Result<(), UserModelError> {
    let username_regex = Regex::new(r"^[\w]{6,20}$").unwrap();
    if !username_regex.is_match(&user_info.username) {
        return Err(UserModelError::ValidationError {
            msg: "Invalid username".to_owned(),
        });
    }

    let password_regex = Regex::new(r"^[A-Za-z\d#$@!%&*?]{8,30}$").unwrap();
    if !password_regex.is_match(&user_info.raw_password)
        || !user_info.raw_password.chars().any(|c| c.is_alphabetic()) // rust regex not support lookaround yet.
        || !user_info.raw_password.chars().any(|c| c.is_numeric())
        || !user_info
            .raw_password
            .chars()
            .any(|c| "#$@!%&*?".contains(c))
    {
        return Err(UserModelError::ValidationError {
            msg: "Invalid password".to_owned(),
        });
    }

    Ok(())
}
