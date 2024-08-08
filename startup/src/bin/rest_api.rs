extern crate dotenv;

use rest_api::start_rest_api;

use dotenv::dotenv;
use std::process;
use clap::Parser; 

#[tokio::main]
async fn main() {
        dotenv().ok();
        let cli = Cli::parse();

        match cli.commands {
            Commands::Run => {
                println!("Starting RESTful API server");
                start_rest_api().await;  
            }
        }

}

