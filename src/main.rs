extern crate dotenv;

use csv::Reader;
use dotenv::dotenv;

use std::env;
use std::fs::File;
use std::io;
// use std::io::Cursor;
use std::error::Error;
use std::io::prelude::*;
use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{ Allergy, NewAllergy, Patient, NewPatient };
use crate::schema::{ allergies, patients };

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
pub enum Tables {
    Allergies,
    Patients,
}

pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// csv_import_intitialization(table:Table, location) 
/// returns: Result<>
/// This function 
///     takes in the location of  csv file for import,
///     asks what table the data goes into (csv we are assuming have a specific set of data)
///     opens the file
///     checks the csv to ensure it has the same number of fields as the table
/// 
/// at any point it could return a result, or an error
///
pub fn csv_import(table: Tables, location: String) -> io::Result<()> {

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
    // This code needs to be updated when a new table is added
    match table {
        Tables::Allergies => {
            // println!("Be sure the csv contains the following fields:\r\n{:?}", allergies::table::all_columns());

            match import_allergies(&mut csv_file) {
                Ok(()) => println!("Import successful"),
                Err(e) => {
                    println!("Error occured during import {}", e);
                    process::exit(1);
                }

            };
        }
        Tables::Patients => {
            // println!("Be sure the csv contains the following fields:\r\n{:?}", patients::table::all_columns());

            match import_patients(&mut csv_file) {
                Ok(()) => println!("Import successful"),
                Err(e) => {
                    println!("Error occured during import {}", e);
                    process::exit(1);
                }

            };
        }
        _ => {
            println!("That table does not exist. Check the name and try again.");
        }
    }

    Ok(()) 
}

fn import_allergies(csv_file_reader: &mut Reader<&File>) -> Result<(), Box<dyn Error>>{

    let db_connection = &mut establish_connection();

   
    // get data from file
    //
    let mut import_count: usize = 0;
    for csv_record in csv_file_reader.deserialize::<NewAllergy>() { // csv_file_reader.records() {
        // if we have a record, push it into the DB
        match csv_record {
            Ok(record) => {
                let new_allergy: NewAllergy = record;
                import_count += diesel::insert_into(allergies::table)
                    .values(&new_allergy)
                    // .returning(Allergy::as_returning())
                    .execute(db_connection)
                    .expect("Error saving new post");
            },
            Err(err) => return Err(From::from(err)),
        }
    }
    println!("Imported {}", import_count);
  
    Ok(())
}

fn import_patients(csv_file_reader: &mut Reader<&File>) -> Result<(), Box<dyn Error>>{

    let db_connection = &mut establish_connection();

   
    // get data from file
    //
    let mut import_count: usize = 0;
    for csv_record in csv_file_reader.deserialize::<NewPatient>() { // csv_file_reader.records() {
        // if we have a record, push it into the DB
        match csv_record {
            Ok(record) => {
                let new_patient: NewPatient = record;
                import_count += diesel::insert_into(patients::table)
                    .values(&new_patient)
                    // .returning(Allergy::as_returning())
                    .execute(db_connection)
                    .expect("Error saving new post");
            },
            Err(err) => return Err(From::from(err)),
        }
    }
    println!("Imported {}", import_count);
  
    Ok(())
}

fn main() {
        dotenv().ok();
        
        // We can get our setup from a .env file. Nice for development!
        // let db_url = env::var("DATABASE_URL")
        //             .expect("DATABASE_URL must be set.");

        let db_connection: PgConnection = establish_connection();

        // We should have command line tools callable
        // So we take command line args.
        // -- -i <file type> <file location>
        let cli = Cli::parse();

        match cli.commands {
            Commands::Import { file_type, table, location } => {
                match file_type {
                    FileTypes::Csv => {
                        match  csv_import(table, location) {
                            Ok(()) => process::exit(0),
                            Err(e) => process::exit(1),
                        }

                     //       let file_handle = csv_import(table, location);
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

//    #[test]
//    fn csv_import_initialization_doesnt_die() {
//        let location: String = "./csv/allergies_cleaned.csv".to_string();
//
//        assert!(csv_import(Tables::Allergies, location).is_ok(), "CSV import finished" ); 
//
//    }
//
//    #[test]
//    fn csv_import_initialization_fails() {
//        let bad_location: String = "./csv/jimmy.joe".to_string();
//
//        assert!(csv_import(Tables::Allergies, bad_location).is_err(), "CSV file doesn't exist");
//    }

//    #[test]
//    fn import_allergies_returns_nothing_on_success() {
//        // we need N number of rows, not including header.
//
//        // create a csv::Reader object to pass in
//        //
//        let data = b"
//                    start, stop, patient_id, encounter_id, code, system, description, type, category,snomed 
//                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,111088007,Unknown,Latex (substance),allergy,environment,247472004
//                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,84489001,Unknown,Mold (organism),allergy,environment,76067001
//                    2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,260147004,Unknown,House dust mite (organism),allergy,environment,
//                    ";
//
//        let dir = env::temp_dir();
//
//        let mut f = std::fs::OpenOptions::new()
//            .create(true)
//            .write(true)
//            .read(true)
//            .open(dir.as_path().join("foo.txt"))
//            .unwrap();
//        
//        f.write_all(data); 
//
//        f.rewind(); 
//
//        let mut rdr = csv::Reader::from_reader(&f);
//
//        let import_result = match import_allergies(&mut rdr) {
//            Ok(value) => dbg!(value),
//            Err(e) => println!("Someerror {}", e),
//
//        };
//
//        // assert Ok()
//        assert_eq!(import_result, ());
//    }

    #[test]
    fn import_patients_returns_nothing_on_success() {
        // we need N number of rows, not including header.

        // create a csv::Reader object to pass in
        //
        let data = b"
        BIRTHDATE,DEATHDATE,SSN,DRIVERS,PASSPORT,PREFIX,FIRST,LAST,SUFFIX,MAIDEN,MARITAL,RACE,ETHNICITY,GENDER,BIRTHPLACE,ADDRESS,CITY,STATE,COUNTY,ZIP,LAT,LON,HEALTHCARE_EXPENSES,HEALTHCARE_COVERAGE
        2019-02-17,,999-65-3251,,,,Damon455,Langosh790,,,,white,nonhispanic,M,Middleborough  Massachusetts  US,620 Lynch Tunnel Apt 0,Springfield,Massachusetts,Hampden County,01104,42.08038942501558,-72.48043144917739,9039.1645,7964.1255
        2005-07-04,,999-49-3323,S99941126,,,Thi53,Wunsch504,,,,white,nonhispanic,F,Danvers  Massachusetts  US,972 Tillman Branch Suite 48,Bellingham,Massachusetts,Norfolk County,,42.03521335752818,-71.48251904737748,402723.415,14064.135000000002
        1998-05-11,,999-10-8743,S99996708,X75063318X,Mr.,Chi716,Greenfelder433,,,,white,nonhispanic,M,Athens  Athens Prefecture  GR,1060 Bernhard Crossroad Suite 15,Boston,Massachusetts,Suffolk County,02131,42.29255662362827,-71.06116042204106,571935.8725,787.5374999999999
                    ";

        let dir = env::temp_dir();

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(dir.as_path().join("foo.txt"))
            .unwrap();
        
        f.write_all(data); 

        f.rewind(); 

        let mut rdr = csv::Reader::from_reader(&f);

        let import_result = match import_patients(&mut rdr) {
            Ok(value) => dbg!(value),
            Err(e) => println!("Someerror {}", e),

        };

        // assert Ok()
        assert_eq!(import_result, ());
    }

}
