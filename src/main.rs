#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate core;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<username>/<age>")]
fn user_details(username: &RawStr, age: u8) -> String {
    format!("Hello, world! {}, {}", username, age)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/<username>/<age>", routes![user_details]);
}
