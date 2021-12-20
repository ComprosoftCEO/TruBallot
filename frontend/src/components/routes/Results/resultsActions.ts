import { useEffect, useLayoutEffect } from 'react';
import { useParams } from 'react-router-dom';
import { TabProps } from 'semantic-ui-react';
import {
  apiLoading,
  APIOption,
  APIResult,
  apiSome,
  APISuccess,
  apiSuccess,
  axiosApi,
  getErrorInformation,
  GlobalErrorCode,
  resolveResult,
} from 'api';
import { useTitle } from 'helpers/title';
import {
  CollectorElectionParameters,
  ElectionParameters,
  ElectionResult,
  ElectionStatus,
  PublicElectionDetails,
} from 'models/election';
import { clearNestedState, getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { ExtendedQuestionResult } from 'redux/state/results';
import { PublicCollectorList } from 'models/mediator';

const getState = getNestedState('results');
const mergeState = mergeNestedState('results');
const useSelector = nestedSelectorHook('results');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('results'), []);
};

export const useElectionId = (): string => useParams<{ electionId: string }>().electionId;

export const useFetchData = (electionId: string): void => {
  useEffect(() => {
    tryReFetchData(electionId);
  }, [electionId]);
};

export async function tryReFetchData(electionId: string): Promise<void> {
  // Try to fetch the election details if they failed to fetch
  let { electionDetails } = getState();
  if (electionDetails.loading || !electionDetails.success) {
    electionDetails = await axiosApi.get<PublicElectionDetails>(`/elections/${electionId}`).then(...resolveResult);

    mergeState({ electionDetails });
    if (!electionDetails.success) {
      return;
    }
  }

  // Try to fetch the election parameters if they failed to fetch
  let { electionParams } = getState();
  if (electionParams.loading || !electionParams.success) {
    electionParams = await axiosApi
      .get<ElectionParameters>(`/elections/${electionId}/parameters`)
      .then(...resolveResult);

    mergeState({ electionParams });
    if (!electionParams.success) {
      return;
    }
  }

  // Try to fetch the election results if they failed to fetch
  let { electionResults } = getState();
  if (electionResults.loading || !electionResults.success) {
    electionResults = await axiosApi.get<ElectionResult>(`/elections/${electionId}/results`).then(...resolveResult);

    mergeState({ electionResults });
    if (!electionResults.success) {
      return;
    }
  }

  // Build out the list of questions
  const allQuestionResults = electionResults.data.questionResults;
  const tab = electionDetails.data.status !== ElectionStatus.Finished ? 2 : 0;
  const questions: ExtendedQuestionResult[] = electionDetails.data.questions.map((question) => {
    const questionResults = allQuestionResults[question.id];

    return {
      ...questionResults,
      id: question.id,
      name: question.name,
      candidates: question.candidates.map((candidate, i) => ({
        name: candidate,
        numVotes: questionResults?.candidateVotes?.[i].numVotes,
      })),
      ballots: questionResults.userBallots.map((ballot) => ({ ...ballot, verifying: apiSuccess(undefined) })),
      showVote: false,
      currentTab: tab,
      prevTab: tab,
      vectorTab: 0,
      rawTab: 0,
    };
  });
  mergeState({ questions, generator: BigInt(electionParams.data.generator), prime: BigInt(electionParams.data.prime) });

  // Try to fetch the list of election collectors if they failed to fetch
  let { electionCollectors } = getState();
  if (electionCollectors.loading || !electionCollectors.success) {
    electionCollectors = await axiosApi
      .get<PublicCollectorList[]>(`/mediator/elections/${electionId}/collectors`)
      .then(...resolveResult);

    mergeState({ electionCollectors });
    if (!electionCollectors.success) {
      return;
    }
  }

  // Try to fetch the encrypted locations from ALL collectors in the elections
  //   Run all of these requests in parallel
  const { collectorRequests } = getState();
  const results = await Promise.all(
    electionCollectors.data
      .filter(({ id }) => {
        // Only re-request ones that failed
        const request = collectorRequests[id];
        return request === undefined || request.loading || !request.success;
      })
      .map(async ({ id }) => {
        const collectorResponse = await axiosApi
          .get<CollectorElectionParameters>(`/collector/${id}/elections/${electionId}/parameters`)
          .then(...resolveResult);

        mergeState((state) => ({ collectorRequests: { ...state.collectorRequests, [id]: collectorResponse } }));
        return collectorResponse;
      }),
  );

  // Stop if any results had an error
  if (results.some((result) => !result.success)) {
    return;
  }

  // Compute the encrypted location
  //   Since the user MUST be registered for an election, collectors should NEVER return "undefined"
  //   But we still add a check to make TypeScript happy :)
  const allRequests: (CollectorElectionParameters | undefined)[] = Object.values(getState().collectorRequests).map(
    (request) => (request as APISuccess<CollectorElectionParameters>)?.data,
  );

  if (allRequests.map((request) => request !== undefined && request.encryptedLocation !== undefined)) {
    const modulo = BigInt(electionParams.data.prime) - BigInt(1);
    const encryptedLocation: bigint =
      allRequests.reduce((acc, request) => BigInt(request?.encryptedLocation ?? 0) + acc, BigInt(0)) % modulo;

    mergeState({ encryptedLocation });
  }
}

