//
// Functions and structures related to the math behind the e-voting protocol
//
mod generator;
mod verify_voting_vector;

pub use generator::generator_prime_pair;
pub use verify_voting_vector::{verify_voting_vector, VerifyVotingVectorInput};
