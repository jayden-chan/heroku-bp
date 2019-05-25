#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::env;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};

mod routes;

fn main() {
    let port = env::var("PORT")
        .unwrap_or(String::from("5000"))
        .parse::<u16>()
        .unwrap_or(5000);

    let mut pg_config = HashMap::with_capacity(1);
    let mut db = HashMap::with_capacity(1);
    let db_url = env::var("DATABASE_URL").unwrap();

    pg_config.insert("url", Value::from(db_url));
    db.insert("pg_db", Value::from(pg_config));

    let config = Config::build(Environment::Staging)
        .port(port)
        .extra("databases", db)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(routes::DbConn::fairing())
        .mount("/", routes![
               routes::ping,
               routes::test,
               routes::post
        ])
        .launch();
}
