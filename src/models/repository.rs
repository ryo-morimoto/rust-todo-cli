use super::todo::Todo;
use super::value_objects::TodoId;

pub trait TodoRepository {
    type Error: std::error::Error + Send + Sync + 'static;

    fn find_all(&self) -> Result<Vec<Todo>, Self::Error>;

    fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, Self::Error>;

    fn save(&mut self, todo: Todo) -> Result<(), Self::Error>;

    fn delete(&mut self, id: &TodoId) -> Result<(), Self::Error>;

    fn next_id(&self) -> Result<u32, Self::Error>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub struct MockTodoRepository {
        todos: Arc<Mutex<HashMap<u32, Todo>>>,
        next_id: Arc<Mutex<u32>>,
    }

    impl MockTodoRepository {
        pub fn new() -> Self {
            Self {
                todos: Arc::new(Mutex::new(HashMap::new())),
                next_id: Arc::new(Mutex::new(1)),
            }
        }
    }

    #[derive(Debug)]
    pub struct MockError(String);

    impl std::fmt::Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock Repository error: {}", self.0)
        }
    }

    impl std::error::Error for MockError {}

    impl TodoRepository for MockTodoRepository {
        type Error = MockError;

        fn find_all(&self) -> Result<Vec<Todo>, Self::Error> {
            let todos = self.todos.lock().unwrap();
            Ok(todos.values().cloned().collect())
        }

        fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, Self::Error> {
            let todos = self.todos.lock().unwrap();
            Ok(todos.get(&id.value()).cloned())
        }

        fn save(&mut self, todo: Todo) -> Result<(), Self::Error> {
            let mut todos = self.todos.lock().unwrap();
            todos.insert(todo.id.value(), todo);
            Ok(())
        }

        fn delete(&mut self, id: &TodoId) -> Result<(), Self::Error> {
            let mut todos = self.todos.lock().unwrap();
            todos.remove(&id.value());
            Ok(())
        }

        fn next_id(&self) -> Result<u32, Self::Error> {
            let mut next_id = self.next_id.lock().unwrap();
            let id = *next_id;
            *next_id += 1;
            Ok(id)
        }
    }
}
