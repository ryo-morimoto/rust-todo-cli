use crate::models::{repository::TodoRepository, todo::Todo};
use anyhow::Result;

pub fn execute<R: TodoRepository>(repo: &mut R, title: String) -> Result<()> {
    let id = repo.next_id()?;
    let todo = Todo::new(id, title)?;
    repo.save(todo)?;
    println!("Task Added! (ID: {})", id);
    Ok(())
}
