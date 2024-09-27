use std::ops::Deref;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub address: String,
    pub work: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[aliases(PersonWithId = WithId<Person>)]
pub struct WithId<T> {
    pub id: i32,

    #[serde(flatten)]
    pub inner: T,
}

impl<T> Deref for WithId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
