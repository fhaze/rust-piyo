// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        msg -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
    }
}

diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
