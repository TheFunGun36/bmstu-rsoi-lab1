use std::string::String;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
// for `collect`
use testcontainers::{
    core::IntoContainerPort,
    runners::AsyncRunner,
    ContainerAsync, ImageExt,
};
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

use crate::app;

pub const POSTGRES_USER: &str = "rsoi_lab1";
pub const POSTGRES_PASSWORD: &str = "rsoi_lab1";
pub const POSTGRES_DB: &str = "rsoi_lab1";

pub async fn database_container() -> (ContainerAsync<Postgres>, String) {
    let container = Postgres::default()
        .with_env_var("POSTGRES_USER", POSTGRES_USER)
        .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
        .with_env_var("POSTGRES_DB", POSTGRES_DB)
        .start()
        .await
        .unwrap();
    // let container = GenericImage::new("postgres", "13")
    //     .with_exposed_port(5432.tcp())
    //     .with_wait_for(WaitFor::message_on_stdout(
    //         "database system is ready to accept connections",
    //     ))
    //     .with_env_var("POSTGRES_USER", POSTGRES_USER)
    //     .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
    //     .with_env_var("POSTGRES_DB", POSTGRES_DB)
    //     .start()
    //     .await
    //     .expect("Failed to start the database");

    let host = container.get_host().await.unwrap();
    let port = container.get_host_port_ipv4(5432.tcp()).await.unwrap();

    let endpoint =
        format!("postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{host}:{port}/{POSTGRES_DB}");

    (container, endpoint)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_empty_select() {
    let (_container, db_endpoint) = database_container().await;

    let response = app(db_endpoint)
        .await
        .oneshot(
            Request::builder()
                .uri("/api/v1/persons")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!([]));
}
