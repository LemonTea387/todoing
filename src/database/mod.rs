use std::{error::Error, fmt::Display};

use chrono::{NaiveDate, Utc};
use sqlx::{sqlite::SqliteRow, FromRow, QueryBuilder, Row, Sqlite, SqlitePool};

const STATE_TODO: i32 = 0;
const STATE_DOING: i32 = 1;
const STATE_DONE: i32 = 2;

use crate::task::{Task, TaskPriority, TaskStatus};
impl<'r> FromRow<'r, SqliteRow> for Task {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let date_created: chrono::DateTime<Utc> = row.try_get("created_dt")?;
        let description: Option<String> = row.try_get("description")?;
        let status_val: i32 = row.try_get("status")?;
        let status: TaskStatus = match status_val {
            STATE_TODO => Ok(TaskStatus::Todo),
            STATE_DOING => Ok(TaskStatus::Doing {
                date_commenced: row.try_get("started_dt")?,
            }),
            STATE_DONE => Ok(TaskStatus::Done {
                date_commenced: row.try_get("started_dt")?,
                date_done: row.try_get("ended_dt")?,
            }),
            _ => Err(sqlx::Error::Decode(Box::new(TaskDbError::ParsingError))),
        }?;
        let priority: TaskPriority = row.try_get("priority")?;
        Ok(Self {
            id,
            title,
            date_created,
            description,
            status,
            priority,
        })
    }
}

#[derive(Debug)]
pub enum TaskDbError {
    ParsingError,
}

impl Error for TaskDbError {}

impl Display for TaskDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub async fn add_task(pool: &SqlitePool, task: Task) -> Result<i64, sqlx::Error> {
    let id = sqlx::query(
        r#"
    INSERT INTO tasks (id, title, description, priority)
    VALUES (NULL, ?, ?, ?)
    "#,
    )
    .bind(task.title)
    .bind(task.description.unwrap_or("NULL".to_string()))
    .bind(task.priority as i32)
    .execute(pool)
    .await?
    .last_insert_rowid();
    sqlx::query(
        r#"
        INSERT INTO tasks_activity (task_id, status)
        VALUES (?, ?)
        "#,
    )
    .bind(id)
    .bind(STATE_TODO)
    .execute(pool)
    .await?;
    Ok(id)
}

pub async fn list_tasks(pool: &SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"
    SELECT id, title, description, priority, created_dt, status, started_dt, ended_dt
    FROM tasks
    INNER JOIN tasks_activity ON tasks.id = tasks_activity.task_id
    ORDER BY DATETIME(created_dt)
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(tasks)
}

#[derive(Debug, Default)]
pub struct TaskFilterQuery {
    pub day: Option<NaiveDate>,
    pub priority: Option<TaskPriority>,
}

pub async fn list_tasks_filter(
    pool: &SqlitePool,
    filter: &TaskFilterQuery,
) -> Result<Vec<Task>, sqlx::Error> {
    let mut query = QueryBuilder::<Sqlite>::new(
        r#"
    SELECT id, title, description, priority, created_dt, status, started_dt, ended_dt
    FROM tasks 
    INNER JOIN tasks_activity ON tasks.id = tasks_activity.task_id 
    WHERE 1=1
        "#,
    );
    if let Some(date) = filter.day {
        query.push(" AND DATE(created_dt) = DATE(");
        query.push_bind(date);
        query.push(")");
    }
    if let Some(priority) = filter.priority {
        query.push(" AND priority = ");
        query.push_bind(priority as i32);
    }
    query.push("ORDER BY DATETIME(created_dt)");
    let tasks = query.build_query_as::<Task>().fetch_all(pool).await?;
    Ok(tasks)
}