/**
 * Handle errors with fetching the initial data
 *
 * @returns [title, content] for any errors
 */
export const useFetchError = (): APIOption<[string, string] | undefined> => {
  const electionDetails = useSelector((state) => state.electionDetails);
  const electionParams = useSelector((state) => state.electionParams);
  const electionResults = useSelector((state) => state.electionResults);
  const electionCollectors = useSelector((state) => state.electionCollectors);
  const collectorRequests = useSelector((state) => state.collectorRequests);

  if (electionDetails.loading) {
    return apiLoading();
  }
  if (!electionDetails.success) {
    return apiSome(['Failed to load election details', getErrorInformation(electionDetails.error).description]);
  }

  if (electionParams.loading) {
    return apiLoading();
  }
  if (!electionParams.success) {
    return apiSome(['Failed to load election parameters', getErrorInformation(electionParams.error).description]);
  }

  if (electionResults.loading) {
    return apiLoading();
  }
  if (!electionResults.success) {
    return apiSome(['Failed to load election results', getErrorInformation(electionResults.error).description]);
  }

  if (electionCollectors.loading) {
    return apiLoading();
  }
  if (!electionCollectors.success) {
    return apiSome(['Failed to load election collectors', getErrorInformation(electionCollectors.error).description]);
  }

  // Check all of the collector parameters
  if (Object.values(collectorRequests).some((request) => request.loading)) {
    return apiLoading();
  }
  for (const [index, [collectorId, request]] of Object.entries(collectorRequests).entries()) {
    if (request.loading) {
      return apiLoading();
    }
    if (!request.success) {
      const name = electionCollectors.data.find((c) => c.id === collectorId)?.name;
      return apiSome([
        `Failed to load parameters from ${name !== undefined ? `collector '${name}'` : `Collector ${index + 1}`}`,
        getErrorInformation(request.error).description,
      ]);
    }
  }

  return apiSome(undefined);
};

/**
 * Handle fetch errors
 */
const FATAL_ERROR_CODES: GlobalErrorCode[] = [
  GlobalErrorCode.NoSuchResource,
  GlobalErrorCode.ElectionNotInitialized,
  GlobalErrorCode.NotOpenForVoting,
];

const VOTING_STARTED_STATUS: ElectionStatus[] = [
  ElectionStatus.Voting,
  ElectionStatus.CollectionFailed,
  ElectionStatus.Finished,
];

export const getFatalError = (input: APIResult<PublicElectionDetails>): string | undefined => {
  if (input.loading) {
    return undefined;
  }

  // Test for the various fatal backend error codes
  if (!input.success) {
    const errorDetails = getErrorInformation(input.error);
    if (FATAL_ERROR_CODES.includes(errorDetails.globalErrorCode ?? GlobalErrorCode.UnknownError)) {
      return errorDetails.description;
    }
    return undefined;
  }

  // Run through the errors that keep us from voting
  //  These all come from the backend logic
  const election = input.data;
  if (election.status === ElectionStatus.InitFailed) {
    return 'Election parameters have not been initialized';
  }

  if (!VOTING_STARTED_STATUS.includes(election.status)) {
    return 'Voting has not started';
  }

  // No other errors
  return undefined;
};

/**
 * Set the document title based on the name of the election
 */
export const useSetResultsTitle = (electionDetails: APIResult<PublicElectionDetails>): void => {
  const title =
    electionDetails.loading || !electionDetails.success ? 'Results' : `Results: ${electionDetails.data.name}`;
  useTitle(title);
};

/**
 * Set the new tab for the main component
 */
export const setCurrentTab = (event: React.MouseEvent<HTMLDivElement>, { activeIndex }: TabProps): void =>
  mergeState((state) => {
    const newQuestions = [...state.questions];
    newQuestions[state.currentQuestionIndex].prevTab = newQuestions[state.currentQuestionIndex].currentTab;
    newQuestions[state.currentQuestionIndex].currentTab = Number(activeIndex);

    return { questions: newQuestions };
  });

/// Move to the next question in the results
export const nextQuestion = (): void =>
  mergeState((state) => ({ currentQuestionIndex: state.currentQuestionIndex + 1 }));

/// Move to the previous question in the results
export const prevQuestion = (): void =>
  mergeState((state) => ({ currentQuestionIndex: state.currentQuestionIndex - 1 }));
