// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        due_date -> Nullable<Date>,
        status -> Bool,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        task_count -> Nullable<Int8>,
        completed_task_count -> Nullable<Int8>,
        incomplete_task_count -> Nullable<Int8>,
        overdue_task_count -> Nullable<Int8>,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
