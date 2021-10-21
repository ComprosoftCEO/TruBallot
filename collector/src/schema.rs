table! {
    elections (id) {
        id -> Uuid,
        generator -> Numeric,
        prime -> Numeric,
        paillier_p -> Numeric,
        paillier_q -> Numeric,
    }
}

table! {
    encrypted_locations (user_id, election_id) {
        user_id -> Uuid,
        election_id -> Uuid,
        location -> Numeric,
    }
}

table! {
    questions (id) {
        id -> Uuid,
        election_id -> Uuid,
        num_candidates -> Int8,
    }
}

table! {
    registrations (user_id, election_id, question_id) {
        user_id -> Uuid,
        election_id -> Uuid,
        question_id -> Uuid,
        forward_verification_shares -> Numeric,
        reverse_verification_shares -> Numeric,
        forward_ballot_shares -> Numeric,
        reverse_ballot_shares -> Numeric,
    }
}

joinable!(encrypted_locations -> elections (election_id));
joinable!(questions -> elections (election_id));
joinable!(registrations -> elections (election_id));
joinable!(registrations -> questions (question_id));

allow_tables_to_appear_in_same_query!(
    elections,
    encrypted_locations,
    questions,
    registrations,
);
