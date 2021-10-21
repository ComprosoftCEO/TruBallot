use curv_kzen::arithmetic::traits::Converter;
use curv_kzen::BigInt;
use std::iter;

///
/// Test the validity of the voting vector, then count the number of votes
///
/// On error, this returns None
/// On success, it returns the number of votes for each candidate
///
pub fn count_ballot_votes(
  forward_ballot: &BigInt,
  reverse_ballot: &BigInt,
  num_candidates: i64,
  num_voters: i64,
  no_vote_count: usize,
) -> Option<Vec<i64>> {
  let total_bits = (num_candidates * num_voters) as usize;
  let forward_bits = get_bits(forward_ballot, total_bits);
  let reverse_bits = get_bits(reverse_ballot, total_bits);

  // Step 1: Make sure the forward and reverse bits match
  for (f, r) in forward_bits.iter().zip(reverse_bits.iter().rev()) {
    if *f != *r {
      return None;
    }
  }

  // Step 2: Make sure each chunk has, at most, only one candidate selected
  let chunks = get_chunks(&forward_bits, num_candidates as usize);
  let (voted, no_vote) = match verify_chunks(&chunks) {
    Some((voted, no_vote)) => (voted, no_vote),
    None => return None,
  };

  // Step 3: Make sure the number of voted bits matches the users who voted
  if voted + no_vote != total_bits || no_vote != no_vote_count {
    return None;
  }

  // All checks passed!
  Some(count_votes(&chunks, num_candidates as usize))
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

/// Total up the votes for each candidate
///
/// This function assumes that the chunk input has already been validated by verify_chunks
fn count_votes(input: &Vec<Vec<bool>>, num_candidates: usize) -> Vec<i64> {
  let mut votes = vec![0; num_candidates];

  for chunk in input {
    // Searches for the first candidate bit set in the chunk
    if let Some(candidate) = chunk
      .into_iter()
      .rev()
      .enumerate()
      .filter_map(|(i, c)| (*c).then(|| i))
      .next()
    {
      votes[candidate] += 1;
    }
  }

  votes
}
