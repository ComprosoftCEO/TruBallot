import { useEffect, useLayoutEffect } from 'react';
import { useParams } from 'react-router-dom';
import {
  APIError,
  apiLoading,
  APIOption,
  APIResult,
  apiSome,
  apiSuccess,
  APISuccess,
  axiosApi,
  getErrorInformation,
  GlobalErrorCode,
  resolveResult,
} from 'api';
import { useTitle } from 'helpers/title';
import {
  CollectorElectionParameters,
  CollectorQuestionParameters,
  ElectionParameters,
  ElectionStatus,
  HasVotedStatus,
  PublicElectionDetails,
} from 'models/election';
import { clearNestedState, getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { QuestionDetails, VoteState, VotingStatus } from 'redux/state';
import { showConfirm } from 'showConfirm';
import { getVotingVector, computeBallot } from 'protocol';
import { PublicCollectorList } from 'models/mediator';

const getState = getNestedState('vote');
const mergeState = mergeNestedState('vote');
const useSelector = nestedSelectorHook('vote');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('vote'), []);
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

    mergeState(setElection(electionDetails));
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

  // We can ONLY get to this point if all collectors returned a success, so the cast is safe
  const allRequests: (CollectorElectionParameters | undefined)[] = Object.values(getState().collectorRequests).map(
    (request) => (request as APISuccess<CollectorElectionParameters>)?.data,
  );

  if (allRequests.every((request) => request !== undefined && request.encryptedLocation !== undefined)) {
    const encryptedLocation: bigint =
      allRequests.reduce((acc, request) => BigInt(request?.encryptedLocation ?? 0) + acc, BigInt(0)) %
      BigInt(electionParams.data.locationModulus);

    mergeState({ encryptedLocation });
  }
}

/**
 * Generate the Redux partial state object to update the election
 *
 * @param electionDetails APIResult for the election
 * @returns Partial state
 */
function setElection(electionDetails: APISuccess<PublicElectionDetails> | APIError): Partial<VoteState> {
  if (!electionDetails.success) {
    return { electionDetails };
  }

  // Build out the questions
  const questions: QuestionDetails[] = electionDetails.data.questions.map((question, i) => ({
    id: question.id,
    name: question.name,
    candidates: question.candidates,
    hasVoted: question.hasVoted,
    choices: new Set(),
    voting: apiSuccess(false),
  }));

  return {
    electionDetails,
    questions,
  };
}

/**
 * Handle errors with fetching the initial data
 *
 * @returns [title, content] for any errors
 */
export const useFetchError = (): APIOption<[string, string] | undefined> => {
  const electionDetails = useSelector((state) => state.electionDetails);
  const electionParams = useSelector((state) => state.electionParams);
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
  GlobalErrorCode.NotRegistered,
  GlobalErrorCode.ElectionNotInitialized,
  GlobalErrorCode.AlreadyVoted,
  GlobalErrorCode.NotOpenForVoting,
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

  if (election.status !== ElectionStatus.Voting) {
    return 'Election not open for voting';
  }

  if (!election.isRegistered) {
    return 'User is not registered for election';
  }

  if (election.hasVotedStatus === HasVotedStatus.Yes) {
    return 'User already voted';
  }

  // No other errors
  return undefined;
};

/**
 * Set the document title based on the name of the election
 */
export const useSetVoteTitle = (electionDetails: APIResult<PublicElectionDetails>): void => {
  const title = electionDetails.loading || !electionDetails.success ? 'Vote' : `Vote: ${electionDetails.data.name}`;
  useTitle(title);
};

/**
 * Hook to get a question given the index
 */
export const useQuestion = (questionIndex: number): QuestionDetails =>
  useSelector((state) => state.questions[questionIndex]);

/**
 * Switch between "cheat" mode and normal mode
 */
export const toggleCheatMode = (): void => {
  const { cheatMode, questions } = getState();

  const newQuestions: QuestionDetails[] = questions.map((question) => ({
    ...question,
    choices: question.choices.size <= 1 ? question.choices : new Set(),
  }));

  mergeState({ cheatMode: !cheatMode, questions: newQuestions });
};

/**
 * Set a single choice for the given question
 *
 * @param questionIndex Question to update
 * @param choice Which candidate to select
 */
export const setChoice = (questionIndex: number, choice: number): void => {
  const { questions } = getState();

  const newQuestions: QuestionDetails[] = questions.map((question, index) =>
    index === questionIndex
      ? {
          ...question,
          choices: new Set([choice]),
        }
      : question,
  );

  mergeState({ questions: newQuestions });
};

/**
 * Toggle a choice on or off for "cheat" mode
 *
 * @param questionIndex Question to update
 * @param choice Which candidate to toggle
 */
export const toggleChoice = (questionIndex: number, choice: number): void => {
  const { questions } = getState();

  const newQuestions: QuestionDetails[] = questions.map((question, index) => {
    if (index !== questionIndex) {
      return question;
    }

    const newSet = new Set(question.choices);
    if (newSet.has(choice)) {
      newSet.delete(choice);
    } else {
      newSet.add(choice);
    }

    return { ...question, choices: newSet };
  });

  mergeState({ questions: newQuestions });
};

