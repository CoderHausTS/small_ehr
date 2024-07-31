extern crate dotenv;

use std::env;
use dotenv::dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use uuid::Uuid;

pub mod schema;
pub mod models;

use models::{ Patient, NewPatient, Organization, NewOrganization, Payer, NewPayer,
               Provider, NewProvider};
use schema::patients::dsl::patients;
use schema::organizations::dsl::organizations;
use schema::payers::dsl::payers;
use schema::providers::dsl::providers;

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

pub fn create_organization(conn: &mut PgConnection, new_organization: &NewOrganization) ->  Result<Organization, Error> {  // Patient {
    use crate::schema::organizations;

    diesel::insert_into(organizations::table)
        .values(new_organization)
        .returning(Organization::as_returning())
        .get_result(conn)
        //.expect("Error saving new post")
}

// We are sending back a result, so in our code we can address this with and Ok, Err match.
pub fn get_organization_by_uuid(conn: &mut PgConnection, organization_uuid: &Uuid) -> Result<Organization, Error> {
   organizations 
        .find(organization_uuid)
        .first(conn)
}

pub fn create_payer(conn: &mut PgConnection, new_payer: &NewPayer) ->  Result<Payer, Error> {  // Patient {
    use crate::schema::payers;

    diesel::insert_into(payers::table)
        .values(new_payer)
        .returning(Payer::as_returning())
        .get_result(conn)
        //.expect("Error saving new post")
}

// We are sending back a result, so in our code we can address this with and Ok, Err match.
pub fn get_payer_by_uuid(conn: &mut PgConnection, payer_uuid: &Uuid) -> Result<Payer, Error> {
    payers
        .find(payer_uuid)
        .first(conn)
}

pub fn create_provider(conn: &mut PgConnection, new_provider: &NewProvider) ->  Result<Provider, Error> {  // Patient {
    use crate::schema::providers;

    diesel::insert_into(providers::table)
        .values(new_provider)
        .returning(Provider::as_returning())
        .get_result(conn)
        //.expect("Error saving new post")
}

// We are sending back a result, so in our code we can address this with and Ok, Err match.
pub fn get_provider_by_uuid(conn: &mut PgConnection, provider_uuid: &Uuid) -> Result<Provider, Error> {
   providers 
        .find(provider_uuid)
        .first(conn)
}

#[cfg(test)]

