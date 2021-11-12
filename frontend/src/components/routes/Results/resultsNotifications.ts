import { useMemo } from 'react';
import { apiSuccess, axiosApi, getErrorInformation, resolveResult } from 'api';
import { ElectionResult, ElectionStatus, PublicElectionDetails } from 'models/election';
import {
  buildNotificationHandler,
  ElectionEvents,
  useNotifications,
  WebsocketSubscriptionList,
  VotingOpenedEvent,
  VoteReceivedEvent,
  VotingClosedEvent,
  ResultsPublishedEvent,
} from 'notifications';
import { mergeNestedState } from 'redux/helpers';
import { isDev } from 'env';
import { ExtendedQuestionResult } from 'redux/state/results';

const mergeState = mergeNestedState('results');

//
// List of all event handlers for the results page
//
const onMessageResults = buildNotificationHandler({
  [ElectionEvents.VotingOpened]: handleVotingOpened,
  [ElectionEvents.VoteReceived]: handleVoteReceived,
  [ElectionEvents.VotingClosed]: handleVotingClosed,
  [ElectionEvents.ResultsPublished]: handleResultsPublished,
});

const ELECTION_EVENTS: ElectionEvents[] = [
  ElectionEvents.VotingOpened,
  ElectionEvents.VoteReceived,
  ElectionEvents.VotingClosed,
  ElectionEvents.ResultsPublished,
];

/**
 * Hook to handle notifications for the results page
 *
 * @param electionId ID of the election
 */
export const useResultsNotifications = (electionId: string) => {
  const subscribeTo: WebsocketSubscriptionList = useMemo(
    () => ({ electionEvents: { [electionId]: ELECTION_EVENTS } }),
    [electionId],
  );

  useNotifications({ subscribeTo, onMessage: onMessageResults });
};

function handleVotingOpened(event: VotingOpenedEvent): void {
  mergeElection({ status: ElectionStatus.Voting });
}

function handleVoteReceived(event: VoteReceivedEvent): void {
  mergeState((state) => {
    // Find the question in the list of questions
    const questionIndex = state.questions.findIndex((q) => q.id === event.questionId);
    const questions: ExtendedQuestionResult[] = questionIndex > -1 ? [...state.questions] : state.questions;
    if (questionIndex > -1) {
      // Append the ballot to the current question ballots
      const ballotIndex = questions[questionIndex].ballots.findIndex((b) => b.id === event.userId);
      const ballots =
        ballotIndex > -1
          ? questions[questionIndex].ballots
          : [
              ...questions[questionIndex].ballots,
              {
                id: event.userId,
                name: event.userName,
                forwardBallot: event.forwardBallot,
                reverseBallot: event.reverseBallot,
                gS: event.gS,
                gSPrime: event.gSPrime,
                gSSPrime: event.gSSPrime,
                verifying: apiSuccess(undefined),
              },
            ].sort((a, b) => a.name.localeCompare(b.name));

      // Update the question
      questions[questionIndex] = {
        ...questions[questionIndex],
        ballots,
        noVotes: questions[questionIndex].noVotes.filter((user) => user.id !== event.userId),
      };
    }

    return { questions };
  });
}

function handleVotingClosed(event: VotingClosedEvent): void {
  mergeElection({ status: ElectionStatus.CollectionFailed });
}

function handleResultsPublished(event: ResultsPublishedEvent): void {
  fetchElectionResults(event.electionId, (results) => {
    mergeState(({ electionDetails, questions }) => {
      // Build out the new questions with results
      const newQuestions: ExtendedQuestionResult[] = questions.map((question) => {
        const questionResults = results.questionResults[question.id];
        return {
          ...question,
          ...questionResults,
          candidates: question.candidates.map((candidate, i) => ({
            name: candidate.name,
            numVotes: questionResults?.candidateVotes?.[i].numVotes,
          })),
          ballots: questionResults.userBallots.map((ballot, i) => ({
            ...question.ballots[i] /* Index should be in the exact same order */,
            ...ballot,
          })),
        };
      });

      if (electionDetails.loading || !electionDetails.success) {
        return { electionResults: apiSuccess(results), questions: newQuestions };
      }

      return {
        electionDetails: apiSuccess({ ...electionDetails.data, status: ElectionStatus.Finished }),
        electionResults: apiSuccess(results),
        questions: newQuestions,
      };
    });
    mergeElection({ status: ElectionStatus.Finished });
  });
}

/**
 * Fetch the election results from the API backend
 *
 * @param electionId ID of the election
 * @param handler Handler after fetching the election results
 */
async function fetchElectionResults(electionId: string, handler: (results: ElectionResult) => void): Promise<void> {
  const result = await axiosApi.get<ElectionResult>(`/elections/${electionId}/results`).then(...resolveResult);
  if (!result.success) {
    if (isDev()) {
      const errorInfo = getErrorInformation(result.error);
      // eslint-disable-next-line no-console
      console.error(`Failed to fetch results for election '${electionId}': ${errorInfo.description}`);
    }
  } else {
    handler(result.data);
  }
}

/**
 * Helpful utility function to update specific election properties
 *
 * @param newDetails New data for the election
 */
function mergeElection(
  newDetails: Partial<PublicElectionDetails> | ((input: PublicElectionDetails) => Partial<PublicElectionDetails>),
): void {
  mergeState(({ electionDetails }) => {
    if (electionDetails.loading || !electionDetails.success) return {};

    const newElection: PublicElectionDetails =
      typeof newDetails === 'function'
        ? { ...electionDetails.data, ...newDetails(electionDetails.data) }
        : { ...electionDetails.data, ...newDetails };

    return { electionDetails: apiSuccess(newElection) };
  });
}
