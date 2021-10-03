table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hashed_password -> Varchar,
        name -> Varchar,
    }
}
