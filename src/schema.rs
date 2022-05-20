table! {
    license_change (id) {
        id -> Uuid,
        status -> Text,
        substance -> Text,
        date -> Timestamp,
        comment -> Nullable<Text>,
        link_to_documentation -> Nullable<Text>,
        created_by_id -> Uuid,
        created_at -> Timestamp,
        updated_by_id -> Uuid,
    }
}

table! {
    thermostat_status (id) {
        id -> Int4,
        status -> Bool,
        timestamp -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    license_change,
    thermostat_status,
    user,
);
