use crate::authorization;
use rocket::serde::json::{Json, json, Value};
use crate::model;
use crate::repo;
use rocket::response::status::{self, Custom};
use diesel::result::Error::NotFound;
use rocket::http::Status;
use crate::database::DatabasePool;


// For the BasicAuth type is implemented the FromRequest method.
// So this endpoint is executed first it runs the request guard
// of the BasicAuth type

/// GET Student
#[get("/students")]
pub async fn get_students(
    _auth: authorization::BasicAuth,
    db: DatabasePool,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        repo::StudentRepository::find_all(c, 1000)
            .map(|students| json!(students))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

/// Post Student.
#[post("/students", format = "json", data = "<new_student>")]
pub async fn create_student(
    _auth: authorization::BasicAuth,
    db: DatabasePool,
    new_student: Json<model::NewStudent>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        repo::StudentRepository::create(c, new_student.into_inner())
            .map(|students| json!(students))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

/// VIEW Student.
#[get("/students/<id>")]
pub async fn get_student(
    id: i32,
    _auth: authorization::BasicAuth,
    db: DatabasePool,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        repo::StudentRepository::find(c, id)
            .map(|student| json!(student))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

/// UPDATE Student.
#[post("/students/<id>", format = "json", data = "<student>")]
pub async fn update_student(
    id: i32,
    _auth: authorization::BasicAuth,
    db: DatabasePool,
    student: Json<model::Student>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        repo::StudentRepository::save(c, id, student.into_inner())
            .map(|student| json!(student))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

/// DELETE Student.
#[delete("/students/<id>", format = "json")]
pub async fn delete_student(
    id: i32,
    _auth: authorization::BasicAuth,
    db: DatabasePool,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        repo::StudentRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}
