// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Integer,
        hash -> Text,
        url -> Text,
    }
}
