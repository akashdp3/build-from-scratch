use clap::Parser;

use crate::commands::{Command, Todo};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run() -> Self {
        Self::parse()
    }

    pub fn execute(&self) {
        let mut todo = Todo::new();

        match &self.command {
            Command::ADD { title } => match todo.add(title.as_str()) {
                Ok(_) => println!("Task added successfully!"),
                Err(e) => println!("Error: {}", e),
            },
            Command::LIST => match todo.list() {
                Ok(tasks) => println!("{}", tasks.join("\n")),
                Err(e) => println!("Error: {}", e),
            },
            Command::DONE { task_id } => match todo.change_status(*task_id, "done") {
                Ok(_) => println!("Task marked as done successfully!"),
                Err(e) => println!("Error: {}", e),
            },
            Command::DELETE { task_id } => match todo.delete(*task_id) {
                Ok(_) => println!("Task deleted successfully!"),
                Err(e) => println!("Error: {}", e),
            },
        }
    }
}
