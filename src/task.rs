use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub done: bool,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Task {
    pub fn new(id: usize, title: &str, priority: Priority) -> Self {
        Self {
            id,
            title: title.to_string(),
            done: false,
            priority,
            created_at: Utc::now(),
            completed_at: None,
        }
    }

    /// Mark the task as complete.
    pub fn complete(&mut self) {
        self.done = true;
    }
}
