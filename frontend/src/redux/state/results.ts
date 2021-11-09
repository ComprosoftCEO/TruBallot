/**
 * State used to view the election results
 */
import { APIResult, apiLoading, APIOption, apiSome } from 'api';
import {
  CollectorElectionParameters,
  ElectionParameters,
  ElectionResult,
  PublicElectionDetails,
  QuestionResult,
  UserBallotResult,
} from 'models/election';
import { VerificationResult } from 'models/verification';

export interface ResultsState {
  electionDetails: APIResult<PublicElectionDetails>;
  electionParams: APIResult<ElectionParameters>;
  electionResults: APIResult<ElectionResult>;
  c1Params: APIResult<CollectorElectionParameters>;
  c2Params: APIResult<CollectorElectionParameters>;

  questions: ExtendedQuestionResult[];
  currentQuestionIndex: number;
  generator: bigint;
  prime: bigint;
  verifySum: APIOption<[bigint, bigint] | null>;

  // Only if registered in the election
  encryptedLocation: bigint | null;
}

export interface ExtendedQuestionResult extends QuestionResult {
  id: string;
  name: string;
  candidates: ExtendedCandidatesResult[];
  ballots: ExtendedBallotsResult[];

  currentTab: number;
  prevTab: number;
  vectorTab: number;
  rawTab: number;

  showVote: boolean;
}

export interface ExtendedCandidatesResult {
  name: string;
  numVotes?: number;
}

export interface ExtendedBallotsResult extends UserBallotResult {
  verifying: APIResult<VerificationResult | undefined>;
}

export const initialResultsState: ResultsState = {
  electionDetails: apiLoading(),
  electionParams: apiLoading(),
  electionResults: apiLoading(),

  c1Params: apiLoading(),
  c2Params: apiLoading(),

  questions: [],
  currentQuestionIndex: 0,
  generator: BigInt(0),
  prime: BigInt(0),
  verifySum: apiSome(null),

  encryptedLocation: null,
};
