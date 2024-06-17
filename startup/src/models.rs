use diesel::{prelude::*, sql_types::{BigInt, Float}};
use uuid::Uuid;
use chrono::prelude::*;
use serde::Deserialize;
use bigdecimal::BigDecimal;

#[derive(Queryable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::allergies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(patient))]
#[diesel(belongs_to(encounter))]
pub struct Allergy {
    pub id: Uuid,
    pub start: Option<NaiveDate>,
    pub stop: Option<NaiveDate>,
    pub patient_id: Uuid,
    pub encounter_id: Uuid,
    pub code: Option<String>,
    pub system: Option<String>,
    pub description: Option<String>,
    pub type_ : Option<String>,
    pub category: Option<String>,
    pub snomed: Option<Vec<Option<String>>>    // Vec<String>,
}

#[derive(Insertable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::allergies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(patient))]
#[diesel(belongs_to(encounter))]
pub struct NewAllergy {
    pub start: Option<NaiveDate>,
    pub stop: Option<NaiveDate>,
    pub patient_id: Uuid,
    pub encounter_id: Uuid,
    pub code: Option<String>,
    pub system: Option<String>,
    pub description: Option<String>,
    pub type_ : Option<String>,
    pub category: Option<String>,
    pub snomed: Option<Vec<Option<String>>>    // Vec<String>,
}

#[derive(Queryable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Patient {
    pub id: Uuid,
    pub birthdate: NaiveDate,
    pub deathdate: Option<NaiveDate>,
    pub ssn: Option<String>,
    pub drivers: Option<String>,
    pub passport: Option<String>,
    pub prefix: Option<String>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub suffix: Option<String>,
    pub maiden: Option<String>,
    pub marital: Option<String>,
    pub race: Option<String>,
    pub ethnicity: Option<String>,
    pub gender: Option<String>,
    pub birthplace: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub healthcare_expenses: Option<String>,
    pub healthcare_coverage: Option<String>,
}

#[derive(Insertable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::patients)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPatient {
    pub birthdate: NaiveDate,
    pub deathdate: Option<NaiveDate>,
    pub ssn: Option<String>,
    pub drivers: Option<String>,
    pub passport: Option<String>,
    pub prefix: Option<String>,
    pub first: Option<String>,
    pub last: Option<String>,
    pub suffix: Option<String>,
    pub maiden: Option<String>,
    pub marital: Option<String>,
    pub race: Option<String>,
    pub ethnicity: Option<String>,
    pub gender: Option<String>,
    pub birthplace: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub county: Option<String>,
    pub zip: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub healthcare_expenses: Option<String>,
    pub healthcare_coverage: Option<String>,
}

#[derive(Queryable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state : Option<String>,
    pub zip: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub phone: Option<String>,
    pub revenue : Option<BigDecimal>,
    pub utilization: Option<i32>,
}

#[derive(Insertable, Selectable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewOrganization {
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state : Option<String>,
    pub zip: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub phone: Option<String>,
    pub revenue : Option<BigDecimal>,
    pub utilization: Option<i32>,
}  
