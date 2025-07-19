use core::fmt;

#[derive(Debug, Clone)]
pub enum ValidationError {
    EmptyTitle,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyTitle => write!(f, "The title is empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone)]
pub enum DomainError {
    ValidationError(ValidationError),
    AlreadyCompleted,
    InvalidStateTransition,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::ValidationError(e) => write!(f, "Validation error: {}", e),
            DomainError::AlreadyCompleted => write!(f, "Already completed"),
            DomainError::InvalidStateTransition => write!(f, "Invalid state transition"),
        }
    }
}

impl std::error::Error for DomainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DomainError::ValidationError(e) => Some(e),
            _ => None,
        }
    }
}
