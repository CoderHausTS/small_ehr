extern crate dotenv;

use std::env;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use uuid::Uuid;

pub mod schema;
pub mod models;

use models::{ Patient, NewPatient };
use schema::patients::dsl::patients;

pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_patient(conn: &mut PgConnection, new_patient: &NewPatient) ->  Result<Patient, Error> {  // Patient {
    use crate::schema::patients;

    diesel::insert_into(patients::table)
        .values(new_patient)
        .returning(Patient::as_returning())
        .get_result(conn)
        //.expect("Error saving new post")
}

// We are sending back a result, so in our code we can address this with and Ok, Err match.
pub fn get_patient_by_uuid(conn: &mut PgConnection, patient_uuid: &Uuid) -> Result<Patient, Error> {
    patients
        .find(patient_uuid)
        .first(conn)
}

#[cfg(test)]


mod tests {
    use super::*;
    use uuid::{uuid, Uuid};

    #[test]
    fn successfully_creates_new_patient_record() {
       let record = r#"
       { 
            "birthdate": "2019-02-17", "ssn": "999-65-3251", "first": "Damon455", "last": "Langosh790"
       }"#;

        let db_connection = &mut establish_connection();

        let new_patient: NewPatient = serde_json::from_str(record).unwrap();
        
        let created_patient = create_patient(db_connection, &new_patient);
        
        assert!(created_patient.is_ok());
        // let new_pt_bday = new_patient.birthdate;
        // let created_pt_bday = created_patient.birthdate;

        // assert_eq!(new_pt_bday, created_pt_bday);
    }

    #[test]
    fn successfully_retrieves_patient_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let patient_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_patient = get_patient_by_uuid(db_connection, &patient_uuid);
        assert!(retrieved_patient.is_ok())

        // assert_eq!(patient_uuid, retrieved_patient.id);
    }

    #[test]
    fn fail_to_retrieve_patient_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let patient_uuid: Uuid = uuid!("2b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_patient = get_patient_by_uuid(db_connection, &patient_uuid);

        assert!(retrieved_patient.is_err());
    }
}
