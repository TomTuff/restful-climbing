use dotenvy_macro::dotenv;
use sqlx::{postgres, ConnectOptions};

pub async fn conn() -> postgres::PgConnection {
    println!("{}", dotenv!("POSTGRES_PASSWORD"));
    postgres::PgConnectOptions::new()
        .host(dotenv!("DB_HOST"))
        .port(dotenv!("POSTGRES_PORT").parse::<u16>().unwrap())
        .username(dotenv!("POSTGRES_USER"))
        .password(dotenv!("POSTGRES_PASSWORD"))
        .database(dotenv!("DATABASE_NAME"))
        .connect()
        .await
        .unwrap() // TODO: Handle Err() on failure to connect
}
