use crate::models::{repository::TodoRepository, todo::TodoStatus};
use anyhow::Result;
use colored::Colorize;

pub fn execute<R: TodoRepository>(repo: &R, all: bool) -> Result<()> {
    let todos = repo.find_all()?;

    if todos.is_empty() {
        println!("No tasks");
        return Ok(());
    }

    // Header
    println!("{}", "=== TODO List ===".bold());
    println!();

    let active_todos: Vec<_> = todos
        .iter()
        .filter(|t| matches!(t.status, TodoStatus::Active))
        .collect();

    if !active_todos.is_empty() {
        println!("{}", "📋 Active Tasks:".yellow());
        for todo in active_todos {
            println!(
                "  {} - {}",
                format!("[{}]", todo.id.value()).cyan(),
                todo.title.as_str()
            );
        }
        println!();
    }

    if all {
        let completed_todos: Vec<_> = todos
            .iter()
            .filter(|t| matches!(t.status, TodoStatus::Completed { .. }))
            .collect();

        if !completed_todos.is_empty() {
            println!("{}", "✅ Completed Tasks:".green());
            for todo in completed_todos {
                println!(
                    "  {} - {} {}",
                    format!("[{}]", todo.id.value()).cyan(),
                    todo.title.as_str().strikethrough(),
                    "✓".green()
                );
            }
        }
    }

    let active_count = todos
        .iter()
        .filter(|t| matches!(t.status, TodoStatus::Active))
        .count();
    let completed_count = todos
        .iter()
        .filter(|t| matches!(t.status, TodoStatus::Completed { .. }))
        .count();

    println!();
    println!(
        "{}",
        format!(
            "Total: {} tasks ({} active, {} completed)",
            todos.len(),
            active_count,
            completed_count
        )
        .dimmed()
    );

    Ok(())
}
