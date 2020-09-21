#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

struct Db {
    connection: SqliteConnection,
}

impl Db {
    pub fn new() -> Db {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let conn = SqliteConntion::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        Db{
            connection: conn
        }
    }
}