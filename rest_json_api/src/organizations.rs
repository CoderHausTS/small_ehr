use axum::{
        http::StatusCode, 
        routing::{ get, post }, 
        Json, 
        Router, 
        extract::Path,
        response::IntoResponse,
};

use database::models::{ NewOrganization , Organization };
use uuid::Uuid;

async fn create_new_organization(Json(neworganization): Json<NewOrganization>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

    match database::create_organization(db_connection, &neworganization) {
        Ok(organization_result) => (StatusCode::OK, Json(organization_result)),
        Err(_err) => (StatusCode::UNPROCESSABLE_ENTITY, Json(Organization::default())),
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
async fn find_organization_by_uuid(Path(organization_uuid): Path<Uuid>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

   match database::get_organization_by_uuid(db_connection, &organization_uuid) {
        // need to add error response, or empty response, here
        Ok(organization_result) => (StatusCode::OK, Json(organization_result)),
        Err(_err) => (StatusCode::OK, Json(Organization::default())),
   }
}

pub fn organization_router() -> Router {
    Router::new()
        .route("/api/organization", post(create_new_organization))
        .route("/api/organization/:id", get( find_organization_by_uuid ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    use uuid::uuid;
    use database::establish_connection;
    use diesel::prelude::*;

    #[tokio::test]
    async fn create_new_organization_json_via_api() {

        let api_router = organization_router();

        let record = r#"
            { 
               "name": "Faraday Electrotherapy"
            }"#;


        let _new_organization: NewOrganization = serde_json::from_str(record).unwrap();

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/organization")
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
    async fn fail_to_create_new_organization_json_via_api() {

        let api_router = organization_router();

        let record = r#"
            { 
               "same": "Roentgen Radiology" 
            }"#;

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/organization")
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
    async fn successfully_retrieve_organization_by_uuid_via_api() {
		// api_router is our router
        let api_router = organization_router();
        let db_connection = &mut establish_connection();


        let organization_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        let name = "Monolith Health"; 
        let query = format!("INSERT INTO organizations (id, name) VALUES (\'{}\', \'{}\')", organization_uuid, name);
        diesel::sql_query(query)
                        .execute(db_connection)
                                .unwrap();


        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/organization/3b75f74f-1101-44da-be73-aa091eb00873")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

     #[tokio::test]
    async fn fail_to_retrieve_organization_by_uuid_via_api() {
        let api_router = organization_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/organization/3b75f74f-1101-44da-be73-aa091eb00872")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
