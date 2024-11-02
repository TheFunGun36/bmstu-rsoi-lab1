use std::string::String;

use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
// for `collect`
use testcontainers::{core::IntoContainerPort, runners::AsyncRunner, ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;
use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

use crate::{
    app,
    model::{PersonPatchRequest, PersonRequest, PersonResponse},
};

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

#[tokio::test(flavor = "multi_thread")]
async fn post_select() {
    let (_container, db_endpoint) = database_container().await;
    let mut app = app(db_endpoint).await.into_service();

    let request_body = PersonRequest {
        address: "Kolotushkina".to_owned(),
        age: 16,
        name: "Kostya".to_owned(),
        work: "IT".to_owned(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/persons")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/persons")
        .body(Body::empty())
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let response_body = response.into_body().collect().await.unwrap().to_bytes();
    let response_body: Value = serde_json::from_slice(&response_body).unwrap();
    let response_body: Vec<PersonResponse> = serde_json::from_value(response_body).unwrap();

    assert_eq!(response_body.len(), 1);
    assert_eq!(response_body[0].age, request_body.age);
    assert_eq!(response_body[0].work, request_body.work);
    assert_eq!(response_body[0].address, request_body.address);
    assert_eq!(response_body[0].name, request_body.name);
}

#[tokio::test]
async fn post_select_single() {
    let (_container, db_endpoint) = database_container().await;
    let mut app = app(db_endpoint).await.into_service();

    let request_body = PersonRequest {
        address: "Kolotushkina".to_owned(),
        age: 16,
        name: "Kostya".to_owned(),
        work: "IT".to_owned(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/persons")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let path = response
        .headers()
        .get(header::LOCATION)
        .expect("No Location header returned on POST request");

    assert_eq!(response.status(), StatusCode::CREATED);

    let request = Request::builder()
        .method("GET")
        .uri(path.to_str().unwrap())
        .body(Body::empty())
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let response_body = response.into_body().collect().await.unwrap().to_bytes();
    let response_body: Value = serde_json::from_slice(&response_body).unwrap();
    let response_body: PersonResponse = serde_json::from_value(response_body).unwrap();

    assert_eq!(response_body.age, request_body.age);
    assert_eq!(response_body.work, request_body.work);
    assert_eq!(response_body.address, request_body.address);
    assert_eq!(response_body.name, request_body.name);
}

#[tokio::test(flavor = "multi_thread")]
async fn post_patch_select() {
    let (_container, db_endpoint) = database_container().await;
    let mut app = app(db_endpoint).await.into_service();

    let request_body = PersonRequest {
        address: "Kolotushkina".to_owned(),
        age: 16,
        name: "Kostya".to_owned(),
        work: "IT".to_owned(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/persons")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let path = response
        .headers()
        .get(header::LOCATION)
        .expect("No Location header returned on POST request");

    let patch = PersonPatchRequest {
        address: None,
        age: Some(17),
        name: None,
        work: None,
    };
    let request = Request::builder()
        .method("PATCH")
        .uri(path.to_str().unwrap())
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&patch).unwrap()))
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/persons")
        .body(Body::empty())
        .unwrap();

    let response = ServiceExt::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    let response_body = response.into_body().collect().await.unwrap().to_bytes();
    let response_body: Value = serde_json::from_slice(&response_body).unwrap();
    let response_body: Vec<PersonResponse> = serde_json::from_value(response_body).unwrap();

    assert_eq!(response_body.len(), 1);
    assert_eq!(response_body[0].age, patch.age.unwrap());
    assert_eq!(response_body[0].work, request_body.work);
    assert_eq!(response_body[0].address, request_body.address);
    assert_eq!(response_body[0].name, request_body.name);
}
