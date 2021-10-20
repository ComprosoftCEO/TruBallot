use curv_kzen::arithmetic::traits::Converter;
use curv_kzen::BigInt;
use std::iter;

pub struct VerifyVotingVectorInput<'a, 'b> {
  pub forward_ballot: &'a BigInt,
  pub reverse_ballot: &'b BigInt,
  pub num_candidates: i64,
  pub num_voters: i64,
  pub no_vote_count: usize,
}

///
/// Test the validity of the voting vector
///
pub fn verify_voting_vector(input: VerifyVotingVectorInput) -> bool {
  let total_bits = (input.num_candidates * input.num_voters) as usize;
  let forward_bits = get_bits(&input.forward_ballot, total_bits);
  let reverse_bits = get_bits(&input.reverse_ballot, total_bits);

  // Step 1: Make sure the forward and reverse bits match
  for (f, r) in forward_bits.iter().zip(reverse_bits.iter().rev()) {
    if *f != *r {
      return false;
    }
  }

  // Step 2: Make sure each chunk has, at most, only one candidate selected
  let chunks = get_chunks(&forward_bits, input.num_candidates as usize);
  let (voted, no_vote) = match verify_chunks(&chunks) {
    Some((voted, no_vote)) => (voted, no_vote),
    None => return false,
  };

  // Step 3: Make sure the number of voted bits matches the users who voted
  let expected_voted = total_bits - input.no_vote_count;
  let expected_no_vote = input.no_vote_count;
  if voted != expected_voted || no_vote != expected_no_vote {
    return false;
  }

  // All checks passed!
  true
}

/// Convert the integer voting vector into a vector of bits
fn get_bits(input: &BigInt, total_bits: usize) -> Vec<bool> {
  let reverse_bits: Vec<bool> = input
    .to_str_radix(2)
    .chars()
    .rev()
    .filter(|c| c.is_numeric())
    .map(|x| x == '1')
    .chain(iter::repeat(false))
    .take(total_bits)
    .collect();

  reverse_bits.into_iter().rev().collect()
}

/// Convert the vector of bits into chunks for each candidate
fn get_chunks(input: &[bool], num_candidates: usize) -> Vec<Vec<bool>> {
  input
    .chunks(num_candidates)
    .map(|c| c.into_iter().cloned().collect())
    .collect()
}

/// Make sure each chunk only has, at most, one candidate
///    Returns Some((voted, didn't vote)) on success, or None on failure
fn verify_chunks(input: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
  let mut voted = 0;
  let mut no_vote = 0;

  for chunk in input {
    let num_votes = chunk.into_iter().filter(|c| **c).count();
    match num_votes {
      0 => no_vote += 1,
      1 => voted += 1,
      _ => return None,
    }
  }

  Some((voted, no_vote))
}
