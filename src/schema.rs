table! {
    controllers (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by_id -> Uuid,
        manufacturer -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        serial_number -> Nullable<Varchar>,
        function -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamp,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        location -> Nullable<Varchar>,
        email -> Varchar,
        hash -> Varchar,
        role -> Varchar,
    }
}

table! {
    valid_roles (role) {
        role -> Varchar,
    }
}

joinable!(controllers -> users (created_by_id));
joinable!(users -> valid_roles (role));

allow_tables_to_appear_in_same_query!(
    controllers,
    users,
    valid_roles,
);
