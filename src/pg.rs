use dotenvy_macro::dotenv;
use sqlx::{postgres, ConnectOptions};
use crate::error::DatabaseError;

pub async fn conn() -> Result<postgres::PgConnection, DatabaseError>  {
    Ok(postgres::PgConnectOptions::new()
        .host(dotenv!("DB_HOST"))
        .port(dotenv!("POSTGRES_PORT").parse::<u16>().unwrap())
        .username(dotenv!("POSTGRES_USER"))
        .password(dotenv!("POSTGRES_PASSWORD"))
        .database(dotenv!("DATABASE_NAME"))
        .connect()
        .await?)// TODO: Handle Err() on failure to connect
}
