use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;

use crate::{
    data_access::establish_connection,
    model::{PersonPatchRequest, PersonRequest, PersonResponse},
    schema::person,
};

fn sample_person() -> PersonResponse {
    PersonResponse {
        id: 13,
        name: "Jora".to_owned(),
        age: 25,
        address: "st. Pushkina, h. Colotushkina".to_owned(),
        work: "PoelPospal Inc.".to_owned(),
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
        (status = 201, description = "Success")
    )
)]
pub async fn post_person(Json(person): Json<PersonRequest>) -> impl IntoResponse {
    let conn = &mut establish_connection();
    let res = diesel::insert_into(person::table)
        .values(&person)
        .execute(conn);

    match res {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[utoipa::path(
    patch,
    path = "/api/v1/persons/{personId}",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn patch_person(
    Path(person_id): Path<i32>,
    Json(person): Json<PersonPatchRequest>,
) -> impl IntoResponse {
    let conn = &mut establish_connection();
    let res = diesel::update(person::table)
        .filter(person::id.eq(person_id))
        .set(person)
        .execute(conn);

    match res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/persons/{personId}",
    responses(
        (status = OK, description = "Success")
    )
)]
pub async fn delete_person(Path(person_id): Path<i32>) -> impl IntoResponse {
    let conn = &mut establish_connection();

    let res = diesel::delete(person::table)
        .filter(person::id.eq(person_id))
        .execute(conn);

    match res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

