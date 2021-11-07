/*
 * State for the voting interface
 */
import { APIResult, apiLoading } from 'api';
import { PublicElectionDetails } from 'models/election';

export interface VoteState {
  electionDetails: APIResult<PublicElectionDetails>;

  questions: QuestionDetails[];
  cheatMode: boolean;
}

// Details needed to update the question
export interface QuestionDetails {
  id: string;
  name: string;
  candidates: string[];
  hasVoted: boolean;

  choices: Set<number>;
  voting: APIResult<{}>;
}

export const initialVoteState: VoteState = {
  electionDetails: apiLoading(),

  questions: [],
  cheatMode: false,
};
