import { useMemo } from 'react';
import { apiError, ErrorResponse, GlobalErrorCode, RequestError } from 'api';
import {
  buildNotificationHandler,
  ElectionEvents,
  useNotifications,
  WebsocketSubscriptionList,
  ElectionUpdatedEvent,
  ElectionDeletedEvent,
  RegistrationOpenedEvent,
} from 'notifications';
import { mergeNestedState } from 'redux/helpers';

const mergeState = mergeNestedState('editor');

//
// List of all event handlers for the edit election interface
//
const onMessageEditElection = buildNotificationHandler({
  [ElectionEvents.ElectionUpdated]: handleElectionUpdated,
  [ElectionEvents.ElectionDeleted]: handleElectionDeleted,
  [ElectionEvents.RegistrationOpened]: handleRegistrationOpened,
});

const ELECTION_EVENTS: ElectionEvents[] = [
  ElectionEvents.ElectionUpdated,
  ElectionEvents.ElectionDeleted,
  ElectionEvents.RegistrationOpened,
];

/**
 * Hook to handle notifications for the edit election interface
 *
 * @param electionId ID of the election being edited
 */
export const useEditElectionNotifications = (electionId: string) => {
  const subscribeTo: WebsocketSubscriptionList = useMemo(
    () => ({ electionEvents: { [electionId]: ELECTION_EVENTS } }),
    [electionId],
  );

  useNotifications({ subscribeTo, onMessage: onMessageEditElection });
};

function handleElectionUpdated(event: ElectionUpdatedEvent): void {
  mergeState({ updated: true });
}

function handleElectionDeleted(event: ElectionDeletedEvent): void {
  mergeState({
    electionDetails: apiError(buildRequestError(404, 'Election Deleted', GlobalErrorCode.NoSuchResource)),
  });
}

function handleRegistrationOpened(event: RegistrationOpenedEvent): void {
  mergeState({
    electionDetails: apiError(
      buildRequestError(409, 'Cannot edit election after it has left the draft status', GlobalErrorCode.NoSuchResource),
    ),
  });
}

/**
 * Build a JavaScript object to simulate a request error on the frontend
 *
 * This is a hacky solution, as the Axios error object is incomplete, but it doesn't seem to break anything
 *
 * @param statusCode Status code
 * @param description Error description
 * @param errorCode Global error code
 * @returns Error object
 */
function buildRequestError(statusCode: number, description: string, errorCode: GlobalErrorCode): RequestError {
  return {
    isAxiosError: true,
    response: {
      data: { statusCode, description, errorCode } as ErrorResponse,
    },
  } as RequestError;
}
