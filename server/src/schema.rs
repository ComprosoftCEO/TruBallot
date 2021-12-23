table! {
    candidates (id) {
        id -> Uuid,
        question_id -> Uuid,
        candidate -> Varchar,
        candidate_number -> Int8,
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
        is_public -> Bool,
        access_code -> Nullable<Varchar>,
        generator -> Numeric,
        prime -> Numeric,
        location_modulus -> Numeric,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        election_id -> Uuid,
        question -> Varchar,
        question_number -> Int8,
        forward_cancelation_shares -> Numeric,
        reverse_cancelation_shares -> Numeric,
    }
}

table! {
    registrations (user_id, election_id) {
        user_id -> Uuid,
        election_id -> Uuid,
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
