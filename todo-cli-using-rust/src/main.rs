use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    println!("Command: {}", args.command);
}
