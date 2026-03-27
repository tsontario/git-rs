use std::env::current_dir;
use clap::{Parser, Subcommand};
use my_git::commands;

#[derive(Parser)]
#[command(version="0.1")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { },
    HashObject(commands::hash_object::HashObjectArgs)
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            let current_dir = current_dir().expect("Should be able to get current directory");
            match commands::init::call(current_dir.as_path()) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::HashObject(args) => {
            match commands::hash_object::call(args) {
                Ok(obj_hash) => println!("{}", obj_hash.hash),
                Err(e) => eprintln!("Error: {}", e),
            }
        },

    }
}