/**
 * Test if the given input is valid, depending on the state of "cheat" mode
 */
export const useIsFormValid = (): boolean =>
  useSelector(
    (state) => state.cheatMode || state.questions.every((question) => question.hasVoted || question.choices.size === 1),
  );

/**
 * Start submitting the votes
 */
export const vote = (electionId: string, override?: true): void => {
  showConfirm({
    header: 'Are you sure you want to submit votes?',
    message: 'You will not be able to change your reponses later',
    override,
    onConfirm: async () => {
      const { questions, encryptedLocation, electionParams, electionCollectors } = getState();
      if (
        electionParams.loading ||
        !electionParams.success ||
        electionCollectors.loading ||
        !electionCollectors.success
      ) {
        return; // Should NOT happen
      }

      // Mark all questions as loading
      const newQuestions = questions.map((question) => ({
        ...question,
        voting: question.hasVoted ? apiSuccess(false) : apiLoading(),
      }));

      mergeState({ votingStatus: VotingStatus.Voting, questions: newQuestions });

      // Run all requests in parallel
      const results = await Promise.all(
        [...questions.entries()]
          .filter((question) => !question[1].hasVoted)
          .map(([questionIndex, question]) =>
            voteForQuestion(
              electionId,
              question,
              questionIndex,
              encryptedLocation,
              electionParams.data,
              electionCollectors.data,
            ),
          ),
      );

      // Test if every request was successful
      const allSuccessful = results.every((result) => result.success);
      if (!allSuccessful) {
        mergeState({ votingStatus: VotingStatus.Error });
      } else {
        mergeState({ votingStatus: VotingStatus.Success });
      }
    },
  });
};

/**
 * Cast a vote for a single question
 *
 * @param electionId ID of the election
 * @param question Question details
 * @param questionIndex Question index in the array
 *
 * @returns Result
 */
async function voteForQuestion(
  electionId: string,
  question: QuestionDetails,
  questionIndex: number,
  encryptedLocation: bigint,
  electionParams: ElectionParameters,
  electionCollectors: PublicCollectorList[],
): Promise<APISuccess<{}> | APIError> {
  // Get shares from all collectors concurrently
  const allResults = await Promise.all(
    electionCollectors.map(({ id }) =>
      axiosApi
        .get<CollectorQuestionParameters>(
          `/collector/${id}/elections/${electionId}/questions/${question.id}/parameters`,
        )
        .then(...resolveResult),
    ),
  );

  // Make sure we didn't get any errors
  const collectorError = allResults.find((result) => !result.success);
  if (collectorError !== undefined) {
    mergeQuestion(questionIndex, { voting: collectorError as APIError });
    return collectorError;
  }

  // This cast is safe, now that we have verified all results
  const collectorParams = allResults.map((result) => (result as APISuccess<CollectorQuestionParameters>).data);

  // Get the binary voting vector
  const { forwardVector, reverseVector } = getVotingVector({
    candidates: [...question.choices],
    encryptedLocation,
    electionParams,
    questionIndex,
  });

  // Compute the ballots
  const ballot = computeBallot({
    forwardVector,
    reverseVector,
    electionParams,
    collectorParams,
  });

  // Submit the vote!!!
  const result = await axiosApi
    .post(`/elections/${electionId}/questions/${question.id}/vote`, {
      forwardBallot: ballot.forwardBallot.toString(10),
      reverseBallot: ballot.reverseBallot.toString(10),
      gS: ballot.gS.toString(10),
      gSPrime: ballot.gSPrime.toString(10),
      gSSPrime: ballot.gSSPrime.toString(10),
    })
    .then(...resolveResult);

  if (!result.success) {
    mergeQuestion(questionIndex, { voting: result });
    return result;
  }

  // We are done!
  mergeQuestion(questionIndex, { hasVoted: true, voting: apiSuccess(true), choices: new Set() });
  return apiSuccess({});
}

/**
 * Make a Redux request to update a question
 *
 * @param questionIndex Question to update
 * @param data New data for the question, or a function to dynamically update the question
 */
function mergeQuestion(
  questionIndex: number,
  data: Partial<QuestionDetails> | ((input: QuestionDetails) => Partial<QuestionDetails>),
): void {
  mergeState((state) => {
    const newQuestions = [...state.questions];
    if (typeof data === 'function') {
      newQuestions[questionIndex] = { ...newQuestions[questionIndex], ...data(newQuestions[questionIndex]) };
    } else {
      newQuestions[questionIndex] = { ...newQuestions[questionIndex], ...data };
    }

    return { questions: newQuestions };
  });
}

/**
 * Clear the voting status to make changes to the ballot
 */
export const changeBallot = (): void => mergeState({ votingStatus: VotingStatus.Init });