mod tests {
    use super::*;
    use chrono::NaiveDate;
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
    }

    #[test]
    fn successfully_retrieves_patient_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let patient_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        let birthdate: NaiveDate = NaiveDate::parse_from_str("2012-12-12", "%Y-%m-%d").unwrap(); 
        let query = format!("INSERT INTO patients (id, birthdate) VALUES (\'{}\', \'{}\')", patient_uuid, birthdate);
        diesel::sql_query(query)
                        .execute(db_connection)
                                .unwrap();

        let retrieved_patient = get_patient_by_uuid(db_connection, &patient_uuid);

        let query = format!("DELETE FROM patients WHERE id = \'{}\'", patient_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();

        assert!(retrieved_patient.is_ok())

    }

    #[test]
    fn fail_to_retrieve_patient_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let patient_uuid: Uuid = uuid!("2b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_patient = get_patient_by_uuid(db_connection, &patient_uuid);

        assert!(retrieved_patient.is_err());
    }
    
    // -----------------------organization -----------
    #[test]
    fn successfully_creates_new_organization_record() {
       let record = r#"
       { 
            "name": "Monolith Health"
       }"#;

        let db_connection = &mut establish_connection();

        let new_organization: NewOrganization = serde_json::from_str(record).unwrap();
        
        let created_organization = create_organization(db_connection, &new_organization);
        
        assert!(created_organization.is_ok());
    }

    #[test]
    fn successfully_retrieves_organization_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let organization_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        let name = "Monolith Health"; 
        let query = format!("INSERT INTO organizations (id, name) VALUES (\'{}\', \'{}\')", organization_uuid, name);
        diesel::sql_query(query)
                        .execute(db_connection)
                                .unwrap();


        let retrieved_organization = get_organization_by_uuid(db_connection, &organization_uuid);

        // clean up
        let query = format!("DELETE FROM organizations WHERE id = \'{}\'", organization_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();

        assert!(retrieved_organization.is_ok())

        // assert_eq!(patient_uuid, retrieved_patient.id);
    }

    #[test]
    fn fail_to_retrieve_organization_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let organization_uuid: Uuid = uuid!("2b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_organization = get_organization_by_uuid(db_connection, &organization_uuid);

        assert!(retrieved_organization.is_err());
    }

    // -------------------------------payer --------------------
    #[test]
    fn successfully_creates_new_payer_record() {
       let record = r#"
       { 
            "name": "Jane Smith"
       }"#;

        let db_connection = &mut establish_connection();

        let new_payer: NewPayer = serde_json::from_str(record).unwrap();
        
        let created_payer = create_payer(db_connection, &new_payer);
        
        assert!(created_payer.is_ok());
    }

    #[test]
    fn successfully_retrieves_payer_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let payer_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        let name = "Jimmy Jones"; 
        let query = format!("INSERT INTO payers (id, name) VALUES (\'{}\', \'{}\')", payer_uuid, name);
        diesel::sql_query(query)
                        .execute(db_connection)
                                .unwrap();


        let retrieved_payer = get_payer_by_uuid(db_connection, &payer_uuid);

        // clean up
        let query = format!("DELETE FROM payers WHERE id = \'{}\'", payer_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();

        assert!(retrieved_payer.is_ok())

        // assert_eq!(patient_uuid, retrieved_patient.id);
    }

    #[test]
    fn fail_to_retrieve_payer_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let payer_uuid: Uuid = uuid!("2b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_payer = get_payer_by_uuid(db_connection, &payer_uuid);

        assert!(retrieved_payer.is_err());
    }


    // ---------------------Provider------------------------------
    #[test]
    fn successfully_creates_new_provider_record() {
       let db_connection = &mut establish_connection();

       // need to create an org ID first
       // bad juju. Need to insert a patient record, then get the UUID to look for.
       let org_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00879");
       let name = "Ruths Chris"; 
       let query = format!("INSERT INTO organizations (id, name) VALUES (\'{}\', \'{}\')", org_uuid, name);
       diesel::sql_query(query)
                       .execute(db_connection)
                               .unwrap();

       let record = r#"
       { 
            "organization_id": "3b75f74f-1101-44da-be73-aa091eb00879" , "name": "Provider, Test", 
            "gender": "Female", 
            "specialty": "Hem Onc", 
            "address": "1313 Mockingbird Lane", 
            "city": "San Bernadino", 
            "state": "Illinois"
       }"#;

//        let db_connection = &mut establish_connection();

        //let new_provider: NewProvider = serde_json::from_str(record).unwrap();
        
        let new_provider: NewProvider = serde_json::from_str(record).unwrap();
        
        let created_provider = create_provider(db_connection, &new_provider);
        
        let created_provider_id = created_provider.as_ref().unwrap().id;

        let query = format!("DELETE FROM providers WHERE id = \'{}\'", created_provider_id);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();
 
        let query = format!("DELETE FROM organizations WHERE id = \'{}\'", org_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();
       
        assert!(created_provider.is_ok());
    }

    #[test]
    fn successfully_retrieves_provider_by_uuid() {
       let db_connection = &mut establish_connection();

       // need to create an org ID first
       // bad juju. Need to insert a patient record, then get the UUID to look for.
       let org_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00878");
       let name = "Ruths Chris"; 
       let query = format!("INSERT INTO organizations (id, name) VALUES (\'{}\', \'{}\')", org_uuid, name);
       diesel::sql_query(query)
                       .execute(db_connection)
                               .unwrap();


        let provider_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        let name = "Provider, Test"; 
        let query = format!("INSERT INTO providers (id, name, organization_id) VALUES (\'{}\', \'{}\', \'{}\')", provider_uuid, name, org_uuid);
 
        diesel::sql_query(query)
                        .execute(db_connection)
                                .unwrap();

        let retrieved_provider = get_provider_by_uuid(db_connection, &provider_uuid);

        let query = format!("DELETE FROM providers WHERE id = \'{}\'", provider_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();

        let query = format!("DELETE FROM organizations  WHERE id = \'{}\'", org_uuid);
        diesel::sql_query(query)
            .execute(db_connection)
            .unwrap();


        assert!(retrieved_provider.is_ok())

    }

    #[test]
    fn fail_to_retrieve_provider_by_uuid() {
        let db_connection = &mut establish_connection();

        // bad juju. Need to insert a patient record, then get the UUID to look for.
        let provider_uuid: Uuid = uuid!("2b75f74f-1101-44da-be73-aa091eb00873");

        let retrieved_provider = get_provider_by_uuid(db_connection, &provider_uuid);

        assert!(retrieved_provider.is_err());
    }

}
