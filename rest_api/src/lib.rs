use axum::{
    routing::get,
    Router,
};

use std::net::SocketAddr;

//mod patients;
//mod organizations;
//mod payers;
//mod providers;
//

fn api_router() -> Router {
    Router::new()
        .route("/web/healthcheck", get(|| async { "Hi from the Rest API" }))
}

pub async fn start_rest_api () {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let router = api_router();

    println!("-------------------------------");
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
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn hello_world_for_rest_api() {
        let api_router = api_router();

        let response = api_router
            .oneshot(Request::builder().uri("/web/healthcheck").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
