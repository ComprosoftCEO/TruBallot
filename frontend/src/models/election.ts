export enum ElectionStatus {
  Draft = 0,
  Registration,
  InitFailed,
  Voting,
  CollectionFailed,
  Finished,
}

export interface NewElectionResult {
  id: string;
}

export interface AllElectionsResult {
  publicElections: PublicElectionList[];
  userElections: PublicElectionList[];
  registeredElections: PublicElectionList[];
}

export interface PublicElectionList {
  id: string;
  name: string;
  status: ElectionStatus;
  isPublic: boolean;

  isRegistered: boolean;
  hasVoted: boolean;
  numRegistered: number;
  numQuestions: number;
}

export type GetElectionByAccessCode = NewElectionResult;

export interface PublicElectionDetails {
  id: string;
  name: string;
  createdBy: UserDetails;
  status: ElectionStatus;

  isPublic: boolean;
  access_code?: string;

  isRegistered: boolean;
  hasVoted: boolean;
  registered: UserDetails[];
  questions: PublicElectionQuestion[];
}

export interface UserDetails {
  id: string;
  name: string;
}

export interface PublicElectionQuestion {
  id: string;
  name: string;
  numVotesReceived: number;
  candidates: string[];
}

export interface ElectionParameters {
  numRegistered: number;
  questions: QuestionParameters[];

  generator: string; // BigInt
  prime: string; // BigInt
}

export interface QuestionParameters {
  numCandidates: number;
}

export interface ElectionResult {
  questionResults: Record<string, QuestionResult>;
}

export interface QuestionResult {
  forwardBallots: string; // BigInt
  reverseBallots: string; // BigInt
  ballotValid: boolean;

  forwardCancelationShares: string; // BigInt
  reverseCancelationShares: string; // BigInt

  candidateVotes: Record<string, CandidateResult>;
  userBallots: UserBallotResult[];
  noVotes: UserDetails[];
}

export interface UserBallotResult {
  id: string;
  name: String;

  forwardBallot: string; // BigInt
  reverseBallot: string; // BigInt

  gS: string; // BigInt
  gSPrime: string; // BigInt
  gSSPrime: string; // BigInt
}

export interface CandidateResult {
  numVotes: number | null;
}
