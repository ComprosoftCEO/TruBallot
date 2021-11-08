export interface VerifyBallotData {
  userId: string;

  // Ballots
  forwardBallot: string; // BigInt: p_i
  reverseBallot: string; // BigInt: p_i'

  // Commitments
  gS: string; // BigInt: g^(s_i)
  gSPrime: string; // BigInt: g^(s_i')
  gSSPrime: string; // BigInt: g^(s_i * s_i')
}

export interface VerificationResult {
  subProtocol1: boolean;
  subProtocol2: boolean;
}
