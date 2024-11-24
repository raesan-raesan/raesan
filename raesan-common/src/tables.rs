use crate::schema;
use diesel;
use serde;

// ----- `Class` table struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
)]
#[diesel(table_name=schema::classes)]
pub struct Class {
    pub id: String,
    pub name: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `Subject` table struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Class))]
#[diesel(table_name=schema::subjects)]
pub struct Subject {
    pub id: String,
    pub name: String,
    pub class_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `Chapter` table struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Subject))]
#[diesel(table_name=schema::chapters)]
pub struct Chapter {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ----- `Question` table struct
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Identifiable,
    diesel::Associations,
)]
#[diesel(belongs_to(Chapter))]
#[diesel(table_name=schema::questions)]
pub struct Question {
    pub id: String,
    pub body: String,
    pub chapter_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}
