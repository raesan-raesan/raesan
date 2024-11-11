// @generated automatically by Diesel CLI.

diesel::table! {
    chapters (id) {
        id -> Text,
        name -> Text,
        display_name -> Text,
        subject_id -> Text,
        subject_name -> Text,
        class_name -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    classes (id) {
        id -> Text,
        name -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    questions (id) {
        id -> Text,
        body -> Text,
        chapter_name -> Text,
        subject_name -> Text,
        class_name -> Integer,
        chapter_id -> Text,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::table! {
    subjects (id) {
        id -> Text,
        name -> Text,
        display_name -> Text,
        class_id -> Text,
        class_name -> Integer,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

diesel::joinable!(chapters -> subjects (subject_id));
diesel::joinable!(questions -> chapters (chapter_id));
diesel::joinable!(subjects -> classes (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    chapters,
    classes,
    questions,
    subjects,
);
