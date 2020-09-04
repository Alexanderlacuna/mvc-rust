use crate::diesel::SqliteConnection;
use diesel::prelude::*;
use crate::dotenv::dotenv;
use std::env;

pub fn establish_connection()->SqliteConnection{
    dotenv().ok();
    let database_url=env::var("DATABASE_URL")
    .expect("Database url must be set");
    SqliteConnection::establish(&database_url)
    .expect("Error trying to connect to the database")

}


