#[macro_use]
extern crate rocket;

mod authorization;
mod model;
mod repo;
mod schema;
mod controller;
mod database;
use rocket::serde::json::{json, Value};

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![    // controllers
                controller::get_students,
                controller::get_student,
                controller::create_student,
                controller::update_student,
                controller::delete_student,
            ],
        )
        .register("/", catchers![not_found]) 
        .attach(database::DatabasePool::fairing()) // Connect to the database before you launch the server
        .launch()
        .await;
}
