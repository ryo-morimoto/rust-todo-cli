use super::errors::ValidationError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(s: String) -> Result<Self, ValidationError> {
        if s.trim().is_empty() {
            Err(ValidationError::EmptyTitle)
        } else {
            Ok(NonEmptyString(s))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TodoId(u32);

impl TodoId {
    pub fn new(id: u32) -> Self {
        TodoId(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string_accepts_valid_string() {
        let result = NonEmptyString::new("有効なタイトル".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "有効なタイトル");
    }

    #[test]
    fn test_non_empty_string_rejects_empty_string() {
        let result = NonEmptyString::new("".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyTitle));
    }

    #[test]
    fn test_non_empty_string_rejects_whitespace_only() {
        let result = NonEmptyString::new("   ".to_string());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ValidationError::EmptyTitle));
    }

    #[test]
    fn test_todo_id_creation_and_value() {
        let id = TodoId::new(42);
        assert_eq!(id.value(), 42);
    }
}
