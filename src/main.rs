use anyhow::Result;
use clap::{Parser, Subcommand};
use rust_todo_cli::{commands, storage::json::JsonTodoRepository};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
    },
    List {
        #[arg(short, long)]
        all: bool,
    },
    Done {
        id: u32,
    },
    Delete {
        id: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut repo = JsonTodoRepository::default()?;

    match cli.command {
        Commands::Add { title } => commands::add::execute(&mut repo, title)?,
        Commands::List { all } => commands::list::execute(&repo, all)?,
        Commands::Done { id } => commands::done::execute(&mut repo, id)?,
        Commands::Delete { id } => commands::delete::execute(&mut repo, id)?,
    }

    Ok(())
}
