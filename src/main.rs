use model::{PersonPatchRequest, PersonRequest, PersonResponse};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

mod data_access;
mod model;
mod routes;
mod schema;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        routes::check_health,
        routes::get_person,
        routes::patch_person,
        routes::delete_person,
        routes::get_persons,
        routes::post_person,
    ),
    components(schemas(PersonRequest, PersonPatchRequest, PersonResponse))
)]
struct ApiDoc;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable was not specified");

    let swagger = SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi());
    let app = OpenApiRouter::<()>::with_openapi(ApiDoc::openapi())
        .routes(routes!(routes::check_health))
        .routes(routes!(
            routes::get_person,
            routes::patch_person,
            routes::delete_person
        ))
        .routes(routes!(routes::get_persons, routes::post_person));

    let app = axum::Router::from(app).merge(swagger);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
