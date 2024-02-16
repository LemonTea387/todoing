use chrono::{DateTime, Utc};
#[derive(Debug, sqlx::Type)]
#[repr(i32)]
pub enum TaskPriority {
    Low = 1,
    Medium,
    High,
    Urgent,
}

#[derive(Debug)]
pub enum TaskStatus {
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
    pub id: i64,
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
