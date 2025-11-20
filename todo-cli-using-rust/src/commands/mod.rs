use clap::Subcommand;

pub mod export;
pub mod import;
pub mod todo;

pub use todo::Todo;

#[derive(Subcommand, Debug)]
pub enum Command {
    Add { title: String },
    List,
    Done { task_id: u8 },
    Delete { task_id: u8 },
}
