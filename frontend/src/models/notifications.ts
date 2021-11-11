import { HasVotedStatus } from './election';

// Events not attached to any specific election
export enum GlobalEvents {
  ElectionCreated = 'electionCreated',
  ElectionPublished = 'electionPublished',
  NameChanged = 'nameChanged',
}

// Events specific to an election
export enum ElectionEvents {
  ElectionUpdated = 'electionUpdated',
  ElectionDeleted = 'electionDeleted',
  RegistrationOpened = 'registrationOpened',
  UserRegistered = 'userRegistered',
  UserUnregistered = 'userUnregistered',
  RegistrationClosed = 'registrationClosed',
  VotingOpened = 'votingOpened',
  VoteReceived = 'voteReceived',
  VotingClosed = 'votingClosed',
  ResultsPublished = 'resultsPublished',
}

/**
 * Update which events the websocket is subscribed to
 */
export type WebsocketSubscriptionRequest = SubscribeRequest | UnsubscribeRequest | UnsubscribeAllRequest;

export enum SubscriptionAction {
  Subscribe = 'subscribe',
  Unsubscribe = 'unsubscribe',
  Replace = 'replace',
  UnsubscribeAll = 'unsubscribeAll',
}

export interface WebsocketSubscriptionList {
  // Specific global events
  globalEvents?: GlobalEvents[];

  // All events from a given election (Uuid)
  elections?: string[];

  // Specific events from a given election
  electionEvents?: Record<string, ElectionEvents[]>;
}

export interface SubscribeRequest extends WebsocketSubscriptionList {
  type: SubscriptionAction.Subscribe;
}

export interface UnsubscribeRequest extends WebsocketSubscriptionList {
  type: SubscriptionAction.Unsubscribe;
}

export interface ReplaceRequest extends WebsocketSubscriptionList {
  type: SubscriptionAction.Replace;
}

export interface UnsubscribeAllRequest {
  type: SubscriptionAction.UnsubscribeAll;
}

/**
 * Data returned from the websocket subscription request
 */
export type WebsocketSubscriptionResponse = WebsocketSuccessResponse | WebsocketErrorResponse;

export interface WebsocketSuccessResponse {
  type: 'success';
}

export interface WebsocketErrorResponse {
  type: 'error';
  developerNotes?: string;
}

/**
 * Event that can be received from the websocket
 */
export type WebsocketNotificationEvent =
  | ElectionCreatedEvent
  | ElectionPublishedEvent
  | NameChangedEvent
  | ElectionUpdatedEvent
  | ElectionDeletedEvent
  | RegistrationOpenedEvent
  | UserRegisteredEvent
  | UserUnregisteredEvent
  | RegistrationClosedEvent
  | VotingOpenedEvent
  | VoteReceivedEvent
  | VotingClosedEvent
  | ResultsPublishedEvent;

export interface ElectionCreatedEvent {
  type: GlobalEvents.ElectionCreated;
  electionId: string;
}

export interface ElectionPublishedEvent {
  type: GlobalEvents.ElectionPublished;
  electionId: string;
}

export interface NameChangedEvent {
  type: GlobalEvents.NameChanged;
  newName: string;
}

export interface ElectionUpdatedEvent {
  type: ElectionEvents.ElectionUpdated;
  electionId: string;
}

export interface ElectionDeletedEvent {
  type: ElectionEvents.ElectionDeleted;
  electionId: string;
}

export interface RegistrationOpenedEvent {
  type: ElectionEvents.RegistrationOpened;
  electionId: string;
}

export interface UserRegisteredEvent {
  type: ElectionEvents.UserRegistered;
  electionId: string;

  userId: string;
  userName: string;
  numRegistered: number;
}

export interface UserUnregisteredEvent {
  type: ElectionEvents.UserUnregistered;
  electionId: string;
  userId: string;
  numRegistered: number;
}

export interface RegistrationClosedEvent {
  type: ElectionEvents.RegistrationClosed;
  electionId: string;
  isPublic: boolean;
}

export interface VotingOpenedEvent {
  type: ElectionEvents.VotingOpened;
  electionId: string;
}

export interface VoteReceivedEvent {
  type: ElectionEvents.VoteReceived;
  electionId: string;
  questionId: string;

  userId: string;
  userName: string;
  hasVotedStatus: HasVotedStatus;

  forwardBallot: string; // BigInt
  reverseBallot: string; // BigInt

  gS: string; // BigInt
  gSPrime: string; // BigInt
  gSSPrime: string; // BigInt

  numVotes: number;
}

export interface VotingClosedEvent {
  type: ElectionEvents.VotingOpened;
  electionId: string;
}

export interface ResultsPublishedEvent {
  type: ElectionEvents.ResultsPublished;
  electionId: string;
}
