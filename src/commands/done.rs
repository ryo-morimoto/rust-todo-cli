use crate::models::{repository::TodoRepository, value_objects::TodoId};
use anyhow::Result;

pub fn execute<R: TodoRepository>(repo: &mut R, id: u32) -> Result<()> {
    let todo_id = TodoId::new(id);

    match repo.find_by_id(&todo_id)? {
        Some(todo) => {
            let completed = todo.complete()?;
            repo.save(completed)?;
            println!("Task completed! (ID: {})", id);
            Ok(())
        }
        None => anyhow::bail!("No task found for ID {}", id),
    }
}
