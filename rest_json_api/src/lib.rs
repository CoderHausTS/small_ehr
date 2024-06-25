use axum::{
        routing::get, 
        Router, 
};

use std::net::SocketAddr;

mod patients;

fn api_router() -> Router {
    // testing
    Router::new()
        .route("/api/healthcheck", get(|| async { "Hello, World!" }))
        .merge(patients::patient_router())
}

pub async fn start_rest_api () {

    // run it with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let router = api_router();
    println!("-----------------------------------------------");
    println!("Listening on {:?}", listener.local_addr()); 
    axum::serve(listener, router).await.unwrap(); 
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

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

}
