import { useMemo } from 'react';
import { APISuccess, apiSuccess, axiosApi, getErrorInformation, resolveResult } from 'api';
import { ElectionStatus, HasVotedStatus, PublicElectionDetails } from 'models/election';
import {
  buildNotificationHandler,
  GlobalEvents,
  ElectionEvents,
  useNotifications,
  WebsocketSubscriptionList,
  CollectorPublishedOrUpdatedEvent,
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
import { getNestedState, mergeNestedState } from 'redux/helpers';
import { isDev } from 'env';
import { getUserId } from 'redux/auth';
import { showConfirm } from 'showConfirm';
import { history } from 'index';
import { PublicCollectorList } from 'models/mediator';

const getState = getNestedState('manageElection');
const mergeState = mergeNestedState('manageElection');

//
// List of all event handlers for the manage election page
//
const onMessageManageElection = buildNotificationHandler({
  [GlobalEvents.CollectorPublishedOrUpdated]: handleCollectorPublishedOrUpdated,
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
  const subscribeTo: WebsocketSubscriptionList = useMemo(
    () => ({ globalEvents: [GlobalEvents.CollectorPublishedOrUpdated], elections: [electionId] }),
    [electionId],
  );

  useNotifications({ subscribeTo, onMessage: onMessageManageElection });
};

function handleCollectorPublishedOrUpdated(event: CollectorPublishedOrUpdatedEvent): void {
  const { allCollectors, electionCollectors } = getState();

  // Append or update the list of all collectors
  if (!allCollectors.loading && allCollectors.success) {
    // Figure out if this is an INSERT or an UPDATE
    const newCollectors = [...allCollectors.data];
    const collectorIndex = allCollectors.data.findIndex((collector) => collector.id === event.id);
    if (collectorIndex < 0) {
      // Append to the list of existing collectors
      newCollectors.push({ id: event.id, name: event.name });
    } else {
      // Update the item in the list
      newCollectors[collectorIndex].name = event.name;
    }

    // Either way, always re-sort the list
    newCollectors.sort((a, b) => a.name.localeCompare(b.name));
    mergeState({ allCollectors: apiSuccess(newCollectors) });
  }

  // Also attempt to update the name in election collectors
  //  Only do this if the collector is actually in the list
  if (!electionCollectors.loading && electionCollectors.success) {
    const collectorIndex = electionCollectors.data.findIndex((collector) => collector.id === event.id);
    if (collectorIndex >= 0) {
      // Update the element and re-sort the list
      const newElectionCollectors = [...electionCollectors.data];
      newElectionCollectors[collectorIndex].name = event.name;
      newElectionCollectors.sort((a, b) => a.name.localeCompare(b.name));

      mergeState({ electionCollectors: apiSuccess(newElectionCollectors) });
    }
  }
}

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
  // Build the list of selected election collectors
  const { allCollectors } = getState();
  const collectorList: PublicCollectorList[] = event.collectors
    .map((id, index) => ({
      id,

      // Find the collector name in the list, or replace with a dummy name if not found
      name:
        (allCollectors as APISuccess<PublicCollectorList[]>)?.data.find((c) => c.id === id)?.name ??
        `Collector ${index + 1}`,
    }))
    .sort((a, b) => a.name.localeCompare(b.name));

  mergeState({ electionCollectors: apiSuccess(collectorList) });
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
