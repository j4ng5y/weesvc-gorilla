#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;


struct App {
    databse: SqliteConnection,
}