use axum::{
        http::StatusCode, 
        routing::{ get, post }, 
        Json, 
        Router, 
        extract::Path,
        response::IntoResponse,
};

use database::models::{ NewPatient , Patient };
use uuid::Uuid;

async fn create_new_patient(Json(newpatient): Json<NewPatient>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

    match database::create_patient(db_connection, &newpatient) {
        Ok(patient_result) => (StatusCode::OK, Json(patient_result)),
        Err(_err) => (StatusCode::UNPROCESSABLE_ENTITY, Json(Patient::default())),
     }
}

// We get the UUID from the incoming path,
// and send back a response that is StatusCode, T
// In our case T is the patient type
// To do this, we have to send a result from our database code.
// We also had to add a default trait to our Patient model, to return nulls
// The response here isn't strictly correct, as we would send a 200 for an error
// THought right now, that error would most likely be the case where we didn't find any
// data. 
// So, we'll leave a 200 for an error, and update this later to make the error handling 
// mroe specific. 
// In the case where a record isn't found, we also send an ID of all 0's
async fn find_patient_by_uuid(Path(patient_uuid): Path<Uuid>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

   match database::get_patient_by_uuid(db_connection, &patient_uuid) {
        // need to add error response, or empty response, here
        Ok(patient_result) => (StatusCode::OK, Json(patient_result)),
        Err(_err) => (StatusCode::OK, Json(Patient::default())),
   }
}

pub fn patient_router() -> Router {
    Router::new()
        .route("/api/patient", post(create_new_patient))
        .route("/api/patient/:id", get( find_patient_by_uuid ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    
    #[tokio::test]
    async fn create_new_patient_json() {

        let api_router = patient_router();

        let record = r#"
            { 
                "birthdate": "2021-02-17", "ssn": "999-65-4444", "first": "Jason", "last": "APIPAT"
            }"#;


        let _new_patient: NewPatient = serde_json::from_str(record).unwrap();

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/patient")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        record,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn fail_to_create_new_patient_json() {

        let api_router = patient_router();

        let record = r#"
            { 
                "birthdate": "", "ssn": "", "first": "", "last": ""
            }"#;

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/patient")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        record,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }


    #[tokio::test]
    async fn successfully_retrieve_patient_by_uuid() {
		// api_router is our router
        let api_router = patient_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/patient/3b75f74f-1101-44da-be73-aa091eb00873")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

     #[tokio::test]
    async fn fail_to_retrieve_patient_by_uuid() {
        let api_router = patient_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/patient/3b75f74f-1101-44da-be73-aa091eb00872")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
