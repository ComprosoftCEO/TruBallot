export * from './useNotifications';
export * from './notificationMap';

// Re-exports from the models
export { GlobalEvents, ElectionEvents } from 'models/notifications';
export type {
  WebsocketSubscriptionList,
  WebsocketNotificationEvent,
  ElectionCreatedEvent,
  ElectionPublishedEvent,
  NameChangedEvent,
  ElectionUpdatedEvent,
  ElectionDeletedEvent,
  RegistrationOpenedEvent,
  UserRegisteredEvent,
  UserUnregisteredEvent,
  RegistrationClosedEvent,
  VotingOpenedEvent,
  VoteReceivedEvent,
  VotingClosedEvent,
  ResultsPublishedEvent,
} from 'models/notifications';
