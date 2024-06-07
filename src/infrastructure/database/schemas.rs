use diesel::table;

table! {
    users (id) {
        id -> BigInt,
        external_id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    emails (id) {
        id -> BigInt,
        user_id -> BigInt,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    phones (id) {
        id -> BigInt,
        user_id -> BigInt,
        phone -> Varchar,
        created_at -> Timestamp,
    }
}