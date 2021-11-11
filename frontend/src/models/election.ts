export enum ElectionStatus {
  Draft = 0,
  Registration,
  InitFailed,
  Voting,
  CollectionFailed,
  Finished,
}

export enum HasVotedStatus {
  No = 0,
  Partial,
  Yes,
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
  createdBy: UserDetails;

  isRegistered: boolean;
  hasVotedStatus: HasVotedStatus;
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
  accessCode?: string;

  isRegistered: boolean;
  hasVotedStatus: HasVotedStatus;
  registered: RegisteredUserDetails[];
  questions: PublicElectionQuestion[];
}

export interface UserDetails {
  id: string;
  name: string;
}

export interface RegisteredUserDetails {
  id: string;
  name: string;
  hasVotedStatus: HasVotedStatus;
}

export interface PublicElectionQuestion {
  id: string;
  name: string;
  hasVoted: boolean;
  numVotesReceived: number;
  candidates: string[];
}

export interface PublishElectionResult {
  accessCode?: string;
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
  forwardBallots?: string; // BigInt
  reverseBallots?: string; // BigInt
  ballotValid: boolean;

  forwardCancelationShares?: string; // BigInt
  reverseCancelationShares?: string; // BigInt

  userBallots: UserBallotResult[];
  noVotes: UserDetails[];
  candidateVotes?: Record<string, CandidateResult>;
}

export interface UserBallotResult {
  id: string;
  name: string;

  forwardBallot: string; // BigInt
  reverseBallot: string; // BigInt

  gS: string; // BigInt
  gSPrime: string; // BigInt
  gSSPrime: string; // BigInt
}

export interface CandidateResult {
  numVotes: number;
}

export interface CollectorElectionParameters {
  encryptedLocation?: string; // BigInt
}

export interface CollectorQuestionParameters {
  forwardVerificationShares: string; // BigInt
  reverseVerificationShares: string; // BigInt
  forwardBallotShares: string; // BigInt
  reverseBallotShares: string; // BigInt
}
