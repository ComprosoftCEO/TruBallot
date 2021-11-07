/*
 * State for the voting interface
 */
import { APIResult, apiLoading } from 'api';
import { CollectorElectionParameters, ElectionParameters, PublicElectionDetails } from 'models/election';

export interface VoteState {
  electionDetails: APIResult<PublicElectionDetails>;
  electionParams: APIResult<ElectionParameters>;
  c1Params: APIResult<CollectorElectionParameters>;
  c2Params: APIResult<CollectorElectionParameters>;

  questions: QuestionDetails[];
  cheatMode: boolean;
  encryptedLocation: bigint;

  votingStatus: VotingStatus;
}

// Details needed to update the question
export interface QuestionDetails {
  id: string;
  name: string;
  candidates: string[];
  hasVoted: boolean;

  choices: Set<number>;
  voting: APIResult<boolean>;
}

export enum VotingStatus {
  Init = 0,
  Voting,
  Error,
  Success,
}

export const initialVoteState: VoteState = {
  electionDetails: apiLoading(),
  electionParams: apiLoading(),
  c1Params: apiLoading(),
  c2Params: apiLoading(),

  questions: [],
  cheatMode: false,
  encryptedLocation: BigInt(-1),

  votingStatus: VotingStatus.Init,
};
