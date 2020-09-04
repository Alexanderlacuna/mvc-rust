table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
    }
}

table! {
    users2 (id) {
        id -> Integer,
        publicId -> Text,
        username -> Text,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    users2,
);
