// State Transition Diagram:
//
//   [Draft] -> [Registration] -> [Voting] -> [Finished]
//      V
//   <Delete>
//
// Note: Elections can ONLY be edited or deleted in the [Draft] state.
//   Once registration has begun, the election must be carried to the end.
//
// Note: Public elections are not visible until the [Registration] state.
sql_enum!(
  pub ElectionStatus {
    Draft,
    Registration = 1,
    Voting,
    Finished
  }
);
