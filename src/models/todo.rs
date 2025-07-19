use super::errors::{DomainError, ValidationError};
use super::value_objects::{NonEmptyString, TodoId};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TodoStatus {
    Active,
    Completed { completed_at: DateTime<Local> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: TodoId,
    pub title: NonEmptyString,
    pub status: TodoStatus,
    pub created_at: DateTime<Local>,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Result<Self, ValidationError> {
        Ok(Todo {
            id: TodoId::new(id),
            title: NonEmptyString::new(title)?,
            status: TodoStatus::Active,
            created_at: Local::now(),
        })
    }

    pub fn complete(self) -> Result<Self, DomainError> {
        match self.status {
            TodoStatus::Active => Ok(Todo {
                status: TodoStatus::Completed {
                    completed_at: Local::now(),
                },
                ..self
            }),
            TodoStatus::Completed { .. } => Err(DomainError::AlreadyCompleted),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_new_creates_active_todo() {
        let todo = Todo::new(1, "新しいタスク".to_string()).unwrap();

        assert_eq!(todo.id.value(), 1);
        assert_eq!(todo.title.as_str(), "新しいタスク");
        assert!(matches!(todo.status, TodoStatus::Active));
    }

    #[test]
    fn test_todo_new_rejects_empty_title() {
        let result = Todo::new(1, "".to_string());

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyTitle));
    }

    #[test]
    fn test_todo_complete_transitions_from_active() {
        let todo = Todo::new(1, "タスク".to_string()).unwrap();
        let before = Local::now();

        let completed = todo.complete().unwrap();

        match completed.status {
            TodoStatus::Completed { completed_at } => {
                assert!(completed_at >= before);
                assert!(completed_at <= Local::now());
            }
            _ => panic!("Expected Completed status"),
        }
    }

    #[test]
    fn test_todo_complete_fails_when_already_completed() {
        let todo = Todo::new(1, "タスク".to_string()).unwrap();
        let completed = todo.complete().unwrap();

        let result = completed.complete();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::AlreadyCompleted));
    }

    #[test]
    fn test_todo_preserves_other_fields_on_complete() {
        let todo = Todo::new(42, "重要なタスク".to_string()).unwrap();
        let created_at = todo.created_at;

        let completed = todo.complete().unwrap();

        assert_eq!(completed.id.value(), 42);
        assert_eq!(completed.title.as_str(), "重要なタスク");
        assert_eq!(completed.created_at, created_at);
    }
}
