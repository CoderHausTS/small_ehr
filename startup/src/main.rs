extern crate dotenv;

// use database::establish_connection;
use import::{ csv_import, Cli, Commands, FileTypes };
use rest_json_api::start_rest_api;

use dotenv::dotenv;

use std::process;

use clap::Parser; //, Subcommand, ValueEnum};

// use diesel::pg::PgConnection;
#[tokio::main]
async fn main() {
        dotenv().ok();
        
        // We can get our setup from a .env file. Nice for development!
        // let db_url = env::var("DATABASE_URL")
        //             .expect("DATABASE_URL must be set.");

//         let db_connection: PgConnection = establish_connection();

        // We should have command line tools callable
        // So we take command line args.
        // -- -i <file type> <file location>
        let cli = Cli::parse();

        match cli.commands {
            Commands::Import { file_type, table, location } => {
                match file_type {
                    FileTypes::Csv => {
                        match csv_import(table, location) {
                            Ok(()) => process::exit(0),
                            Err(e) => process::exit(1),
                        }

                     //       let file_handle = csv_import(table, location);
                    }
                }
            },
            Commands::Run => {
                println!("Run this app.");
                println!("Starting RESTful API server");
                start_rest_api().await;  
            }
        }

}

