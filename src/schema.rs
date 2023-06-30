// @generated automatically by Diesel CLI.

diesel::table! {
    permission (id) {
        id -> Uuid,
        name -> Text,
        guard_name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    role (id) {
        id -> Uuid,
        name -> Text,
        guard_name -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    permission,
    role,
    user,
);
