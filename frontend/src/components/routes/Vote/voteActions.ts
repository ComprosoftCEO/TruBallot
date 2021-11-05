import { useEffect, useLayoutEffect } from 'react';
import { useParams } from 'react-router-dom';
import {
  APIError,
  APIResult,
  apiSuccess,
  APISuccess,
  axiosApi,
  getErrorInformation,
  GlobalErrorCode,
  resolveResult,
} from 'api';
import { useTitle } from 'helpers/title';
import { ElectionStatus, HasVotedStatus, PublicElectionDetails } from 'models/election';
import { clearNestedState, getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { QuestionDetails, VoteState } from 'redux/state';

const getState = getNestedState('vote');
const mergeState = mergeNestedState('vote');
const useSelector = nestedSelectorHook('vote');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('vote'), []);
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
      .then(setElection),
  );
}

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

  if (election.hasVoted === HasVotedStatus.Yes) {
    return 'User already voted';
  }

  // No other errors
  return undefined;
};

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
  const questions: QuestionDetails[] = electionDetails.data.questions.map((question) => ({
    id: question.id,
    name: question.name,
    candidates: question.candidates,
    choices: new Set(),
    voting: apiSuccess(false),
  }));

  return {
    electionDetails,
    questions,
  };
}

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
  useSelector((state) => state.cheatMode || state.questions.every((question) => question.choices.size === 1));
