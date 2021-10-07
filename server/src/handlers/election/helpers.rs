use validator::ValidationError;

///
/// Custom validator function for the individual candidates
///   Checks to make sure the string has between 1 and 255 characters
///
pub fn validate_candidates(candidates: &Vec<String>) -> Result<(), ValidationError> {
  // Each candidate must have between 1 and 255 characters in the string
  for candidate in candidates {
    if candidate.len() < 1 || candidate.len() > 255 {
      return Err(ValidationError::new(
        "length [{\"max\": Number(255), \"value\": String(\"\"), \"min\": Number(1)}]",
      ));
    }
  }

  Ok(())
}
