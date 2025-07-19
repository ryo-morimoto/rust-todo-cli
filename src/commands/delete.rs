use crate::models::{repository::TodoRepository, value_objects::TodoId};
use anyhow::Result;

pub fn execute<R: TodoRepository>(repo: &mut R, id: u32) -> Result<()> {
    let todo_id = TodoId::new(id);
    repo.delete(&todo_id)?;
    println!("Task deleted! (ID {})", id);
    Ok(())
}
