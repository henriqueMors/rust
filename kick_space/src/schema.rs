// @generated automatically by Diesel CLI.

diesel::table! {
    best_players (id) {
        id -> Nullable<Integer>,
        name -> Text,
        score -> Integer,
    }
}

diesel::table! {
    bestplayers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        score -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    best_players,
    bestplayers,
);
