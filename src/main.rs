use std::env;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use model::{PersonPatchRequest, PersonRequest, PersonResponse};
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

mod data_access;
mod logger;
mod model;
mod routes;
mod schema;

#[cfg(test)]
mod integration_tests;

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

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Debug, Clone)]
struct AppState {
    database_url: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable was not specified");

    let _logger_handler = logger::init();
    log::debug!("Logger initialized. Hello, world!");

    let app = app(database_url).await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn app(database_url: String) -> axum::Router {
    init_db(database_url.as_str());

    let swagger = SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi());
    let state = AppState { database_url };
    let app = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(routes::check_health))
        .routes(routes!(
            routes::get_person,
            routes::patch_person,
            routes::delete_person
        ))
        .routes(routes!(routes::get_persons, routes::post_person))
        .with_state(state);

    axum::Router::from(app).merge(swagger)
}

fn init_db(database_url: &str) {
    let mut conn = data_access::establish_connection(database_url);
    let result = conn.run_pending_migrations(MIGRATIONS);
    if let Err(e) = result {
        panic!("Failed to initialize DB: {e}");
    }
}
