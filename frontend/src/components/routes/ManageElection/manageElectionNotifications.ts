import { useMemo } from 'react';
import { apiSuccess, axiosApi, getErrorInformation, resolveResult } from 'api';
import { ElectionStatus, HasVotedStatus, PublicElectionDetails } from 'models/election';
import {
  buildNotificationHandler,
  ElectionEvents,
  useNotifications,
  WebsocketSubscriptionList,
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
} from 'notifications';
import { mergeNestedState } from 'redux/helpers';
import { isDev } from 'env';
import { getUserId } from 'redux/auth';
import { showConfirm } from 'showConfirm';
import { history } from 'index';

const mergeState = mergeNestedState('manageElection');

//
// List of all event handlers for the manage election page
//
const onMessageManageElection = buildNotificationHandler({
  [ElectionEvents.ElectionUpdated]: handleElectionUpdated,
  [ElectionEvents.ElectionDeleted]: handleElectionDeleted,
  [ElectionEvents.RegistrationOpened]: handleRegistrationOpened,
  [ElectionEvents.UserRegistered]: handleUserRegistered,
  [ElectionEvents.UserUnregistered]: handleUserUnregistered,
  [ElectionEvents.RegistrationClosed]: handleRegistrationClosed,
  [ElectionEvents.VotingOpened]: handleVotingOpened,
  [ElectionEvents.VoteReceived]: handleVoteReceived,
  [ElectionEvents.VotingClosed]: handleVotingClosed,
  [ElectionEvents.ResultsPublished]: handleResultsPublished,
});

/**
 * Hook to handle notifications for the manage election page
 *
 * @param electionId ID of the election on this page
 */
export const useManageElectionNotifications = (electionId: string) => {
  const subscribeTo: WebsocketSubscriptionList = useMemo(() => ({ elections: [electionId] }), [electionId]);

  useNotifications({ subscribeTo, onMessage: onMessageManageElection });
};

function handleElectionUpdated(event: ElectionUpdatedEvent): void {
  fetchElection(event.electionId, mergeElection);
}

function handleElectionDeleted(event: ElectionDeletedEvent): void {
  mergeState({ deletingElection: apiSuccess(true) });
  showConfirm({
    message: 'Election has been deleted!',
    confirmButton: 'Ok',
    cancelButton: null,
    onConfirm: () => history.push('/'),
  });
}

function handleRegistrationOpened(event: RegistrationOpenedEvent): void {
  mergeElection({ status: ElectionStatus.Registration });
}

function handleUserRegistered(event: UserRegisteredEvent): void {
  const userId = getUserId();
  mergeElection((election) => {
    // Possibly insert into the list of registered users if the user isn't already in it
    //   Be sure to sort the list after insertion to keep the order consistent
    const userIndex = election.registered.findIndex((r) => r.id === event.userId);
    const registered =
      userIndex > -1
        ? election.registered
        : [...election.registered, { id: event.userId, name: event.userName, hasVotedStatus: HasVotedStatus.No }].sort(
            (a, b) => a.name.localeCompare(b.name),
          );

    return { registered, isRegistered: event.userId === userId ? true : election.isRegistered };
  });
}

function handleUserUnregistered(event: UserUnregisteredEvent): void {
  const userId = getUserId();
  mergeElection((election) => {
    // Possibly remove the user from the list
    const userIndex = election.registered.findIndex((r) => r.id === event.userId);
    const registered = userIndex > -1 ? election.registered.filter((r) => r.id !== event.userId) : election.registered;

    return { registered, isRegistered: event.userId === userId ? false : election.isRegistered };
  });
}

function handleRegistrationClosed(event: RegistrationClosedEvent): void {
  mergeElection({ status: ElectionStatus.InitFailed, accessCode: undefined, isPublic: event.isPublic });
}

function handleVotingOpened(event: VotingOpenedEvent): void {
  mergeElection({ status: ElectionStatus.Voting });
}

function handleVoteReceived(event: VoteReceivedEvent): void {
  const userId = getUserId();
  mergeElection((election) => {
    // Update the vote status in the registered users
    const registeredIndex = election.registered.findIndex((r) => r.id === event.userId);
    const registered = registeredIndex > -1 ? [...election.registered] : election.registered;
    if (registeredIndex > -1) {
      registered[registeredIndex] = { ...registered[registeredIndex], hasVotedStatus: event.hasVotedStatus };
    }

    // Update the vote count in the questions
    const questionIndex = election.questions.findIndex((q) => q.id === event.questionId);
    const questions = questionIndex > -1 ? [...election.questions] : election.questions;
    if (questionIndex > -1) {
      questions[questionIndex] = {
        ...questions[questionIndex],
        numVotesReceived: event.numVotes,
        hasVoted: event.userId === userId ? true : questions[questionIndex].hasVoted,
      };
    }

    // Figure out the has voted status
    let hasVotedStatus = HasVotedStatus.No;
    if (questions.every((q) => q.hasVoted)) {
      hasVotedStatus = HasVotedStatus.Yes;
    } else if (questions.some((q) => q.hasVoted)) {
      hasVotedStatus = HasVotedStatus.Partial;
    }

    return { registered, questions, hasVotedStatus };
  });
}

function handleVotingClosed(event: VotingClosedEvent): void {
  mergeElection({ status: ElectionStatus.CollectionFailed });
}

function handleResultsPublished(event: ResultsPublishedEvent): void {
  mergeElection({ status: ElectionStatus.Finished });
}

/**
 * Fetch the election details from the API backend
 *
 * @param electionId ID of the election
 * @param handler Handler after fetching the election
 */
async function fetchElection(electionId: string, handler: (election: PublicElectionDetails) => void): Promise<void> {
  const result = await axiosApi.get<PublicElectionDetails>(`/elections/${electionId}`).then(...resolveResult);
  if (!result.success) {
    if (isDev()) {
      const errorInfo = getErrorInformation(result.error);
      // eslint-disable-next-line no-console
      console.error(`Failed to fetch election '${electionId}': ${errorInfo.description}`);
    }
  } else {
    handler(result.data);
  }
}

/**
 * Helpful utility function to update specific election properties
 *
 * @param newDetails New data for the election
 */
function mergeElection(
  newDetails: Partial<PublicElectionDetails> | ((input: PublicElectionDetails) => Partial<PublicElectionDetails>),
): void {
  mergeState(({ electionDetails }) => {
    if (electionDetails.loading || !electionDetails.success) return {};

    const newElection: PublicElectionDetails =
      typeof newDetails === 'function'
        ? { ...electionDetails.data, ...newDetails(electionDetails.data) }
        : { ...electionDetails.data, ...newDetails };

    return { electionDetails: apiSuccess(newElection) };
  });
}
