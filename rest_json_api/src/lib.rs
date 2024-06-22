use axum::{
        http::StatusCode, 
        routing::{ get, post }, 
        Json, 
        Router, 
        debug_handler,
        extract::Path,
        response::IntoResponse,
};

use database::models::{ NewPatient , Patient };
use uuid::{ uuid, Uuid };

use serde_json::json;

use std::net::SocketAddr;

// async fn create_new_patient(Json(newpatient): Json<NewPatient>) { 
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

   // Ok(Json(patient_result))
}

pub fn patient_router() -> Router {
    Router::new()
        .route("/api/patient", post(create_new_patient))
        .route("/api/patient/:id", get( find_patient_by_uuid ))
    
}

fn api_router() -> Router {
    // testing
    Router::new()
        .route("/api/healthcheck", get(|| async { "Hello, World!" }))
        .merge(patient_router())
}

pub async fn start_rest_api () {

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let router = api_router();
    axum::serve(listener, router).await.unwrap(); 
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::connect_info::MockConnectInfo,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    // use serde_json::{json, Value};
    // use tokio::net::TcpListener;
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`
    use database::models::Patient;

    // test the health check
    #[tokio::test]
    async fn hello_world() {
		// api_router is our router
        let api_router = api_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/healthcheck").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn create_new_patient_json() {

        // let db_connection = &mut database::establish_connection();
        let api_router = api_router();

        let record = r#"
            { 
                "birthdate": "2021-02-17", "ssn": "999-65-4444", "first": "Jason", "last": "APIPAT"
            }"#;


        let new_patient: NewPatient = serde_json::from_str(record).unwrap();
  
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

        // let db_connection = &mut database::establish_connection();
        let api_router = api_router();

        let record = r#"
            { 
                "birthdate": "", "ssn": "", "first": "", "last": ""
            }"#;


        // let new_patient: NewPatient = serde_json::from_str(record).unwrap();
  
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
        let api_router = api_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/patient/3b75f74f-1101-44da-be73-aa091eb00873")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // let body = response.into_body().collect().await.unwrap().to_bytes();
        // assert_eq!(&body["id"],"3b75f74f-1101-44da-be73-aa091eb00873" );
    }

     #[tokio::test]
    async fn fail_to_retrieve_patient_by_uuid() {
		// api_router is our router
        let api_router = api_router();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = api_router
            .oneshot(Request::builder().uri("/api/patient/3b75f74f-1101-44da-be73-aa091eb00872")
                     .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // let body = response.into_body().collect().await.unwrap().to_bytes();
        // assert_eq!(&body["id"],"3b75f74f-1101-44da-be73-aa091eb00873" );
    }

   // #[tokio::test]
   //  async fn successfully_retrieve_patient_by_uuid() {
   //     let api_router = api_router();

   //     let patient_uuid: Uuid = uuid!("3b75f74f-1101-44da-be73-aa091eb00873");
        

        



//    #[tokio::test]
//    async fn not_found() {
//        let api_router = api_router();
//
//        let response = api_router
//            .oneshot(
//                Request::builder()
//                    .uri("/does-not-exist")
//                    .body(Body::empty())
//                    .unwrap(),
//            )
//            .await
//            .unwrap();
//
//        assert_eq!(response.status(), StatusCode::NOT_FOUND);
//        let body = response.into_body().collect().await.unwrap().to_bytes();
//        assert!(body.is_empty());
//    }
//
//    // You can also spawn a server and talk to it like any other HTTP server:
//    #[tokio::test]
//    async fn the_real_deal() {
//        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
//        let addr = listener.local_addr().unwrap();
//
//        tokio::spawn(async move {
//            axum::serve(listener, api_router()).await.unwrap();
//        });
//
//        let client =
//            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
//                .build_http();
//
//        let response = client
//            .request(
//                Request::builder()
//                    .uri(format!("http://{addr}"))
//                    .header("Host", "localhost")
//                    .body(Body::empty())
//                    .unwrap(),
//            )
//            .await
//            .unwrap();
//
//        let body = response.into_body().collect().await.unwrap().to_bytes();
//        assert_eq!(&body[..], b"Hello, World!");
//    }
//
//    // You can use `ready()` and `call()` to avoid using `clone()`
//    // in multiple request
//    #[tokio::test]
//    async fn multiple_request() {
//        let mut api_router = api_router().into_service();
//
//        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
//        let response = ServiceExt::<Request<Body>>::ready(&mut api_router)
//            .await
//            .unwrap()
//            .call(request)
//            .await
//            .unwrap();
//        assert_eq!(response.status(), StatusCode::OK);
//
//        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
//        let response = ServiceExt::<Request<Body>>::ready(&mut api_router)
//            .await
//            .unwrap()
//            .call(request)
//            .await
//            .unwrap();
//        assert_eq!(response.status(), StatusCode::OK);
//    }

}
