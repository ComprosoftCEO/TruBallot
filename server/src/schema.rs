table! {
    candidates (id) {
        id -> Uuid,
        question_id -> Uuid,
        candidate -> Varchar,
        num_votes -> Nullable<Int8>,
    }
}

table! {
    commitments (user_id, election_id, question_id) {
        user_id -> Uuid,
        election_id -> Uuid,
        question_id -> Uuid,
        forward_ballot -> Numeric,
        reverse_ballot -> Numeric,
        g_s -> Numeric,
        g_s_prime -> Numeric,
        g_s_s_prime -> Numeric,
        single_vote_verified -> Bool,
        published_ballots_verified -> Bool,
    }
}

table! {
    elections (id) {
        id -> Uuid,
        name -> Varchar,
        created_by -> Uuid,
        status -> Int4,
        prime -> Numeric,
        generator -> Numeric,
        encryption_key -> Bytea,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        election_id -> Uuid,
        question -> Varchar,
        final_forward_ballot -> Nullable<Numeric>,
        final_reverse_ballot -> Nullable<Numeric>,
        ballot_valid -> Bool,
    }
}

table! {
    registrations (user_id, election_id) {
        user_id -> Uuid,
        election_id -> Uuid,
        encrypted_location -> Bytea,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        hashed_password -> Varchar,
        name -> Varchar,
        refresh_secret -> Varchar,
    }
}

joinable!(candidates -> questions (question_id));
joinable!(commitments -> elections (election_id));
joinable!(commitments -> questions (question_id));
joinable!(commitments -> users (user_id));
joinable!(elections -> users (created_by));
joinable!(questions -> elections (election_id));
joinable!(registrations -> elections (election_id));
joinable!(registrations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    candidates,
    commitments,
    elections,
    questions,
    registrations,
    users,
);
