use chrono::DateTime;

pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

pub enum TaskStatus {
    Todo,
    Doing {
        date_commenced: DateTime,
    },
    Done {
        date_commenced: DateTime,
        date_done: DateTime,
    },
}

pub struct Task {
    id: u32,
    pub title: String,
    pub date_created: DateTime,
    pub description: Option<String>,
    pub done: TaskStatus,
    pub priority: TaskPriority,
}
