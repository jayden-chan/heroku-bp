#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::env;
use std::collections::HashMap;
use rocket::config::{Config, Environment, Value};
use rocket_contrib::databases::postgres;

#[database("pg_db")]
struct DbConn(postgres::Connection);

struct Entry {
    pub title: String,
    pub number: i16,
    pub published: bool,
};

#[get("/ping")]
fn ping() -> &'static str {
    "Hello there from the /ping endpoint!"
}

#[get("/test")]
fn test(conn: DbConn) -> String {
    for row in &conn.query("SELECT title, number, published FROM data LIMIT 1", &[]).unwrap() {
        let res = Entry {
            title: row.get(0),
            number: row.get(1),
            published: row.get(2),
        };

        return format!("Title: {} Number: {}, Published {}", res.title, res.number, res.published);
    }

    String::from("No results")
}

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
        .attach(DbConn::fairing())
        .mount("/", routes![hello, ping, test])
        .launch();
}
