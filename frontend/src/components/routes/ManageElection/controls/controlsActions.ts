import { APIError, apiLoading, APISuccess, apiSuccess, axiosApi, resolveResult } from 'api';
import { getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { showConfirm } from 'showConfirm';
import { history } from 'index';
import {
  ElectionStatus,
  HasVotedStatus,
  PublicElectionDetails,
  PublicElectionQuestion,
  PublishElectionResult,
} from 'models/election';
import { ManageElectionState } from 'redux/state';
import { PublicCollectorList } from 'models/mediator';

const getState = getNestedState('manageElection');
const useSelector = nestedSelectorHook('manageElection');
const mergeState = mergeNestedState('manageElection');

const getGlobalsState = getNestedState('globals');

/// Need at least 4 users registered before voting can begin
export const MIN_REGISTERED_FOR_VOTING = 4;

/// Need at least 2 votes before closing the election
export const MIN_VOTES_FOR_CLOSING = 2;

/**
 * Test if any of the election requests are loading
 */
export const useIsLoading = (): boolean =>
  useSelector(
    (state) =>
      state.deletingElection.loading ||
      state.publishingElection.loading ||
      state.registering.loading ||
      state.openingVoting.loading ||
      state.closingVoting.loading,
  );

/**
 * Get the current request that errored.
 * This SHOULD only return one UNIQUE error, as we can only make on request at a time
 *
 * @returns Current error
 */
export const useElectionError = (): APIError | undefined =>
  useSelector((state) => {
    if (!state.deletingElection.loading && !state.deletingElection.success) {
      return state.deletingElection;
    }

    if (!state.publishingElection.loading && !state.publishingElection.success) {
      return state.publishingElection;
    }

    if (!state.registering.loading && !state.registering.success) {
      return state.registering;
    }

    if (!state.openingVoting.loading && !state.openingVoting.success) {
      return state.openingVoting;
    }

    if (!state.closingVoting.loading && !state.closingVoting.success) {
      return state.closingVoting;
    }

    return undefined;
  });

/**
 * Clear the response from all API requests
 */
const CLEAR_REQUESTS: Partial<ManageElectionState> = {
  publishingElection: apiSuccess(false),
  deletingElection: apiSuccess(false),
  registering: apiSuccess(false),
  openingVoting: apiSuccess(false),
  closingVoting: apiSuccess(false),
};

export const clearRequests = (): void => {
  mergeState(CLEAR_REQUESTS);
};

/**
 * Generate the partial Redux object to update the nested election
 *
 * @param input Inside properties to update, or function to apply the update
 */
function updateNestedElectionProps(
  input: Partial<PublicElectionDetails> | ((input: PublicElectionDetails) => Partial<PublicElectionDetails>),
): Partial<ManageElectionState> {
  const { electionDetails } = getState();

  // This SHOULD not happen, as we only call this function when the data is successfully loaded
  if (electionDetails.loading || !electionDetails.success) {
    return {};
  }

  const newDetails: PublicElectionDetails = {
    ...electionDetails.data,
    ...(typeof input === 'function' ? input(electionDetails.data) : input),
  };

  return { electionDetails: apiSuccess(newDetails) };
}

/**
 * Delete election action
 */
export const deleteElection = (electionId: string): void => {
  showConfirm({
    header: 'Really delete election draft?',
    message: 'This action cannot be undone',
    onConfirm: async () => {
      mergeState({ ...CLEAR_REQUESTS, deletingElection: apiLoading() });

      const result = await axiosApi.delete(`/elections/${electionId}`).then(...resolveResult);
      if (result.success) {
        mergeState({ deletingElection: apiSuccess(true) });
        showConfirm({
          message: 'Election Deleted!',
          confirmButton: 'Ok',
          cancelButton: null,
          onConfirm: () => history.push('/'),
        });
      } else {
        mergeState({ deletingElection: result });
      }
    },
  });
};

/**
 * Publish an election action
 */
export const publishElection = (electionId: string): void => {
  showConfirm({
    header: 'Are you sure you want to publish election?',
    message: 'This will open the election for voting and prevent any further changes',
    onConfirm: async () => {
      mergeState({ ...CLEAR_REQUESTS, publishingElection: apiLoading() });

      const result = await axiosApi
        .put<PublishElectionResult>(`/elections/${electionId}/registration`)
        .then(...resolveResult);

      if (result.success) {
        mergeState({
          publishingElection: apiSuccess(true),
          ...updateNestedElectionProps({ status: ElectionStatus.Registration, accessCode: result.data.accessCode }),
        });
      } else {
        mergeState({ publishingElection: result });
      }
    },
  });
};

/**
 * Register for an election
 */
export const register = async (electionId: string): Promise<void> => {
  const { name, userId } = getGlobalsState();

  mergeState({ ...CLEAR_REQUESTS, registering: apiLoading() });

  const result = await axiosApi.post(`/elections/${electionId}/registration`).then(...resolveResult);
  if (result.success) {
    mergeState({
      registering: apiSuccess(true),
      ...updateNestedElectionProps((props) => {
        // Possibly insert into the list of registered users if the user isn't already in it
        //   Be sure to sort the list after insertion to keep the order consistent
        const registeredIndex = props.registered.findIndex((r) => r.id === userId);
        const registered =
          registeredIndex > -1
            ? props.registered
            : [...props.registered, { id: userId, name, hasVotedStatus: HasVotedStatus.No }].sort((a, b) =>
                a.name.localeCompare(b.name),
              );

        return {
          isRegistered: true,
          registered,
        };
      }),
    });
  } else {
    mergeState({ registering: result });
  }
};

/**
 * Unregister from an election
 */
export const unregister = async (electionId: string): Promise<void> => {
  const { userId } = getGlobalsState();

  mergeState({ ...CLEAR_REQUESTS, registering: apiLoading() });

  const result = await axiosApi.delete(`/elections/${electionId}/registration`).then(...resolveResult);
  if (result.success) {
    mergeState({
      registering: apiSuccess(true),
      ...updateNestedElectionProps((props) => ({
        isRegistered: false,
        registered: props.registered.filter(({ id }) => id !== userId),
      })),
    });
  } else {
    mergeState({ registering: result });
  }
};

export const openCollectorModal = (): void =>
  mergeState({ pickCollectorsModalOpen: true, collectorsSelected: new Set() });

export const closeCollectorModal = (): void => mergeState({ pickCollectorsModalOpen: false });

export const toggleCollectorSelected = (collectorId: string): void => {
  mergeState((state) => {
    const collectorsSelected = new Set(state.collectorsSelected);
    if (collectorsSelected.has(collectorId)) {
      collectorsSelected.delete(collectorId);
    } else {
      collectorsSelected.add(collectorId);
    }

    return { collectorsSelected };
  });
};

/**
 * Open election voting
 */
export const openVoting = async (electionId: string): Promise<void> => {
  const { collectorsSelected, allCollectors } = getState();
  mergeState({ ...CLEAR_REQUESTS, openingVoting: apiLoading(), pickCollectorsModalOpen: false });

  const result = await axiosApi
    .post(`/elections/${electionId}/voting`, { collectors: [...collectorsSelected] })
    .then(...resolveResult);

  if (result.success) {
    // Build the list of selected election collectors
    const electionCollectors: PublicCollectorList[] = (allCollectors as APISuccess<PublicCollectorList[]>).data.filter(
      ({ id }) => collectorsSelected.has(id),
    );

    mergeState({
      electionCollectors: apiSuccess(electionCollectors),
      openingVoting: apiSuccess(true),
      ...updateNestedElectionProps({
        status: ElectionStatus.Voting,
        accessCode: undefined,
      }),
    });
  } else {
    mergeState({
      openingVoting: result,
      ...updateNestedElectionProps({
        status: ElectionStatus.InitFailed,
        accessCode: undefined,
      }),
    });
  }
};

/**
 * Makes sure each question has at least 2 votes
 */
export const validateNumberOfVotes = (questions: PublicElectionQuestion[]): boolean =>
  questions.every((question) => question.numVotesReceived >= MIN_VOTES_FOR_CLOSING);

/**
 * Close voting action
 */
export const closeVoting = (electionId: string, override?: true) => {
  showConfirm({
    header: 'Are you sure you want to close voting?',
    message: 'This will end the election and compute the final tally',
    override,
    onConfirm: async () => {
      mergeState({ ...CLEAR_REQUESTS, closingVoting: apiLoading() });

      const result = await axiosApi.delete(`/elections/${electionId}/voting`).then(...resolveResult);
      if (result.success) {
        mergeState({
          closingVoting: apiSuccess(true),
          ...updateNestedElectionProps({
            status: ElectionStatus.Finished,
          }),
        });
      } else {
        mergeState({
          closingVoting: result,
          ...updateNestedElectionProps({
            status: ElectionStatus.CollectionFailed,
          }),
        });
      }
    },
  });
};
