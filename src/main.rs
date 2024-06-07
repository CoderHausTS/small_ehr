extern crate dotenv;

use csv::Reader;
use dotenv::dotenv;

use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::Cursor;
use std::error::Error;
use std::io::prelude::*;

use clap::{Parser, Subcommand, ValueEnum};

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::Allergy;
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
        table: Tables,
        location: String,
    },
    Run,
}

#[derive(ValueEnum, Subcommand, Clone)] 
enum FileTypes {
    Csv,
}

#[derive(ValueEnum, Subcommand, Clone)]
enum Tables {
    Allergies,
}

pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// csv_import_intitialization(table:Table, location:String, db_connection: PgConnection) 
/// returns: Result<>
/// This function 
///     takes in the location of  csv file for import,
///     asks what table the data goes into (csv we are assuming have a specific set of data)
///     opens the file
///     checks the csv to ensure it has the same number of fields as the table
/// 
/// at any point it could return a result, or an error
///
pub fn csv_import(table: Tables, location: String, db_connection: &PgConnection) -> io::Result<()> {
    
    // getting the file, or die
    let file = File::open(location)?;
    let mut csv_file = csv::Reader::from_reader(&file);
   
    {
    // get the headers to print out with the table info.
        let csv_headers = csv_file.headers()?;
        println!("CSV file has the following fields: ");
        println!("{:?}", csv_headers);
    }

    // This really checks if the inputted table exists
    match table {
        Tables::Allergies => {
            println!("Be sure the csv contains the following fields:\r\n{:?}", allergies::table::all_columns());
            import_allergies(&mut csv_file);
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

fn import_allergies(csv_file_reader: &mut Reader<&File>) -> Result<i32, Box<dyn Error>>{
    let num_rows: i32 = 0;
   
    // get data from file
    //
    for csv_record in csv_file_reader.records() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record = csv_record?;
        println!("The record is {:?}", record);

        // push data into allergies table
        //
 
    }
                
    // verify number of records written equals number of records in file
    //

    // Return number of rows written
    Ok(num_rows)
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
            Commands::Import { file_type, table, location } => {
                match file_type {
                    FileTypes::Csv => {

                            let file_handle = csv_import(table, location, &db_connection);
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
    fn csv_import_initialization_doesnt_die() {
        let location: String = "./csv/allergies.csv".to_string();
        let db_connection: PgConnection = establish_connection();

        assert!(csv_import(Tables::Allergies, location, &db_connection).is_ok(), "CSV import finished" ); 

    }

    #[test]
    fn csv_import_initialization_fails() {
        let bad_location: String = "./csv/jimmy.joe".to_string();
        let db_connection: PgConnection = establish_connection();

        assert!(csv_import(Tables::Allergies, bad_location, &db_connection).is_err(), "CSV file doesn't exist");
    }

    #[test]
    fn import_allergies_returns_num_records_imported() {
        // we need N number of rows, not including header.
        let num_rows = 3;

        // create a csv::Reader object to pass in
        //
        let data = b"
                    START, STOP, PATIENT_ID, ENCOUNTER_ID, CODE, SYSTEM, DESCRIPTION, TYPE, CATEGORY, SNOMED
                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,111088007,Unknown,Latex (substance),allergy,environment,247472004
                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,84489001,Unknown,Mold (organism),allergy,environment,76067001
                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,260147004,Unknown,House dust mite (organism),allergy,environment,
                    ";

        let dir = env::temp_dir();

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(dir.as_path().join("foo.txt"))
            .unwrap();
        
        let output =  f.write_all(data); 

        f.rewind(); 

        let mut rdr = csv::Reader::from_reader(&f);

        let import_result = import_allergies(&mut rdr);

        // assert Ok()
        assert_eq!(import_result.unwrap(), num_rows);
        }
}
