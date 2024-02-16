use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use todoing::{
    database::{add_task, list_tasks},
    task::{Task, TaskPriority},
};

const DB_URL: &str = "sqlite://todoing.db";

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    if !Sqlite::database_exists(DB_URL).await? {
        Sqlite::create_database(DB_URL)
            .await
            .expect("Could not create database.");
    }

    let todo_db = SqlitePool::connect(DB_URL)
        .await
        .expect("Could not connect to database.");

    sqlx::migrate!().run(&todo_db).await?;

    println!("Adding Task: ");
    add_task(
        &todo_db,
        Task::new("Test1", Some("Hello pulis"), TaskPriority::Low),
    )
    .await?;

    println!("Listing Task: ");
    list_tasks(&todo_db).await?;

    Ok(())
}
