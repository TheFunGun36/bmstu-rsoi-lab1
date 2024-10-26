use diesel::{Connection, PgConnection};

pub fn establish_connection() -> PgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable was not specified");
    PgConnection::establish(&database_url).expect("Failed to establish connection to database")
}
