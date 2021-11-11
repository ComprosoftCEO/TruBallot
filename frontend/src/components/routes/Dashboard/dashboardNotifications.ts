import { useMemo } from 'react';
import { APIResult, apiSuccess, axiosApi, getErrorInformation, resolveResult } from 'api';
import { AllElectionsResult, ElectionStatus, PublicElectionDetails, PublicElectionList } from 'models/election';
import {
  buildNotificationHandler,
  ElectionCreatedEvent,
  ElectionPublishedEvent,
  GlobalEvents,
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
  NameChangedEvent,
} from 'notifications';
import { mergeNestedState } from 'redux/helpers';
import { isDev } from 'env';
import { getUserId } from 'redux/auth';

const mergeState = mergeNestedState('dashboard');
const mergeGlobalsState = mergeNestedState('globals');

//
// List of all event handlers for the menu component
//
const onMessageMenu = buildNotificationHandler({
  [GlobalEvents.NameChanged]: handleNameChanged,
});

const MENU_SUBSCRIBE_TO: WebsocketSubscriptionList = {
  globalEvents: [GlobalEvents.NameChanged],
};

/**
 * Hook to handle the name change in the election menu
 */
export const useDashboardMenuNotifications = () => {
  useNotifications({ subscribeTo: MENU_SUBSCRIBE_TO, onMessage: onMessageMenu });
};

function handleNameChanged(event: NameChangedEvent): void {
  mergeGlobalsState({ name: event.newName });
}

