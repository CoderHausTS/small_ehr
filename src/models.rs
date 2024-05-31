use diesel::prelude::*;
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::allergies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Allergy {
    pub id: Uuid,
    pub start: Option<NaiveDate>,
    pub stop: Option<NaiveDate>,
    pub patient: Uuid,
    pub encounter: Uuid,
    pub code: Option<String>,
    pub system: Option<String>,
    pub description: Option<String>,
    pub type_ : Option<String>,
    pub category: Option<String>,
    pub snomed: Option<Vec<Option<String>>>    // Vec<String>,
}
