import { ElectionParameters } from 'models/election';

export interface GetVotingVectorInput {
  candidates: number[]; // 0...(n-1)

  encryptedLocation: bigint;
  electionParams: ElectionParameters;
  questionIndex: number; // 0...(m-1)
}

export interface GetVotingVectorOutput {
  forwardVector: bigint;
  reverseVector: bigint;
}

/**
 * Compute the binary voting vector for the given question
 *
 * @param input Input parameters to the function
 * @returns Forward and reverse voting vector
 */
export const getVotingVector = ({
  candidates,
  encryptedLocation,
  electionParams,
  questionIndex,
}: GetVotingVectorInput): GetVotingVectorOutput => {
  const numCandidates = BigInt(electionParams.questions[questionIndex].numCandidates);
  const numRegistered = BigInt(electionParams.numRegistered);

  // Set the corresponding bit for each candidate in the list
  const forwardBitsSet = candidates.reduce((prev, current) => prev | (BigInt(1) << BigInt(current)), BigInt(0));
  const reverseBitsSet = candidates.reduce(
    (prev, current) => prev | (BigInt(1) << (numCandidates - (BigInt(current) + BigInt(1)))),
    BigInt(0),
  );

  // Compute the bit vectors
  const forwardVector = forwardBitsSet << (encryptedLocation * numCandidates);
  const reverseVector = reverseBitsSet << ((numRegistered - (encryptedLocation + BigInt(1))) * numCandidates);

  return { forwardVector, reverseVector };
};
