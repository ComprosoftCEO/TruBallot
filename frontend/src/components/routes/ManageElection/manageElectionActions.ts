import { APIResult, axiosApi, getErrorInformation, GlobalErrorCode, resolveResult } from 'api';
import { useTitle } from 'helpers/title';
import { PublicElectionDetails } from 'models/election';
import { useEffect, useLayoutEffect } from 'react';
import { useParams } from 'react-router-dom';
import { clearNestedState, mergeNestedState } from 'redux/helpers';

const mergeState = mergeNestedState('manageElection');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('manageElection'), []);
};

export const useElectionId = (): string => useParams<{ electionId: string }>().electionId;

export const useFetchElection = (electionId: string): void => {
  useEffect(() => tryReFetchElection(electionId), [electionId]);
};

export function tryReFetchElection(electionId: string): void {
  mergeState(
    axiosApi
      .get<PublicElectionDetails>(`/elections/${electionId}`)
      .then(...resolveResult)
      .then((electionDetails) => ({ electionDetails })),
  );
}

/**
 * Handle fetch errors
 */
const FATAL_ERROR_CODES: GlobalErrorCode[] = [GlobalErrorCode.NoSuchResource];

export const getFatalError = (input: APIResult<PublicElectionDetails>): string | undefined => {
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
