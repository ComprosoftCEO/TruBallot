import { APIError, apiLoading, apiSuccess, axiosApi, resolveResult } from 'api';
import { getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { showConfirm } from 'showConfirm';
import { history } from 'index';
import { ElectionStatus, PublicElectionDetails } from 'models/election';
import { ManageElectionState } from 'redux/state';

const getState = getNestedState('manageElection');
const useSelector = nestedSelectorHook('manageElection');
const useGlobalsSelector = nestedSelectorHook('globals');
const mergeState = mergeNestedState('manageElection');

/**
 * Test if any of the election requests are loading
 */
export const useIsLoading = (): boolean =>
  useSelector(
    (state) =>
      state.deletingElection.loading ||
      state.publishingElection.loading ||
      state.openingVoting.loading ||
      state.closingVoting.loading,
  );

/// Get the current user ID
export const useUserId = (): string => useGlobalsSelector((state) => state.userId);

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

      const result = await axiosApi.put(`/elections/${electionId}/registration`).then(...resolveResult);
      if (result.success) {
        mergeState({
          publishingElection: apiSuccess(true),
          ...updateNestedElectionProps({ status: ElectionStatus.Registration }),
        });
      } else {
        mergeState({ publishingElection: result });
      }
    },
  });
};
