use std::fs;
use std::path::PathBuf;

use crate::task::{Priority, Task};

const STORE_FILE: &str = "trellis.json";

#[derive(Debug)]
pub struct Store {
    tasks: Vec<Task>,
    next_id: usize,
}

impl Store {
    /// Load tasks from disk, or create an empty store.
    pub fn load() -> Result<Self, String> {
        let path = store_path();
        if !path.exists() {
            return Ok(Self {
                tasks: Vec::new(),
                next_id: 1,
            });
        }

        let data = fs::read_to_string(&path)
            .map_err(|e| format!("read {}: {e}", path.display()))?;

        let tasks: Vec<Task> =
            serde_json::from_str(&data).map_err(|e| format!("parse: {e}"))?;

        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Ok(Self { tasks, next_id })
    }

    /// Persist tasks to disk.
    pub fn save(&self) -> Result<(), String> {
        let path = store_path();
        let data = serde_json::to_string_pretty(&self.tasks)
            .map_err(|e| format!("serialize: {e}"))?;
        fs::write(&path, data)
            .map_err(|e| format!("write {}: {e}", path.display()))
    }

    /// Add a task and return its ID.
    pub fn add(&mut self, title: &str, priority: Priority) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task::new(id, title, priority));
        id
    }

    /// List all tasks, sorted by priority (high first).
    pub fn list(&self) -> Vec<&Task> {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by_key(|t| match t.priority {
            Priority::High => 0,
            Priority::Medium => 1,
            Priority::Low => 2,
        });
        sorted
    }

    /// Mark a task as complete. Returns false if not found.
    pub fn complete(&mut self, id: usize) -> bool {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.complete();
            true
        } else {
            false
        }
    }

    /// Remove a task. Returns false if not found.
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < before
    }

    /// Search tasks by keyword in their title.
    #[allow(dead_code)]
    pub fn search(&self, query: &str) -> Vec<&Task> {
        let q = query.to_lowercase();
        self.tasks
            .iter()
            .filter(|t| t.title.to_lowercase().contains(&q))
            .collect()
    }
}

fn store_path() -> PathBuf {
    PathBuf::from(STORE_FILE)
}
