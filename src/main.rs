mod models;
mod sql_service;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");

    let mut db_client = sql_service::make_connection()
        .await
        .expect("Couldn't create database connection");

    let migrated = sql_service::do_migrations(&mut db_client).await.unwrap();

    println!("performed {} migrations", migrated)
}
