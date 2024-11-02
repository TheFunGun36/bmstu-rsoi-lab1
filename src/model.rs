use std::collections::BTreeMap;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Insertable)]
#[diesel(table_name = crate::schema::person)]
pub struct PersonRequest {
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

#[derive(Serialize, Deserialize, ToSchema, AsChangeset)]
#[diesel(table_name = crate::schema::person)]
pub struct PersonPatchRequest {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
    pub work: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Queryable, Selectable)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersonResponse {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub address: String,
    pub work: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ErrorsResponse {
    pub message: String,
    pub errors: BTreeMap<String, String>
}
