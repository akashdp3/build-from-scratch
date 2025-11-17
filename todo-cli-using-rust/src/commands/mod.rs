use clap::Subcommand;

pub mod export;
pub mod import;
pub mod task;

pub use task::Todo;

#[derive(Subcommand, Debug)]
pub enum Command {
    ADD { title: String },
    LIST,
    DONE { task_id: u8 },
    DELETE { task_id: u8 },
}
