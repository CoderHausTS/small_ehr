use diesel::prelude::*;
use uuid::Uuid;
use chrono::prelude::*;
use serde::Deserialize;

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

#[derive(Queryable, Selectable, Deserialize, Debug)]
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
