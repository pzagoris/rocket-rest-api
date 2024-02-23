use crate::schema;

/// The Student model. 
#[derive(serde::Deserialize, serde::Serialize, diesel::Queryable, diesel::AsChangeset)]
#[diesel(table_name = schema::students)]
pub struct Student {
    #[serde(skip_deserializing)] //marks it as string it is missing from json body
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

/// The New_Student model.
#[derive(serde::Deserialize, serde::Serialize, diesel::Insertable)]
#[diesel(table_name = schema::students)]
pub struct NewStudent {
    pub first_name: String,
    pub last_name: String,
}
