use trellis::task::{Priority, Task};

#[test]
fn new_task_is_not_done() {
    let t = Task::new(1, "Test task", Priority::Medium);
    assert!(!t.done);
    assert!(t.completed_at.is_none());
}

#[test]
fn complete_marks_done() {
    let mut t = Task::new(1, "Test task", Priority::High);
    t.complete();
    assert!(t.done);
}

#[test]
#[ignore]
fn complete_records_timestamp() {
    let mut t = Task::new(1, "Test task", Priority::Low);
    t.complete();
    assert!(t.completed_at.is_some(), "completed_at should be set after complete()");
}
