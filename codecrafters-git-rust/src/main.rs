use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs;

pub(crate) mod commands;
pub(crate) mod objects;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Init,
    CatFile {
        #[arg(short = 'p')]
        pretty_print: bool,
        #[arg(short = 't')]
        file_type: bool,
        #[arg(short = 's')]
        size: bool,

        object_hash: String,
    },
    HashObject {
        #[arg(short = 'w')]
        write: bool,
        file_path: String,
    },
    LsTree {
        #[arg(long)]
        name_only: bool,
        object_hash: String,
    },
    WriteTree,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Command::Init => {
            fs::create_dir(".git").context("Failed to create .git folder")?;
            fs::create_dir(".git/objects").context("Failed to create .git/objects folder")?;
            fs::create_dir(".git/refs").context("Failed to create .git/refs folder")?;
            fs::write(".git/HEAD", "ref: refs/heads/main\n").context("Failed to create HEAD")?;
            println!("Initialized git repository");
        }
        Command::CatFile {
            pretty_print,
            file_type,
            size,
            object_hash,
        } => {
            commands::cat_file::invoke(pretty_print, file_type, size, &object_hash)?;
        }
        Command::HashObject { write, file_path } => {
            commands::hash_object::invoke(write, file_path)?;
        }
        Command::LsTree {
            name_only,
            object_hash,
        } => commands::ls_tree::invoke(name_only, &object_hash)?,
        Command::WriteTree => {
            print!("Result");
        }
    }

    Ok(())
}
