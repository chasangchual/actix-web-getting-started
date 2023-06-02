use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_URL: &str = "sqlite://otac.db";

pub async fn init_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("creating a new database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("success"),
            Err(e) => panic!("error: {} while creating DB {}", e, DB_URL)
        }
    } else {
        println!("database {} exists.", DB_URL);
    }

    init_tables().await;
}

pub async fn init_tables() {

}