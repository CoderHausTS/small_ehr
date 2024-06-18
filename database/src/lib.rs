extern crate dotenv;

use std::env;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod models;


pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

