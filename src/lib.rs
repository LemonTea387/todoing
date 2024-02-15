use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqliteRow, FromRow, Row, SqlitePool};

#[derive(Debug)]
pub enum TaskPriority {
    Raw(Vec<u8>),
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug)]
pub enum TaskStatus {
    Raw(Vec<u8>),
    Todo,
    Doing {
        date_commenced: DateTime<Utc>,
    },
    Done {
        date_commenced: DateTime<Utc>,
        date_done: DateTime<Utc>,
    },
}

pub struct Task {
    id: i64,
    pub title: String,
    pub date_created: DateTime<Utc>,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
}

impl Task {
    pub fn new(title: &str, description: Option<&str>, priority: TaskPriority) -> Self {
        Self {
            // ID does not matter now as this is likely just to be inserted
            id: 0,
            title: title.into(),
            date_created: chrono::offset::Utc::now(),
            description: description.map(|s| s.into()),
            status: TaskStatus::Todo,
            priority,
        }
    }

}

impl<'r> FromRow<'r, SqliteRow> for Task {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let date_created: chrono::DateTime<Utc> = row.try_get("created_dt")?;
        let description: Option<String> = row.try_get("description")?;
        let done: Vec<u8> = row.try_get("status")?;
        let priority: Vec<u8> = row.try_get("priority")?;
        Ok(Self {
            id,
            title,
            date_created,
            description,
            status: TaskStatus::Raw(done),
            priority: TaskPriority::Raw(priority),
        })
    }
}

pub enum DbError {}

pub async fn add_task(pool: &SqlitePool, task: Task) -> Result<i64, sqlx::Error> {
    let id = sqlx::query(
        r#"
    INSERT INTO tasks (id, title, description, status, priority)
    VALUES (NULL, ?, ?, ?, ?)
    "#,
    )
    .bind(task.title)
    .bind(task.description.unwrap_or("NULL".to_string()))
    .bind("unfinished".to_string())
    .bind("low".to_string())
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn list_tasks(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
    SELECT id, title, description, status, priority, created_dt
    FROM tasks
    ORDER BY DATETIME(created_dt)
        "#,
    )
    .fetch_all(pool)
    .await?;
    for task in tasks {
        println!(
            "Task {}: {}\nStatus : {:?}\nPriority : {:?}\nDate of Creation: {}\n{:?}",
            task.id, task.title, task.status, task.priority, task.date_created, task.description
        );
    }

    Ok(())
}
