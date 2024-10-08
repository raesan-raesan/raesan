// imports
use diesel;
use raesan_common;
use serde;

// ----- `Class` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::class)]
pub struct Class {
    pub id: String,
    pub name: i32,
}

// ----- `Subject` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::subject)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub class_id: String,
    pub class_name: i32,
}

// ----- `Chapter` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::chapter)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub subject_id: String,
}

// ----- `Question` model struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
)]
#[diesel(table_name=raesan_common::schema::question)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_id: String,
}
