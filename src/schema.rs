// @generated automatically by Diesel CLI.

diesel::table! {
    orgstat (id) {
        id -> Int4,
        symbol -> Text,
        count -> Int4,
        processdate -> Timestamptz,
    }
}
