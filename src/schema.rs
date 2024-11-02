// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        name -> Text,
        age -> Int4,
        address -> Text,
        work -> Text,
    }
}
