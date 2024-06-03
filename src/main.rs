extern crate dotenv;

use dotenv::dotenv;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use clap::{Parser, Subcommand, ValueEnum};

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::allergies;

pub mod schema;
pub mod models;

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
pub fn csv_import(location: String, db_connection: &PgConnection) -> Result<(), io::Error> {
    // ask what table this data goes into. 
    //
    let mut input = String::new();
    
    print!("Please enter the table you would like to import in to: ");
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut input)?;

    // getting the file, or die
    let file = match File::open(location) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {:?}", error),
    };
   
    // This really checks if the inputted table exists
    match input.trim() {
        "allergies" => {
            println!("Be sure your Allergies csv contains the following fields:\r\n{:?}", allergies::table::all_columns());
            import_allergies(&file);
        }
        _ => {
            println!("That table does not exist. Check the name and try again.");
        }
    }
    // chec k how many fields in the file
    //

    // evaluate if the table and csv have the same number of fields

    // if we get here, we're OK to process the data, so send the file handle back
    Ok(()) 
}

fn import_allergies(file: &File) {
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
                            let file_handle = csv_import(location, &db_connection);
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

        csv_import(location, &db_connection);

    }

    #[test]
    fn csv_import_initialization_returns_failure() {
        let bad_location: String = "./csv/jimmy.joe".to_string();
        let db_connection: PgConnection = establish_connection();

        csv_import(bad_location, &db_connection);

    }
}
