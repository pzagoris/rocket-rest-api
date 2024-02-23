// @generated automatically by Diesel CLI.

diesel::table! {
    students (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
    }
}
