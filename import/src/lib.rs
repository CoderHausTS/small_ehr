use database;

use csv::Reader;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use std::error::Error;
use std::process;

use clap::{Parser, Subcommand, ValueEnum};

use diesel::RunQueryDsl;

use database::models::{ NewAllergy, NewPatient, NewOrganization };
use database::schema::{ allergies, patients, organizations };

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    // #[arg(short, long)]
    // use a subcommand, that's an enum pointing to the 
    // file type and the file location
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum FileTypes {
    Csv,
}

#[derive(ValueEnum, Subcommand, Clone)]
pub enum Tables {
    Allergies,
    Patients,
    Organizations,
}

/// csv_import_intitialization(table:table, location) 
/// returns: result<>
/// this function 
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
        println!("csv file has the following fields: ");
        println!("{:?}", csv_headers);
    }

    // this really checks if the inputted table exists
    // this code needs to be updated when a new table is added
    match table {
        Tables::Allergies => {
            // println!("be sure the csv contains the following fields:\r\n{:?}", allergies::table::all_columns());

            match import_allergies(&mut csv_file) {
                Ok(()) => println!("import successful"),
                Err(e) => {
                    println!("error occured during import {}", e);
                    process::exit(1);
                }

            };
        }
        Tables::Patients => {
            // println!("be sure the csv contains the following fields:\r\n{:?}", patients::table::all_columns());

            match import_patients(&mut csv_file) {
                Ok(()) => println!("import successful"),
                Err(e) => {
                    println!("error occured during import {}", e);
                    process::exit(1);
                }

            };
        }
        Tables::Organizations => {
            // println!("be sure the csv contains the following fields:\r\n{:?}", patients::table::all_columns());

            match import_organizations(&mut csv_file) {
                Ok(()) => println!("import successful"),
                Err(e) => {
                    println!("error occured during import {}", e);
                    process::exit(1);
                }

            };
        }
        _ => {
            println!("that table does not exist. check the name and try again.");
        }
    }

    Ok(()) 
}

fn import_allergies(csv_file_reader: &mut Reader<&File>) -> Result<(), Box<dyn Error>>{

    let db_connection = &mut database::establish_connection();

   
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

    let db_connection = &mut database::establish_connection();

   
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

fn import_organizations(csv_file_reader: &mut Reader<&File>) -> Result<(), Box<dyn Error>>{

    let db_connection = &mut database::establish_connection();

   
    // get data from file
    //
    let mut import_count: usize = 0;
    for csv_record in csv_file_reader.deserialize::<NewOrganization>() { // csv_file_reader.records() {
        // if we have a record, push it into the DB
        match csv_record {
            Ok(record) => {
                let new_organization: NewOrganization = record;
                import_count += diesel::insert_into(organizations::table)
                    .values(&new_organization)
                    .execute(db_connection)
                    .expect("Error saving new post");
            },
            Err(err) => return Err(From::from(err)),
        }
    }
    println!("Imported {}", import_count);
  
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn csv_import_initialization_doesnt_die() {
        let location: String = "../csv/patients_cleaned.csv".to_string();

        assert!(csv_import(Tables::Patients, location).is_ok(), "CSV import finished" ); 

    }

    #[test]
    fn csv_import_initialization_fails() {
        let bad_location: String = "./csv/jimmy.joe".to_string();

        assert!(csv_import(Tables::Patients, bad_location).is_err(), "CSV file doesn't exist");
    }

    #[test]
    fn import_allergies_returns_nothing_on_success() {
        // we need N number of rows, not including header.

        // create a csv::Reader object to pass in
        //
        let data = b"
                    start, stop, patient_id, encounter_id, code, system, description, type, category,snomed 
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
        
        f.write_all(data);

        f.rewind(); 

        let mut rdr = csv::Reader::from_reader(&f);

        let import_result = match import_allergies(&mut rdr) {
            Ok(value) => dbg!(value),
            Err(e) => println!("Someerror {}", e),

        };

        // assert Ok()
        assert_eq!(import_result, ());
    }

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

    #[test]
    fn import_organizations_returns_nothing_on_success() {
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

        let import_result = match import_organizations(&mut rdr) {
            Ok(value) => dbg!(value),
            Err(e) => println!("Someerror {}", e),

        };

        // assert Ok()
        assert_eq!(import_result, ());
    }

}
