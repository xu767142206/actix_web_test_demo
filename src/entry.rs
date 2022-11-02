use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::{MySql, Pool};
use std::*;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub nickname: String,
    pub phone: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result<T> {
    pub data: T,
    pub msg: String,
    pub code: i32,
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    #[serde(default)]
    pub age: String,

    #[serde(default)]
    pub bra: String,
}

pub struct MysqlData {
    pub app_name: String,
    pub pool: Pool<MySql>,
}
