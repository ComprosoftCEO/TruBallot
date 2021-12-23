import {
  apiLoading,
  APIOption,
  APIResult,
  apiSome,
  apiSuccess,
  axiosApi,
  getErrorInformation,
  GlobalErrorCode,
  resolveResult,
} from 'api';
import { useTitle } from 'helpers/title';
import { ElectionStatus, PublicElectionDetails } from 'models/election';
import { PublicCollectorList } from 'models/mediator';
import { useEffect, useLayoutEffect } from 'react';
import { useParams } from 'react-router-dom';
import { clearNestedState, getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';

const mergeState = mergeNestedState('manageElection');
const getState = getNestedState('manageElection');
const useSelector = nestedSelectorHook('manageElection');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('manageElection'), []);
};

export const useElectionId = (): string => useParams<{ electionId: string }>().electionId;

export const useFetchElection = (electionId: string): void => {
  useEffect(() => {
    tryReFetchElection(electionId);
  }, [electionId]);
};

export async function tryReFetchElection(electionId: string): Promise<void> {
  // Try to fetch the election details if they failed to fetch
  let { electionDetails } = getState();
  if (electionDetails.loading || !electionDetails.success) {
    electionDetails = await axiosApi.get<PublicElectionDetails>(`/elections/${electionId}`).then(...resolveResult);

    mergeState({ electionDetails });
    if (!electionDetails.success) {
      return;
    }
  }

  // Try to fetch the list of all collectors if they failed to fetch
  let { allCollectors } = getState();
  if (allCollectors.loading || !allCollectors.success) {
    allCollectors = await axiosApi.get<PublicCollectorList[]>('/mediator/collectors').then(...resolveResult);

    mergeState({ allCollectors });
    if (!allCollectors.success) {
      return;
    }
  }

  // Fetch the election collectors if needec
  let { electionCollectors } = getState();
  if (electionCollectors.loading || !electionCollectors.success) {
    // Only certain election statuses should fetch the election collectors
    if (
      [ElectionStatus.Voting, ElectionStatus.CollectionFailed, ElectionStatus.Finished].includes(
        electionDetails.data.status,
      )
    ) {
      electionCollectors = await axiosApi
        .get<PublicCollectorList[]>(`/mediator/elections/${electionId}/collectors`)
        .then(...resolveResult);

      mergeState({ electionCollectors });
      if (!electionCollectors.success) {
        // eslint-disable-next-line no-useless-return
        return;
      }
    } else {
      // Initialize to an empty list for now for election collectors
      electionCollectors = apiSuccess([]);
      mergeState({ electionCollectors });
    }
  }
}

/**
 * Handle errors with fetching the initial data
 *
 * @returns [title, content] for any errors
 */
export const useFetchError = (): APIOption<[string, string] | undefined> => {
  const electionDetails = useSelector((state) => state.electionDetails);
  const allCollectors = useSelector((state) => state.allCollectors);
  const electionCollectors = useSelector((state) => state.electionCollectors);

  if (electionDetails.loading) {
    return apiLoading();
  }
  if (!electionDetails.success) {
    return apiSome(['Failed to load election details', getErrorInformation(electionDetails.error).description]);
  }

  if (allCollectors.loading) {
    return apiLoading();
  }
  if (!allCollectors.success) {
    return apiSome([
      'Failed to load the list of available collectors',
      getErrorInformation(allCollectors.error).description,
    ]);
  }

  if (electionCollectors.loading) {
    return apiLoading();
  }
  if (!electionCollectors.success) {
    return apiSome([
      'Failed to load the list of election collectors',
      getErrorInformation(electionCollectors.error).description,
    ]);
  }

  return apiSome(undefined);
};

/**
 * Handle fetch errors
 */
const FATAL_ERROR_CODES: GlobalErrorCode[] = [GlobalErrorCode.NoSuchResource];

export const getFatalError = (input: APIResult<PublicElectionDetails>, userId: string): string | undefined => {
  if (input.loading) {
    return undefined;
  }

  // Test for the various fatal backend errors
  if (!input.success) {
    const errorDetails = getErrorInformation(input.error);
    if (FATAL_ERROR_CODES.includes(errorDetails.globalErrorCode ?? GlobalErrorCode.UnknownError)) {
      return errorDetails.description;
    }
    return undefined;
  }

  // Special case for websockets:
  //  Hide the election if it is private and registration has closed on an unregistered user
  //  who isn't the election creator (Creator can ALWAYS view the election)
  const election = input.data;
  if (
    election.createdBy.id !== userId &&
    !election.isPublic &&
    !election.isRegistered &&
    election.status !== ElectionStatus.Registration
  ) {
    return 'Registration has closed for a private election';
  }

  // No other errors
  return undefined;
};

/**
 * Set the document title based on the name of the election
 */
export const useSetElectionTitle = (electionDetails: APIResult<PublicElectionDetails>): void => {
  const title = electionDetails.loading || !electionDetails.success ? 'View Election' : electionDetails.data.name;
  useTitle(title);
};
