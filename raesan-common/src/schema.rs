// @generated automatically by Diesel CLI.

diesel::table! {
    chapter (id) {
        id -> Text,
        name -> Text,
        subject_id -> Text,
    }
}

diesel::table! {
    class (id) {
        id -> Text,
        name -> Integer,
    }
}

diesel::table! {
    question (id) {
        id -> Text,
        body -> Text,
        chapter_id -> Text,
    }
}

diesel::table! {
    subject (id) {
        id -> Text,
        name -> Text,
        class_id -> Text,
    }
}

diesel::joinable!(chapter -> subject (subject_id));
diesel::joinable!(question -> chapter (chapter_id));
diesel::joinable!(subject -> class (class_id));

diesel::allow_tables_to_appear_in_same_query!(
    chapter,
    class,
    question,
    subject,
);
