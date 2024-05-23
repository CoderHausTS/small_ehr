extern crate dotenv;

use dotenv::dotenv;
use std::env;

use clap::{Parser, Subcommand, ValueEnum};

// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long)]
    // use a subcommand, that's an enum pointing to the 
    // file type and the file location
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(arg_required_else_help = false)]
    Import {
        #[arg(required = true)]
        file_type: FileTypes,
        location: String,
    },
    Run,
}

#[derive(ValueEnum, Subcommand, Clone)] 
enum FileTypes {
    Csv,
}


fn main() {
        dotenv().ok();
        
        // We can get our setup from a .env file. Nice for development!
        let db_url = env::var("DATABASE_URL")
                     .expect("DATABASE_URL must be set.");

        dbg!(db_url);

        // We should have command line tools callable
        // So we take command line args.
        // -- -i <file type> <file location>
        let cli = Cli::parse();

        match cli.commands {
            Commands::Import { file_type, location } => {
                match file_type {
                    FileTypes::Csv => {
                        println!("Git us a CSV");
                    }
                }
                println!("location is {}", location);
            },
            Commands::Run => {
                println!("Run this app.");
            }
        }

}
