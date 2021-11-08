import { CollectorQuestionParameters, ElectionParameters } from 'models/election';
import { modPow, toZn } from 'bigint-mod-arith';

export interface ComputeBallotInput {
  forwardVector: bigint;
  reverseVector: bigint;

  electionParams: ElectionParameters;
  c1QuestionParams: CollectorQuestionParameters;
  c2QuestionParams: CollectorQuestionParameters;
}

export interface ComputeBallotOutput {
  forwardBallot: bigint;
  reverseBallot: bigint;

  gS: bigint;
  gSPrime: bigint;
  gSSPrime: bigint;
}

/**
 * Compute the entire ballot to submit to the backend
 *
 * @param input Parameters
 * @returns Output data
 */
export const computeBallot = ({
  forwardVector,
  reverseVector,
  electionParams,
  c1QuestionParams,
  c2QuestionParams,
}: ComputeBallotInput): ComputeBallotOutput => {
  const prime = BigInt(electionParams.prime);
  const modulus = prime - BigInt(1);

  // Compute the secrets
  const secret = toZn(
    forwardVector -
      BigInt(c1QuestionParams.forwardVerificationShares) -
      BigInt(c2QuestionParams.forwardVerificationShares),
    modulus,
  );

  const secretPrime = toZn(
    reverseVector -
      BigInt(c1QuestionParams.reverseVerificationShares) -
      BigInt(c2QuestionParams.reverseVerificationShares),
    modulus,
  );

  // Compute the commitments
  const g = BigInt(electionParams.generator);
  const gS = modPow(g, secret, prime);
  const gSPrime = modPow(g, secretPrime, prime);
  const gSSPrime = modPow(g, secret * secretPrime, prime);

  // Compute the ballots
  const forwardBallot = toZn(
    secret + BigInt(c1QuestionParams.forwardBallotShares) + BigInt(c2QuestionParams.forwardBallotShares),
    modulus,
  );

  const reverseBallot = toZn(
    secretPrime + BigInt(c1QuestionParams.reverseBallotShares) + BigInt(c2QuestionParams.reverseBallotShares),
    modulus,
  );

  return { forwardBallot, reverseBallot, gS, gSPrime, gSSPrime };
};
