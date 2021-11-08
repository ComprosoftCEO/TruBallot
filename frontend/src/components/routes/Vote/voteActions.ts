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

  // Try to fetch encrypted location from collector 1 if it failed to fetch
  let { c1Params } = getState();
  if (c1Params.loading || !c1Params.success) {
    c1Params = await axiosApi
      .get<CollectorElectionParameters>(`/collector/1/elections/${electionId}/parameters`)
      .then(...resolveResult);

    mergeState({ c1Params });
    if (!c1Params.success) {
      return;
    }
  }

  let { c2Params } = getState();
  if (c2Params.loading || !c2Params.success) {
    c2Params = await axiosApi
      .get<CollectorElectionParameters>(`/collector/2/elections/${electionId}/parameters`)
      .then(...resolveResult);

    mergeState({ c2Params });
    if (!c2Params.success) {
      return;
    }
  }

  // Compute the encrypted location
  //   Since the user MUST be registered for an election, collectors should NEVER return "undefined"
  //   But we still add a check to make TypeScript happy :)
  if (c1Params.data.encryptedLocation !== undefined && c2Params.data.encryptedLocation !== undefined) {
    const encryptedLocation =
      (BigInt(c1Params.data.encryptedLocation) + BigInt(c2Params.data.encryptedLocation)) %
      (BigInt(electionParams.data.prime) - BigInt(1));

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
  const c1Params = useSelector((state) => state.c1Params);
  const c2Params = useSelector((state) => state.c2Params);

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

  if (c1Params.loading) {
    return apiLoading();
  }
  if (!c1Params.success) {
    return apiSome(['Failed to load parameters from collector 1', getErrorInformation(c1Params.error).description]);
  }

  if (c2Params.loading) {
    return apiLoading();
  }
  if (!c2Params.success) {
    return apiSome(['Failed to load parameters from collector 2', getErrorInformation(c2Params.error).description]);
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
      const { questions, encryptedLocation, electionParams } = getState();
      if (electionParams.loading || !electionParams.success) return; // Should NOT happen

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
            voteForQuestion(electionId, question, questionIndex, encryptedLocation, electionParams.data),
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
): Promise<APISuccess<{}> | APIError> {
  // Get shares from collector 1
  const c1Result = await axiosApi
    .get<CollectorQuestionParameters>(`/collector/1/elections/${electionId}/questions/${question.id}/parameters`)
    .then(...resolveResult);

  if (!c1Result.success) {
    mergeQuestion(questionIndex, { voting: c1Result });
    return c1Result;
  }

  // Get shares from collector 2
  const c2Result = await axiosApi
    .get<CollectorQuestionParameters>(`/collector/2/elections/${electionId}/questions/${question.id}/parameters`)
    .then(...resolveResult);

  if (!c2Result.success) {
    mergeQuestion(questionIndex, { voting: c2Result });
    return c2Result;
  }

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
    c1QuestionParams: c1Result.data,
    c2QuestionParams: c2Result.data,
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