#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

use std::env;
use rocket::config::{Config, Environment};

#[get("/")]
fn hello() -> &'static str {
    "Hello world!"
}

#[get("/ping")]
fn ping() -> &'static str {
    "Hello there from the /ping endpoint!"
}

fn main() {
    let port = env::var("PORT")
        .unwrap_or(String::from("5000"))
        .parse::<u16>()
        .unwrap_or(5000);

    let config = Config::build(Environment::Staging)
        .port(port)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![hello, ping])
        .launch();
}
