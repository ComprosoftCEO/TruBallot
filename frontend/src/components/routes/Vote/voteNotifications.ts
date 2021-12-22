import { useMemo } from 'react';
import { apiSuccess } from 'api';
import { ElectionParameters, ElectionStatus, HasVotedStatus, PublicElectionDetails } from 'models/election';
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
import { getNestedState, mergeNestedState } from 'redux/helpers';
import { getUserId } from 'redux/auth';
import { QuestionDetails, VotingStatus } from 'redux/state';
import { PublicCollectorList } from 'models/mediator';

const mergeState = mergeNestedState('vote');
const getState = getNestedState('vote');

//
// List of all event handlers for the vote page
//
const onMessageVote = buildNotificationHandler({
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
 * Hook to handle notifications for the vote page
 *
 * @param electionId ID of the election
 */
export const useVoteNotifications = (electionId: string) => {
  const subscribeTo: WebsocketSubscriptionList = useMemo(
    () => ({ electionEvents: { [electionId]: ELECTION_EVENTS } }),
    [electionId],
  );
  useNotifications({ subscribeTo, onMessage: onMessageVote });
};

function handleVotingOpened(event: VotingOpenedEvent): void {
  const collectorList: PublicCollectorList[] = event.collectors.map((id, index) => ({
    id,
    // Note: Name is NOT used on this page, so we can fill it with a dummy value
    name: `Collector ${index + 1}`,
  }));

  mergeElectionParams({ prime: event.prime, generator: event.generator, locationModulus: event.locationModulus });
  mergeState({ electionCollectors: apiSuccess(collectorList) });
  mergeElection({ status: ElectionStatus.Voting });
}

function handleVoteReceived(event: VoteReceivedEvent): void {
  const userId = getUserId();
  if (event.userId === userId) {
    // Mark the question as having already voted
    mergeQuestion(event.questionId, { hasVoted: true, choices: new Set() });

    const { questions, votingStatus } = getState();

    // Don't trigger the error message if the vote is actively submitting
    if (votingStatus !== VotingStatus.Init) {
      return;
    }

    // Compute the new status for has voted
    let hasVotedStatus = HasVotedStatus.No;
    if (questions.every((q) => q.hasVoted)) {
      hasVotedStatus = HasVotedStatus.Yes;
    } else if (questions.some((q) => q.hasVoted)) {
      hasVotedStatus = HasVotedStatus.Partial;
    }
    mergeElection({ hasVotedStatus });
  }
}

function handleVotingClosed(event: VotingClosedEvent): void {
  mergeElection({ status: ElectionStatus.CollectionFailed });
}

function handleResultsPublished(event: ResultsPublishedEvent): void {
  mergeElection({ status: ElectionStatus.Finished });
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

/**
 * Helpful utility function to update specific election parameters
 *
 * @param newDetails New data for the election
 */
function mergeElectionParams(
  newDetails: Partial<ElectionParameters> | ((input: ElectionParameters) => Partial<ElectionParameters>),
): void {
  mergeState(({ electionParams }) => {
    if (electionParams.loading || !electionParams.success) return {};

    const newParams: ElectionParameters =
      typeof newDetails === 'function'
        ? { ...electionParams.data, ...newDetails(electionParams.data) }
        : { ...electionParams.data, ...newDetails };

    return { electionParams: apiSuccess(newParams) };
  });
}

/**
 * Helpful utility to update a specific question
 *
 * @param questionId ID of the question to update
 * @param newDetails New question details
 */
function mergeQuestion(
  questionId: string,
  newDetails: Partial<QuestionDetails> | ((input: QuestionDetails) => Partial<QuestionDetails>),
): void {
  mergeState((state) => {
    const questionIndex = state.questions.findIndex((q) => q.id === questionId);
    const questions = questionIndex > -1 ? [...state.questions] : state.questions;
    if (questionIndex > -1) {
      if (typeof newDetails === 'function') {
        questions[questionIndex] = { ...questions[questionIndex], ...newDetails(questions[questionIndex]) };
      } else {
        questions[questionIndex] = { ...questions[questionIndex], ...newDetails };
      }
    }

    return { questions };
  });
}
