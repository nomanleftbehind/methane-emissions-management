table! {
    active_sessions (session_user_id, token_hash) {
        session_user_id -> Uuid,
        token_hash -> Bytea,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
    }
}

table! {
    controllers (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by_id -> Uuid,
        updated_by_id -> Uuid,
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

joinable!(active_sessions -> users (session_user_id));
joinable!(users -> valid_roles (role));

allow_tables_to_appear_in_same_query!(
    active_sessions,
    controllers,
    users,
    valid_roles,
);
