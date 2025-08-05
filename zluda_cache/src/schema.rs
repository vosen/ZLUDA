// @generated automatically by Diesel CLI.

diesel::table! {
    globals (key) {
        key -> Text,
        value -> BigInt,
    }
}

diesel::table! {
    modules (id) {
        id -> BigInt,
        hash -> Text,
        compiler_version -> Text,
        zluda_version -> Text,
        device -> Text,
        backend_key -> Text,
        binary -> Binary,
        last_access -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    globals,
    modules,
);
