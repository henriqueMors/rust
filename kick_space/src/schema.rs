// @generated automatically by Diesel CLI.

diesel::table! {
    bestplayers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        quantity -> Integer,
    }
}
