import { useLayoutEffect } from 'react';
import { clearNestedState, getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { LastLocationType } from 'react-router-last-location';
import { history } from 'index';
import { apiLoading, axiosApi, resolveResult } from 'api';
import { NewElectionResult } from 'models/election';

const mergeState = mergeNestedState('editor');
const useSelector = nestedSelectorHook('editor');
const getState = getNestedState('editor');

export const PLACEHOLDER_TEXT = `Question 1
- Candidate 1
- Candidate 2

Question 2
- Candidate 1
- Candidate 2`;

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('editor'), []);
};

//
// Setters
//
export const setName = (name: string): void => mergeState({ name, modified: true });
export const toggleIsPublic = (): void => mergeState((state) => ({ isPublic: !state.isPublic, modified: true }));
export const setQuestions = (questions: string): void => mergeState({ questions, modified: true });

// The questions string gets parsed into a QuestionList type
export type QuestionList = QuestionEntry[];
export type QuestionEntry = [string, string[]];

/**
 * Parse the question text string into a list of questions and candidates
 *
 * Correct response for PLACEHOLDER_TEXT:
 *   [
 *     ["Question 1", ["Candidate 1", "Candidate 2"],
 *     ["Question 2", ["Candidate 1", "Candidate 2"]
 *   ]
 *
 * @param inputText String to parse
 * @returns Parsed output
 */
export const parseListString = (inputText: string, placeholder = PLACEHOLDER_TEXT): QuestionList => {
  const input = inputText.replace(/\s/g, '') ? inputText : placeholder;
  const lines = input.split(/\r?\n/).map((line) => line.trimLeft().slice(0, 255));

  const items: [string, string[]][] = [];

  // Read items one-by-one and push them into the list
  let currentLine: string | null = null;
  let currentLineQuestions: string[] = [];
  for (const line of lines) {
    // Blank line
    if (line.length === 0) {
      if (currentLine !== null) {
        // Flush the current question
        items.push([currentLine, currentLineQuestions]);
        currentLine = null;
        currentLineQuestions = [];
      }
      continue;
    }

    // Non-blank line
    if (currentLine === null) {
      // First line initializes a new question
      currentLine = line.trim();
    } else {
      // Next lines are candidates, so trim the "- " or "* " from the front of the string
      currentLineQuestions.push(line.replace(/^[-*][ ]?/g, '').trim());
    }
  }

  // Flush at the end if we haven't flushed yet
  if (currentLine !== null) {
    items.push([currentLine, currentLineQuestions]);
  }

  return items;
};

/**
 * Validate the list of questions for any errors
 *
 * @param input The list of questions
 * @returns List of errors, if any
 */
export const validateQuestionList = (input: QuestionList): string[] => {
  const errors = [];
  if (input.length === 0) {
    errors.push('Must have at least one question');
  }

  for (const [i, [question, candidates]] of input.entries()) {
    if (question.length === 0) {
      errors.push(`Question ${i + 1} title cannot be empty`);
    }

    if (candidates.length < 2) {
      errors.push(`Question ${i + 1} needs at least 2 candidates`);
    }

    for (const [j, candidate] of candidates.entries()) {
      if (candidate.length === 0) {
        errors.push(`Question ${i + 1}, candidate ${j + 1} cannot be empty`);
      }
    }
  }

  return errors;
};

/**
 * Validate all of the form inputs
 */
export const useIsFormValid = (): boolean =>
  useSelector((state) => {
    const questions = parseListString(state.questions, '');
    return state.name.length > 0 && questions.length > 0 && validateQuestionList(questions).length === 0;
  });

export const goBack = (lastLocation: LastLocationType): void => {
  if (lastLocation === null) {
    history.push('/');
  } else {
    history.goBack();
  }
};

/**
 * Create the election
 */
export const createElection = async () => {
  const { name, isPublic, questions: questionString } = getState();
  const questions = parseListString(questionString).map(([question, candidates]) => ({ name: question, candidates }));

  mergeState({ submitting: apiLoading() });

  const result = await axiosApi
    .post<NewElectionResult>('/elections', { name, isPublic, questions })
    .then(...resolveResult);

  if (result.success) {
    mergeState({ modified: false });
    history.push(`/elections/${result.data.id}`);
  } else {
    mergeState({ submitting: result });
  }
};
