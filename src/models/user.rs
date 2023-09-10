use deadpool_postgres::{Client, PoolError};
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
    pub raw_passwrod: String,
}

impl From<ReqRegisterBody> for CreateUserInfo {
    fn from(value: ReqRegisterBody) -> Self {
        CreateUserInfo {
            username: value.username,
            raw_passwrod: value.password,
        }
    }
}

pub async fn create_user(client: &Client, user_info: &CreateUserInfo) -> Result<i32, PoolError> {
    let statement = include_str!("../sql/create-user.sql");
    let statement = client.prepare(&statement).await.unwrap();

    let user_id = client
        .query(&statement, &[&user_info.username, &user_info.raw_passwrod])
        .await?
        .iter()
        .map(|row| row.get(0))
        .collect::<Vec<i32>>()
        .pop()
        .unwrap();

    Ok(user_id)
}
