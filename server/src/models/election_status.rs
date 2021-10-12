// State Transition Diagram:
//
//   [Draft] -> [Registration] -> [Voting] ----------> [Finished]
//      V        V                 ^  V                      ^
//   <Delete>    \-> [InitFailed] -/  \->[CollectionFailed] -/
//
// Note: Elections can ONLY be edited or deleted in the [Draft] state.
//   Once registration has begun, the election must be carried to the end.
//
// Note: Public elections are not visible until the [Registration] state.
sql_enum!(
  pub ElectionStatus {
    Draft = 0,
    Registration,
    InitFailed,
    Voting,
    CollectionFailed,
    Finished
  }
);

impl ElectionStatus {
  pub fn get_name(&self) -> &'static str {
    match self {
      ElectionStatus::Draft => "Draft",
      ElectionStatus::Registration => "Registration",
      ElectionStatus::InitFailed => "Initialization Failed",
      ElectionStatus::Voting => "Voting",
      ElectionStatus::CollectionFailed => "Collection Failed",
      ElectionStatus::Finished => "Finished",
    }
  }

  /// Test if the election parameters have been initialized
  pub fn is_initialized(&self) -> bool {
    match self {
      ElectionStatus::Draft => false,
      ElectionStatus::Registration => false,
      ElectionStatus::InitFailed => false,
      ElectionStatus::Voting => true,
      ElectionStatus::CollectionFailed => true,
      ElectionStatus::Finished => true,
    }
  }
}
