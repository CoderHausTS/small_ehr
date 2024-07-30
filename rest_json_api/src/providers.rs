use axum::{
        http::StatusCode, 
        routing::{ get, post }, 
        Json, 
        Router, 
        extract::Path,
        response::IntoResponse,
};

use database::models::{ NewProvider , Provider };
use uuid::Uuid;

async fn create_new_provider(Json(newprovider): Json<NewProvider>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

    match database::create_provider(db_connection, &newprovider) {
        Ok(provider_result) => (StatusCode::OK, Json(provider_result)),
        Err(_err) => (StatusCode::UNPROCESSABLE_ENTITY, Json(Provider::default())),
     }
}

// We get the UUID from the incoming path,
// and send back a response that is StatusCode, T
// In our case T is the provider type
// To do this, we have to send a result from our database code.
// We also had to add a default trait to our Provider model, to return nulls
// The response here isn't strictly correct, as we would send a 200 for an error
// THought right now, that error would most likely be the case where we didn't find any
// data. 
// So, we'll leave a 200 for an error, and update this later to make the error handling 
// mroe specific. 
// In the case where a record isn't found, we also send an ID of all 0's
async fn find_provider_by_uuid(Path(provider_uuid): Path<Uuid>) -> impl IntoResponse { 
    let db_connection = &mut database::establish_connection();

   match database::get_provider_by_uuid(db_connection, &provider_uuid) {
        // need to add error response, or empty response, here
        Ok(provider_result) => (StatusCode::OK, Json(provider_result)),
        Err(_err) => (StatusCode::OK, Json(Provider::default())),
   }
}

pub fn provider_router() -> Router {
    Router::new()
        .route("/api/provider", post(create_new_provider))
        .route("/api/provider/:id", get( find_provider_by_uuid ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    use uuid::{uuid, Uuid};
    use database::establish_connection;
    use diesel::prelude::*;

  
    #[tokio::test]
    async fn create_new_provider_json() {
       let db_connection = &mut establish_connection();

       // need to create an org ID first
       // bad juju. Need to insert a patient record, then get the UUID to look for.
       let org_uuid: Uuid = uuid!("4b73f74f-1101-44da-be73-aa091eb00879");
       let name = "Jims Place"; 
       let query = format!("INSERT INTO organizations (id, name) VALUES (\'{}\', \'{}\')", org_uuid, name);
       diesel::sql_query(query)
                       .execute(db_connection)
                               .unwrap();

       let record = r#"
       { 
            "organization_id": "4b73f74f-1101-44da-be73-aa091eb00879" , "name": "Provider, Test", 
            "gender": "Female", 
            "specialty": "Hem Onc", 
            "address": "1313 Mockingbird Lane", 
            "city": "San Bernadino", 
            "state": "Illinois"
       }"#;

        let api_router = provider_router();

        let _new_provider: NewProvider = serde_json::from_str(record).unwrap();

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/provider")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        record,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

//        let created_provider_id = response.as_ref().;
//
//        let query = format!("DELETE FROM providers WHERE id = \'{}\'", created_provider_id);
//        diesel::sql_query(query)
//            .execute(db_connection)
//            .unwrap();
//  
//        let query = format!("DELETE FROM organizations WHERE id = \'{}\'", org_uuid);
//        diesel::sql_query(query)
//            .execute(db_connection)
//            .unwrap();
 
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn fail_to_create_new_provider_json() {

        let api_router = provider_router();

        let record = r#"
            { 
                "birthdate": "", "ssn": "", "first": "", "last": ""
            }"#;

        let response = api_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/provider")
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
    async fn successfully_retrieve_provider_by_uuid() {
		// api_router is our router
        let api_router = provider_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/provider/3b75f74f-1101-44da-be73-aa091eb00873")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

     #[tokio::test]
    async fn fail_to_retrieve_provider_by_uuid() {
        let api_router = provider_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/provider/3b75f74f-1101-44da-be73-aa091eb00872")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
