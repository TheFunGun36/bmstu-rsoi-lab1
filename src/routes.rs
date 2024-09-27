use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};

use crate::model::{Person, PersonWithId};

fn sample_person() -> PersonWithId {
    PersonWithId {
        id: 13,
        inner: Person {
            name: "Jora".to_owned(),
            age: 25,
            address: "st. Pushkina, h. Colotushkina".to_owned(),
            work: "PoelPospal Inc.".to_owned(),
        },
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/check-health",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn check_health() -> impl IntoResponse {
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/api/v1/persons/{personId}",
    responses(
        (status = OK, description = "Success", body = PersonWithId, content_type = "application/json"),
        (status = NOT_FOUND, description = "Person with requested id does not exist")
    )
)]
pub async fn get_person(Path(person_id): Path<i32>) -> impl IntoResponse {
    let result = sample_person();
    if person_id == result.id {
        Json::from(result).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/persons",
    responses(
        (status = OK, description = "Success", body = Vec<PersonWithId>, content_type = "application/json")
    )
)]
pub async fn get_persons() -> impl IntoResponse {
    let result = vec![sample_person()];
    Json::from(result)
}

#[utoipa::path(
    post,
    path = "/api/v1/persons",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn post_person(Json(_person): Json<Person>) -> impl IntoResponse {
    StatusCode::OK
}

#[utoipa::path(
    patch,
    path = "/api/v1/persons/{personId}",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn patch_person(Path(_person_id): Path<i32>, Json(_person): Json<Person>) -> impl IntoResponse {
    StatusCode::OK
}

#[utoipa::path(
    delete,
    path = "/api/v1/persons/{personId}",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn delete_person(Path(_person_id): Path<i32>) -> impl IntoResponse {
    StatusCode::OK
}