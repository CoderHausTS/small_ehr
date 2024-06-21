// This is cool. We can have each router in its own file in this crate.
// Then we just use each mod, and it handles the requests for a certain type of endpoint.
//
// fn api_router() -> Router {
//     // This is the order that the modules were authored in.
//     users::router()
//         .merge(profiles::router())
//         .merge(articles::router())
// }

use axum::{
        Router,
        routing::{ get, post },
        Json,
};

use database::models::NewPatient; // , Patient };

async fn create_new_patient(Json(newpatient): Json<NewPatient>) { // -> Json<Patient> {
    let db_connection = &mut database::establish_connection();
    database::create_patient(db_connection, &newpatient); 

    // if the create was good, return 200,
    // if not, return error
}

pub fn patient_router() -> Router {
    Router::new()
        .route("/api/patient", post(create_new_patient))
    
}

fn api_router() -> Router {
    // testing
    Router::new()
        .route("/api/healthcheck", get(|| async { "Hello, World!" }))
        .merge(patient_router())
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
