table! {
    collectors (id) {
        id -> Uuid,
        name -> Varchar,
        private_base_uri -> Text,
        is_secure -> Bool,
    }
}

table! {
    election_collectors (election_id, collector_id) {
        election_id -> Uuid,
        collector_id -> Uuid,
    }
}

table! {
    elections (id) {
        id -> Uuid,
        is_public -> Bool,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        election_id -> Uuid,
    }
}

table! {
    registrations (user_id, election_id) {
        user_id -> Uuid,
        election_id -> Uuid,
    }
}

joinable!(election_collectors -> collectors (collector_id));
joinable!(election_collectors -> elections (election_id));
joinable!(questions -> elections (election_id));
joinable!(registrations -> elections (election_id));

allow_tables_to_appear_in_same_query!(
    collectors,
    election_collectors,
    elections,
    questions,
    registrations,
);
