import { SemanticTRANSITIONS, TabProps } from 'semantic-ui-react';
import exportFromJSON from 'export-from-json';
import { apiLoading, apiSuccess, axiosApi, resolveResult } from 'api';
import { VerificationResult, VerifyBallotData } from 'models/verification';
import { getNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { ExtendedBallotsResult } from 'redux/state/results';

const getState = getNestedState('results');
const mergeState = mergeNestedState('results');
const useSelector = nestedSelectorHook('results');

/**
 * Get the animation for the tab
 */
export const useTabAnimation = (): SemanticTRANSITIONS =>
  useSelector(({ questions, currentQuestionIndex }) => {
    const { currentTab, prevTab } = questions[currentQuestionIndex];

    // Special case for initialization
    if (currentTab === prevTab) {
      return 'fade down';
    }

    return currentTab < prevTab ? 'fade right' : 'fade left';
  });

/**
 * Set the new tab for the voting vector component
 */
export const setVectorTab = (event: React.MouseEvent<HTMLDivElement>, { activeIndex }: TabProps): void =>
  mergeState((state) => {
    const newQuestions = [...state.questions];
    newQuestions[state.currentQuestionIndex].vectorTab = Number(activeIndex);

    return { questions: newQuestions };
  });

/**
 * Toggle the "Show Vote" option for a given question
 */
export const toggleShowVote = (questionIndex: number): void => {
  const { questions } = getState();

  const newQuestions = [...questions];
  newQuestions[questionIndex].showVote = !newQuestions[questionIndex].showVote;

  mergeState({ questions: newQuestions });
};

/**
 * Handle ballot verification
 */
export const verifyBallot = async (electionId: string, questionIndex: number, ballotIndex: number) => {
  const { questions } = getState();
  const question = questions[questionIndex];
  const ballot = question.ballots[ballotIndex];

  updateBallot(questionIndex, ballotIndex, { verifying: apiLoading() });

  // Get all data needed for verification
  const requestData: VerifyBallotData = {
    userId: ballot.id,
    forwardBallot: ballot.forwardBallot,
    reverseBallot: ballot.reverseBallot,
    gS: ballot.gS,
    gSPrime: ballot.gSPrime,
    gSSPrime: ballot.gSSPrime,
  };

  // We can verify with either collector, but always choose collector 1 for simplicity
  //  The result from the collectors verifies with both collectors either way
  const result = await axiosApi
    .post<VerificationResult>(`/collector/1/elections/${electionId}/questions/${question.id}/verification`, requestData)
    .then(...resolveResult);

  updateBallot(questionIndex, ballotIndex, { verifying: result });
};

/**
 * Update the ballot nested inside the store
 *
 * @param questionIndex Question index in the array
 * @param ballotIndex Ballot index in the array
 * @param value New value or function to generate the new value for the ballot
 */
function updateBallot(
  questionIndex: number,
  ballotIndex: number,
  value: Partial<ExtendedBallotsResult> | ((input: ExtendedBallotsResult) => Partial<ExtendedBallotsResult>),
): void {
  mergeState(({ questions }) => {
    const question = questions[questionIndex];

    const newBallots = [...question.ballots];
    if (typeof value === 'function') {
      newBallots[ballotIndex] = { ...newBallots[ballotIndex], ...value(newBallots[ballotIndex]) };
    } else {
      newBallots[ballotIndex] = { ...newBallots[ballotIndex], ...value };
    }

    const newQuestions = [...questions];
    newQuestions[questionIndex] = { ...newQuestions[questionIndex], ballots: newBallots };

    return { questions: newQuestions };
  });
}

/**
 * Clear the ballot request
 */
export const clearVerifyResult = (questionIndex: number, ballotIndex: number): void =>
  updateBallot(questionIndex, ballotIndex, { verifying: apiSuccess(undefined) });

/**
 * Set the new tab for the raw component
 */
export const setRawTab = (event: React.MouseEvent<HTMLDivElement>, { activeIndex }: TabProps): void =>
  mergeState((state) => {
    const newQuestions = [...state.questions];
    newQuestions[state.currentQuestionIndex].rawTab = Number(activeIndex);

    return { questions: newQuestions };
  });

/**
 * Export the election results as a JSON file
 */
export const exportResultsJSON = (): void => {
  const { prime, generator, questions, currentQuestionIndex } = getState();
  const question = questions[currentQuestionIndex];

  const data = {
    prime: prime.toString(10),
    generator: generator.toString(10),
    forwardBallotSum: question.forwardBallots ?? '0',
    reverseBallotSum: question.reverseBallots ?? '0',
    forwardCancelationShares: question.forwardCancelationShares ?? '0',
    reverseCancelationShares: question.reverseCancelationShares ?? '0',
    ballots: question.ballots.map((ballot) => ({
      userId: ballot.id,
      name: ballot.name,
      forwardBallot: ballot.forwardBallot,
      reverseBallot: ballot.reverseBallot,
      gS: ballot.gS,
      gSPrime: ballot.gSPrime,
      gSSPrime: ballot.gSSPrime,
    })),
  };

  const fileName = 'results';
  const exportType = exportFromJSON.types.json;
  exportFromJSON({ data, fileName, exportType });
};