//
// List of all event handlers for the election list pages
//
const onMessageElectionList = buildNotificationHandler({
  [GlobalEvents.ElectionCreated]: handleElectionCreated,
  [GlobalEvents.ElectionPublished]: handleElectionPublished,
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
 * Hook to handle notifications for filtered elections
 * @param filteredElections List of filtered elections
 */
export const useElectionListNotifications = (filteredElections: APIResult<PublicElectionList[]>) => {
  const subscribeTo: WebsocketSubscriptionList = useMemo(
    () => ({
      globalEvents: [GlobalEvents.ElectionCreated, GlobalEvents.ElectionPublished],
      elections:
        filteredElections.loading || !filteredElections.success
          ? []
          : filteredElections.data.map((election) => election.id),
    }),
    [filteredElections],
  );

  useNotifications({ subscribeTo, onMessage: onMessageElectionList });
};

function handleElectionCreated(event: ElectionCreatedEvent): void {
  fetchElection(event.electionId, (election) => {
    mergeElectionList(({ userElections }) => ({
      userElections: appendNoDuplicates(userElections, election),
    }));
  });
}

function handleElectionPublished(event: ElectionPublishedEvent): void {
  const userId = getUserId();
  fetchElection(event.electionId, (election) =>
    mergeElectionList((data) => {
      const publicElections = election.isPublic
        ? appendNoDuplicates(data.publicElections, election)
        : data.publicElections;

      const userElections =
        election.createdBy.id === userId ? appendNoDuplicates(data.userElections, election) : data.userElections;

      return { publicElections, userElections };
    }),
  );
}

function handleElectionUpdated(event: ElectionUpdatedEvent): void {
  fetchElection(event.electionId, (election) => mergeElection(event.electionId, election));
}

function handleElectionDeleted(event: ElectionDeletedEvent): void {
  mergeElectionList((data) => ({
    publicElections: data.publicElections.filter((e) => e.id !== event.electionId),
    userElections: data.userElections.filter((e) => e.id !== event.electionId),
    registeredElections: data.registeredElections.filter((e) => e.id !== event.electionId),
  }));
}

function handleRegistrationOpened(event: RegistrationOpenedEvent): void {
  mergeElection(event.electionId, { status: ElectionStatus.Registration });
}

function handleUserRegistered(event: UserRegisteredEvent): void {
  const userId = getUserId();
  mergeElection(event.electionId, (election) => ({
    numRegistered: event.numRegistered,
    isRegistered: userId === event.userId ? true : election.isRegistered,
  }));
}

function handleUserUnregistered(event: UserUnregisteredEvent): void {
  const userId = getUserId();
  mergeElection(event.electionId, (election) => ({
    numRegistered: event.numRegistered,
    isRegistered: userId === event.userId ? false : election.isRegistered,
  }));
}

function handleRegistrationClosed(event: RegistrationClosedEvent): void {
  mergeElection(event.electionId, { status: ElectionStatus.InitFailed });
}

function handleVotingOpened(event: VotingOpenedEvent): void {
  mergeElection(event.electionId, { status: ElectionStatus.Voting });
}

function handleVoteReceived(event: VoteReceivedEvent): void {
  const userId = getUserId();
  if (event.userId === userId) {
    mergeElection(event.electionId, { hasVotedStatus: event.hasVotedStatus });
  }
}

function handleVotingClosed(event: VotingClosedEvent): void {
  mergeElection(event.electionId, { status: ElectionStatus.CollectionFailed });
}

function handleResultsPublished(event: ResultsPublishedEvent): void {
  mergeElection(event.electionId, { status: ElectionStatus.Finished });
}

/**
 * Fetch the election details from the API backend
 *
 * @param electionId ID of the election
 * @param handler Handler after fetching the election
 */
async function fetchElection(
  electionId: string,
  handler: (input: PublicElectionList, election: PublicElectionDetails) => void,
): Promise<void> {
  const result = await axiosApi.get<PublicElectionDetails>(`/elections/${electionId}`).then(...resolveResult);
  if (!result.success) {
    if (isDev()) {
      const errorInfo = getErrorInformation(result.error);
      // eslint-disable-next-line no-console
      console.error(`Failed to fetch election '${electionId}': ${errorInfo.description}`);
    }
  } else {
    handler(electionDetailsToElectionList(result.data), result.data);
  }
}

/**
 * Convert a PublicElectionDetails object to an ElectionList object
 *
 * @param election Election Details object
 * @returns Election List object
 */
function electionDetailsToElectionList(election: PublicElectionDetails): PublicElectionList {
  return {
    id: election.id,
    name: election.name,
    status: election.status,
    isPublic: election.isPublic,
    createdBy: election.createdBy,

    isRegistered: election.isRegistered,
    hasVotedStatus: election.hasVotedStatus,
    numRegistered: election.registered.length,
    numQuestions: election.questions.length,
  };
}

/**
 * Helpful function to update the lists
 *
 * @param handler
 */
function mergeElectionList(handler: (input: AllElectionsResult) => Partial<AllElectionsResult>): void {
  mergeState(({ data }) => {
    if (data.loading || !data.success) return {};
    return { data: apiSuccess({ ...data.data, ...handler(data.data) }) };
  });
}

/**
 * Append an election to the list, skipping the append if there are any duplicates.
 * It then sorts the list by name .
 *
 * @param list List to modify
 * @param newElection New election to append
 * @returns New List (or exisitng list if not appended)
 */
function appendNoDuplicates(list: PublicElectionList[], newElection: PublicElectionList): PublicElectionList[] {
  // Don't append if the election already exists in the list
  if (list.find((election) => election.id === newElection.id)) {
    return list;
  }

  // Append and then sort the list
  return [...list, newElection].sort((a, b) => a.name.localeCompare(b.name));
}

/**
 * Helpful utility function to update specific election properties given the election ID
 *
 * @param id Election ID
 * @param newDetails New data for the election
 */
function mergeElection(
  id: string,
  newDetails: Partial<PublicElectionList> | ((input: PublicElectionList) => Partial<PublicElectionList>),
): void {
  mergeState(({ data }) => {
    if (data.loading || !data.success) return {};
    const allElections = data.data;

    // Update public elections
    const inPublicElections = allElections.publicElections.findIndex((e) => e.id === id);
    const publicElections: PublicElectionList[] =
      inPublicElections > -1 ? [...allElections.publicElections] : allElections.publicElections;
    if (inPublicElections > -1) {
      if (typeof newDetails === 'function') {
        publicElections[inPublicElections] = {
          ...publicElections[inPublicElections],
          ...newDetails(publicElections[inPublicElections]),
        };
      } else {
        publicElections[inPublicElections] = { ...publicElections[inPublicElections], ...newDetails };
      }
    }

    // Update user elections
    const inUserElections = allElections.userElections.findIndex((e) => e.id === id);
    const userElections: PublicElectionList[] =
      inUserElections > -1 ? [...allElections.userElections] : allElections.userElections;
    if (inUserElections > -1) {
      if (typeof newDetails === 'function') {
        userElections[inUserElections] = {
          ...userElections[inUserElections],
          ...newDetails(userElections[inUserElections]),
        };
      } else {
        userElections[inUserElections] = { ...userElections[inUserElections], ...newDetails };
      }
    }

    // Update registered elections
    const inRegisteredElections = allElections.registeredElections.findIndex((e) => e.id === id);
    const registeredElections: PublicElectionList[] =
      inRegisteredElections > -1 ? [...allElections.registeredElections] : allElections.registeredElections;
    if (inRegisteredElections > -1) {
      if (typeof newDetails === 'function') {
        registeredElections[inRegisteredElections] = {
          ...registeredElections[inRegisteredElections],
          ...newDetails(registeredElections[inRegisteredElections]),
        };
      } else {
        registeredElections[inRegisteredElections] = { ...registeredElections[inRegisteredElections], ...newDetails };
      }
    }

    return {
      data: apiSuccess({
        publicElections,
        userElections,
        registeredElections,
      }),
    };
  });
}
