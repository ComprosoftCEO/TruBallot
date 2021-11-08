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

export interface ParseVectorEntry {
  candidatePicked: number | null | undefined; // "null" = No vote, "undefined" = Invalid
  bits: string;
}

/**
 * Parse the voting vector
 *
 * @param vector Voting vector to parse
 * @param numCandidates Number of candidates
 * @param numRegistered Number of registered users
 * @param reverse If true, reverses the order of the candidates
 *
 * @returns Vector of parsed entries
 */
export const parseVotingVector = (
  vector: bigint,
  numCandidates: number,
  numRegistered: number,
  reverse = false,
): ParseVectorEntry[] => {
  // Convert the voting vector into a bit string with L = n*m bits
  const totalBits = numCandidates * numRegistered;
  const bitString = vector.toString(2).padStart(totalBits, '0');

  // Split string into chunks of bit strings
  const chunks = [];
  for (let i = 0; i < numRegistered; i += 1) {
    chunks.push(bitString.slice(i * numCandidates, (i + 1) * numCandidates));
  }

  // Now parse each chunk to find the candidate index
  return chunks.map((bits) => {
    // Make sure at least 1 bit is set
    const index = bits.indexOf('1');
    if (index < 0) {
      // No bits set, so return NULL for "no vote"
      return { candidatePicked: null, bits };
    }

    // Test to see if two bits are set
    const nextIndex = bits.indexOf('1', index + 1);
    if (nextIndex >= 0) {
      // Picked at least two candidates, so invalid vector!
      return { candidatePicked: undefined, bits };
    }

    // We only picked one candidate
    // If the bits are in the reverse order, adjust the calculations
    const candidatePicked = reverse ? index : numCandidates - (index + 1);
    return { candidatePicked, bits };
  });
};
