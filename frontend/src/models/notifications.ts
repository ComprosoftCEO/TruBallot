// Events not attached to any specific election
export enum GlobalEvents {
  ElectionPublished = 'electionPublished',
}

// Events specific to an election
export enum ElectionEvents {
  RegistrationOpened = 'registrationOpened',
  RegistrationCountUpdated = 'registrationCountUpdated',
  RegistrationClosed = 'registrationClosed',
  VotingOpened = 'votingOpened',
  VoteCountUpdated = 'voteCountUpdated',
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
  | ElectionPublishedEvent
  | RegistrationOpenedEvent
  | RegistrationCountUpdatedEvent
  | RegistrationClosedEvent
  | VotingOpenedEvent
  | VoteCountUpdatedEvent
  | VotingClosedEvent
  | ResultsPublishedEvent;

export interface ElectionPublishedEvent {
  type: GlobalEvents.ElectionPublished;
  electionId: string;
}

export interface RegistrationOpenedEvent {
  type: ElectionEvents.RegistrationOpened;
  electionId: string;
}

export interface RegistrationCountUpdatedEvent {
  type: ElectionEvents.RegistrationCountUpdated;
  electionId: string;
  numRegistered: number;
}

export interface RegistrationClosedEvent {
  type: ElectionEvents.RegistrationClosed;
  electionId: string;
}

export interface VotingOpenedEvent {
  type: ElectionEvents.VotingOpened;
  electionId: string;
}

export interface VoteCountUpdatedEvent {
  type: ElectionEvents.VoteCountUpdated;
  questionId: string;
  newCount: string;
}

export interface VotingClosedEvent {
  type: ElectionEvents.VotingOpened;
  electionId: string;
}

export interface ResultsPublishedEvent {
  type: ElectionEvents.ResultsPublished;
  electionId: string;
}
