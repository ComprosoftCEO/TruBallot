/*
 * State for the voting interface
 */
import { APIResult, apiLoading } from 'api';
import { CollectorElectionParameters, ElectionParameters, PublicElectionDetails } from 'models/election';
import { PublicCollectorList } from 'models/mediator';

export interface VoteState {
  electionDetails: APIResult<PublicElectionDetails>;
  electionParams: APIResult<ElectionParameters>;
  electionCollectors: APIResult<PublicCollectorList[]>;
  collectorRequests: Record<string, APIResult<CollectorElectionParameters>>;

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
  electionCollectors: apiLoading(),
  collectorRequests: {},

  questions: [],
  cheatMode: false,
  encryptedLocation: BigInt(-1),

  votingStatus: VotingStatus.Init,
};
