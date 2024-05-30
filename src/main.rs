extern crate dotenv;

use dotenv::dotenv;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use clap::{Parser, Subcommand, ValueEnum};

use diesel::pg::PgConnection;
use diesel::prelude::*;

// used for testing file work

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

pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// csv_import_intitialization(location:String, db_connection: PgConnection) 
/// returns: Result<FIle>
/// This function 
///     takes in the location of  csv file for import,
///     asks what table the data goes into (csv we are assuming have a specific set of data)
///     opens the file
///     checks the csv to ensure it has the same number of fields as the table
/// 
/// at any point it could return a result, or an error
///
/// let location = ./csv/allergies.csv;
/// let file_handle = csv_import_initialization(location)?;
/// ```
///
pub fn csv_import_initialization(location: String, db_connection: &PgConnection) -> std::io::Result<File> {
    // ask what table this data goes into. 
    //

    // Get the table metadata to see how many fields
    // 

    let file = File::open(location)?;

    // check how many fields in the file
    //

    // evaluate if the table and csv have the same number of fields

    // if we get here, we're OK to process the data, so send the file handle back
    Ok(file) 
}

fn main() {
        dotenv().ok();
        
        // We can get our setup from a .env file. Nice for development!
        // let db_url = env::var("DATABASE_URL")
        //             .expect("DATABASE_URL must be set.");

        //dbg!(db_url);
        let db_connection: PgConnection = establish_connection();

        // We should have command line tools callable
        // So we take command line args.
        // -- -i <file type> <file location>
        let cli = Cli::parse();

        match cli.commands {
            Commands::Import { file_type, location } => {
                match file_type {
                    FileTypes::Csv => {
                            let file_handle = csv_import_initialization(location, &db_connection);
                    }
                }
            },
            Commands::Run => {
                println!("Run this app.");
            }
        }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_import_initialization_returns_file_handle() {
        let location: String = "./csv/allergies.csv".to_string();
        let db_connection: PgConnection = establish_connection();

        let file_handle = csv_import_initialization(location, &db_connection);

        assert!(!file_handle.is_err())

    }

    #[test]
    fn csv_import_initialization_returns_failure() {
        let bad_location: String = "./csv/jimmy.joe".to_string();
        let db_connection: PgConnection = establish_connection();

        let file_handle = csv_import_initialization(bad_location, &db_connection);

        assert!(file_handle.is_err());
    }
}
