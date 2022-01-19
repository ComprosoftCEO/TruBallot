//
// Functions and structures related to the math behind the e-voting protocol
//
mod count_ballot_votes;
mod generator;
pub mod location_anonymization;
mod shares_matrix;
pub mod stpm;

pub use count_ballot_votes::count_ballot_votes;
pub use generator::generator_prime_pair;
pub use shares_matrix::SharesMatrix;
