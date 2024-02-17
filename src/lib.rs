use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub mod database;
pub mod task;

pub struct TaskManager {
    pub db_connection: SqlitePool,
}

impl TaskManager {
    pub async fn init(db_url: &str) -> Result<Self, TaskError> {
        if !Sqlite::database_exists(db_url).await? {
            Sqlite::create_database(db_url)
                .await
                .expect("Could not create database.");
        }
        let db_connection = SqlitePool::connect(db_url)
            .await
            .expect("Could not connect to database.");
        sqlx::migrate!().run(&db_connection).await?;
        Ok(Self { db_connection })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Database Migration error!")]
    DatabaseMigration(#[from] sqlx::migrate::MigrateError),
    #[error("Database error!")]
    Database(#[from] sqlx::Error),
}
