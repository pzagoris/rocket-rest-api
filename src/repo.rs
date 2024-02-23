use diesel::prelude::*;

use crate::{
    model::{self, NewStudent, Student},
    schema::students,
};
use diesel::{QueryResult, SqliteConnection};

pub struct StudentRepository;

impl StudentRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Student> {
        students::table.find(id).get_result::<model::Student>(c)
    }

    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Student>> {
        students::table
            .order(students::id.desc())
            .limit(limit)
            .load::<model::Student>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_student: NewStudent) -> QueryResult<Student> {
        diesel::insert_into(students::table)
            .values(new_student)
            .execute(c)?;

        let last_id = Self::get_last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn save(c: &mut SqliteConnection, id: i32, student: Student) -> QueryResult<Student> {
        diesel::update(students::table.find(id))
            .set((
                students::first_name.eq(student.first_name.clone()), //Update only the fields
                students::last_name.eq(student.last_name.clone()),
            ))
            .execute(c)?;
        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(students::table.find(id)).execute(c)
    }

    fn get_last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        // this is problematic when 2 requests run simultaneously
        students::table
            .select(students::id)
            .order_by(students::id.desc())
            .first(c)
    }
}
