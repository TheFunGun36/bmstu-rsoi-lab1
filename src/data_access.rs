use diesel::{Connection, PgConnection};

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url).expect("Failed to establish connection to database")
}
