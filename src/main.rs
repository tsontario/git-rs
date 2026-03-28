use clap::{Parser, Subcommand};
use my_git::commands;
use my_git::commands::CliConfig;

#[derive(Parser)]
#[command(version="0.1")]
struct Cli {
    #[arg(short = 'C', default_value = ".")]
    work_dir: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { },
    HashObject(commands::hash_object::HashObjectArgs),
    CatFile(commands::cat_file::CatFileArgs),
}

fn main() {
    let cli = Cli::parse();
    let config = CliConfig::build(cli.work_dir);

    match &cli.command {
        Commands::Init {} => {
            let dir = std::path::Path::new(config.work_dir.as_str());
            match commands::init::call(dir) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::HashObject(args) => {
            match commands::hash_object::call(&config, args) {
                Ok(obj_hash) => println!("{}", obj_hash.hash),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::CatFile(args) => {
            match commands::cat_file::call(&config, args) {
                Ok(response) => {  println!("{}", response) }
                Err(e) => eprintln!("Error: {}", e),
            }
        },
    }
}