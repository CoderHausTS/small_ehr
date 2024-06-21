extern crate dotenv;

use std::env;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub mod schema;
pub mod models;

use models::{ Patient, NewPatient };
// use schema::patients;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_patient(conn: &mut PgConnection, new_patient: &NewPatient) ->  Patient {
    use crate::schema::patients;

    diesel::insert_into(patients::table)
        .values(new_patient)
        .returning(Patient::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successfully_creates_new_patient_record() {
       let record = r#"
       { 
            "birthdate": "2019-02-17", "ssn": "999-65-3251", "first": "Damon455", "last": "Langosh790"
       }"#;

        let db_connection = &mut establish_connection();

        let new_patient: NewPatient = serde_json::from_str(record).unwrap();
        
        let created_patient = create_patient(db_connection, &new_patient);

        print!("{:?}", created_patient);
        
        let new_pt_bday = new_patient.birthdate;
        let created_pt_bday = created_patient.birthdate;

        assert_eq!(new_pt_bday, created_pt_bday);
    }

 
}
