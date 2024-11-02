use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;

use crate::{
    data_access::establish_connection,
    model::{PersonPatchRequest, PersonRequest, PersonResponse},
    schema::person,
    AppState,
};

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
        (status = OK, description = "Success", body = PersonResponse, content_type = "application/json"),
        (status = NOT_FOUND, description = "Person with requested id does not exist")
    )
)]
pub async fn get_person(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> impl IntoResponse {
    let conn = &mut establish_connection(state.database_url.as_str());
    let res = person::table
        .find(person_id)
        .select(PersonResponse::as_select())
        .first(conn);

    match res {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(diesel::result::Error::NotFound) => (StatusCode::NOT_FOUND).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/persons",
    responses(
        (status = OK, description = "Success", body = Vec<PersonResponse>, content_type = "application/json")
    )
)]
pub async fn get_persons(State(state): State<AppState>) -> impl IntoResponse {
    let conn = &mut establish_connection(state.database_url.as_str());
    let res = person::table.select(PersonResponse::as_select()).load(conn);

    match res {
        Ok(values) => (StatusCode::OK, Json(values)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/persons",
    responses(
        (status = 201, description = "Success")
    ),
)]
pub async fn post_person(
    State(state): State<AppState>,
    uri: Uri,
    Json(person): Json<PersonRequest>,
) -> impl IntoResponse {
    let conn = &mut establish_connection(state.database_url.as_str());
    let res = diesel::insert_into(person::table)
        .values(&person)
        .returning(PersonResponse::as_returning())
        .get_result(conn);

    let mut headers = HeaderMap::new();
    match res {
        Ok(created_person) => {
            let path = format!("{}/{}", uri.path(), created_person.id);
            headers.insert(header::LOCATION, path.parse().unwrap());
            (StatusCode::CREATED, headers)
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, headers),
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
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(person): Json<PersonPatchRequest>,
) -> impl IntoResponse {
    let conn = &mut establish_connection(state.database_url.as_str());
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
pub async fn delete_person(Path(person_id): Path<i32>, State(state): State<AppState>) -> impl IntoResponse {
    let conn = &mut establish_connection(state.database_url.as_str());

    let res = diesel::delete(person::table)
        .filter(person::id.eq(person_id))
        .execute(conn);

    match res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}
