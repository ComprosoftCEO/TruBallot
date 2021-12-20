import { CollectorQuestionParameters, ElectionParameters } from 'models/election';
import { modPow, toZn } from 'bigint-mod-arith';

export interface ComputeBallotInput {
  forwardVector: bigint;
  reverseVector: bigint;

  electionParams: ElectionParameters;
  collectorParams: CollectorQuestionParameters[];
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
  collectorParams,
}: ComputeBallotInput): ComputeBallotOutput => {
  const prime = BigInt(electionParams.prime);
  const modulus = prime - BigInt(1);

  // Sum together all the shares
  const forwardVerificationShares = collectorParams.reduce(
    (acc, params) => acc + BigInt(params.forwardVerificationShares),
    BigInt(0),
  );

  const reverseVerificationShares = collectorParams.reduce(
    (acc, params) => acc + BigInt(params.forwardVerificationShares),
    BigInt(0),
  );

  const forwardBallotShares = collectorParams.reduce(
    (acc, params) => acc + BigInt(params.forwardBallotShares),
    BigInt(0),
  );

  const reverseBallotShares = collectorParams.reduce(
    (acc, params) => acc + BigInt(params.reverseBallotShares),
    BigInt(0),
  );

  // Compute the secrets
  const secret = toZn(forwardVector - forwardVerificationShares, modulus);
  const secretPrime = toZn(reverseVector - reverseVerificationShares, modulus);

  // Compute the commitments
  const g = BigInt(electionParams.generator);
  const gS = modPow(g, secret, prime);
  const gSPrime = modPow(g, secretPrime, prime);
  const gSSPrime = modPow(g, secret * secretPrime, prime);

  // Compute the ballots
  const forwardBallot = toZn(secret + forwardBallotShares, modulus);
  const reverseBallot = toZn(secretPrime + reverseBallotShares, modulus);

  return { forwardBallot, reverseBallot, gS, gSPrime, gSSPrime };
};
