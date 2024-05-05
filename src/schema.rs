// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        due -> Nullable<Date>,
        status -> Bool,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_super -> Bool,
        task_count_current -> Nullable<Int8>,
        task_count_complete -> Nullable<Int8>,
        task_count_incomplete -> Nullable<Int8>,
        task_count_overdue -> Nullable<Int8>,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
