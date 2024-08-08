extern crate dotenv;

use cli_commands::{RestCli, RestApiCommands };

use rest_api::start_rest_api;

use dotenv::dotenv;
// use std::process;
use clap::Parser; 

#[tokio::main]
async fn main() {
        dotenv().ok();
        let cli = RestCli::parse();

        match cli.rest_commands {
            RestApiCommands::Run => {
                println!("Starting RESTful API server");
                start_rest_api().await;  
            }
        }

}

