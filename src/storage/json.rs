use crate::models::repository::TodoRepository;
use crate::models::todo::Todo;
use crate::models::value_objects::TodoId;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum JsonRepositoryError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for JsonRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::SerdeError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for JsonRepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            Self::SerdeError(e) => Some(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoDatabase {
    version: String,
    next_id: u32,
    todos: Vec<Todo>,
}

pub struct JsonTodoRepository {
    file_path: PathBuf,
}

impl JsonTodoRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn default() -> Result<Self, JsonRepositoryError> {
        let user_dirs = directories::UserDirs::new().ok_or_else(|| {
            JsonRepositoryError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        })?;

        let path = user_dirs.home_dir().join(".todo.json");
        Ok(Self::new(path))
    }

    fn load_database(&self) -> Result<TodoDatabase, JsonRepositoryError> {
        match fs::read_to_string(&self.file_path) {
            Ok(content) => serde_json::from_str(&content).map_err(JsonRepositoryError::SerdeError),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(TodoDatabase {
                version: "1.0".to_string(),
                next_id: 1,
                todos: Vec::new(),
            }),
            Err(e) => Err(JsonRepositoryError::IoError(e)),
        }
    }

    fn save_database(&self, db: &TodoDatabase) -> Result<(), JsonRepositoryError> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(JsonRepositoryError::IoError)?;
        }

        let json = serde_json::to_string_pretty(db).map_err(JsonRepositoryError::SerdeError)?;

        let temp_path = self.file_path.with_extension("tmp");

        fs::write(&temp_path, json).map_err(JsonRepositoryError::IoError)?;

        fs::rename(&temp_path, &self.file_path).map_err(JsonRepositoryError::IoError)?;

        Ok(())
    }
}

impl TodoRepository for JsonTodoRepository {
    type Error = JsonRepositoryError;

    fn find_all(&self) -> Result<Vec<Todo>, Self::Error> {
        let db = self.load_database()?;
        Ok(db.todos)
    }

    fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, Self::Error> {
        let db = self.load_database()?;
        Ok(db.todos.into_iter().find(|t| t.id.value() == id.value()))
    }

    fn save(&mut self, todo: Todo) -> Result<(), Self::Error> {
        let mut db = self.load_database()?;

        let todo_id = todo.id.value();
        match db.todos.iter().position(|t| t.id.value() == todo_id) {
            Some(pos) => db.todos[pos] = todo,
            None => db.todos.push(todo),
        }

        self.save_database(&db)?;
        Ok(())
    }

    fn delete(&mut self, id: &TodoId) -> Result<(), Self::Error> {
        let mut db = self.load_database()?;
        db.todos.retain(|t| t.id.value() != id.value());
        self.save_database(&db)?;
        Ok(())
    }

    fn next_id(&self) -> Result<u32, Self::Error> {
        let mut db = self.load_database()?;
        let id = db.next_id;
        db.next_id += 1;
        self.save_database(&db)?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_repository() -> (JsonTodoRepository, TempDir) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("test_todos.json");
        let repo = JsonTodoRepository::new(file_path);
        (repo, temp_dir)
    }

    #[test]
    fn test_empty_repository() {
        let (repo, _temp_dir) = create_test_repository();

        assert_eq!(repo.find_all().unwrap().len(), 0);
        assert_eq!(repo.next_id().unwrap(), 1);
    }

    #[test]
    fn test_save_and_find() {
        let (mut repo, _temp_dir) = create_test_repository();

        // 保存
        let todo = Todo::new(1, "テストタスク".to_string()).unwrap();
        repo.save(todo.clone()).unwrap();

        // ID検索
        let found = repo.find_by_id(&TodoId::new(1)).unwrap();
        assert!(found.is_some());

        // 存在しないID
        assert!(repo.find_by_id(&TodoId::new(999)).unwrap().is_none());
    }

    #[test]
    fn test_update_existing() {
        let (mut repo, _temp_dir) = create_test_repository();

        // 初回保存
        repo.save(Todo::new(1, "初期".to_string()).unwrap())
            .unwrap();

        // 更新
        repo.save(Todo::new(1, "更新後".to_string()).unwrap())
            .unwrap();

        // 件数は1件のまま
        assert_eq!(repo.find_all().unwrap().len(), 1);
    }

    #[test]
    fn test_delete() {
        let (mut repo, _temp_dir) = create_test_repository();

        // 3件保存
        for i in 1..=3 {
            repo.save(Todo::new(i, format!("タスク{}", i)).unwrap())
                .unwrap();
        }

        // 1件削除
        repo.delete(&TodoId::new(2)).unwrap();

        // 2件残存を確認
        let todos = repo.find_all().unwrap();
        assert_eq!(todos.len(), 2);
        assert!(!todos.iter().any(|t| t.id.value() == 2));
    }

    #[test]
    fn test_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("persist.json");

        // 保存
        {
            let mut repo = JsonTodoRepository::new(file_path.clone());
            repo.save(Todo::new(1, "永続化テスト".to_string()).unwrap())
                .unwrap();
        }

        // 別インスタンスで読み込み
        {
            let repo = JsonTodoRepository::new(file_path);
            assert_eq!(repo.find_all().unwrap().len(), 1);
        }
    }

    #[test]
    fn test_next_id_persistence() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("id_test.json");

        // ID採番
        let id1 = {
            let repo = JsonTodoRepository::new(file_path.clone());
            repo.next_id().unwrap()
        };

        // 別インスタンスで続きから採番
        let id2 = {
            let repo = JsonTodoRepository::new(file_path);
            repo.next_id().unwrap()
        };

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}
