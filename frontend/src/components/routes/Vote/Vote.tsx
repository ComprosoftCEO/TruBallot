import { useLastLocation } from 'react-router-last-location';
import { Button, Checkbox, Container, Divider, Header, Transition } from 'semantic-ui-react';
import { APISuccess } from 'api';
import { ErrorOccured } from 'components/errorDialogs';
import { DashboardMenu, ErrorPortal, Flex, TransitionList } from 'components/shared';
import { goBack } from 'helpers/goBack';
import { nestedSelectorHook } from 'redux/helpers';
import { Prompt } from 'react-router-dom';
import { VotingStatus } from 'redux/state';
import { PublicElectionDetails } from 'models/election';
import {
  getFatalError,
  toggleCheatMode,
  tryReFetchElection,
  useClearState,
  useElectionId,
  useFetchElection,
  useFetchError,
  useIsFormValid,
  useSetVoteTitle,
  vote,
} from './voteActions';
import { QuestionBox } from './QuestionBox';
import { VotingModal } from './VotingModal';
import styles from './vote.module.scss';

const useSelector = nestedSelectorHook('vote');

export const Vote = () => {
  useClearState();

  // Fetch the election to vote for
  const electionId = useElectionId();
  useFetchElection(electionId);
  const fetchError = useFetchError();

  // Set the title based on the election
  const electionDetails = useSelector((state) => state.electionDetails);
  useSetVoteTitle(electionDetails);

  const cheatMode = useSelector((state) => state.cheatMode);
  const votingStatus = useSelector((state) => state.votingStatus);
  const formValid = useIsFormValid();

  const lastLocation = useLastLocation();

  // Test for a fatal error when or after loading the resource
  const fatalError = getFatalError(electionDetails);
  if (fatalError !== undefined) {
    return <ErrorOccured header="Error: Cannot vote in election!" message={fatalError} />;
  }

  // Show the blank loading form
  if (fetchError.loading || fetchError.data !== undefined) {
    return (
      <>
        <DashboardMenu />
        <Transition animation="scale" duration={300} transitionOnMount>
          <Container textAlign="center" text style={{ marginTop: '8em' }}>
            <Header size="large">Loading Election...</Header>

            <Divider />

            <Button
              icon="arrow left"
              content="Go Back"
              style={{ marginTop: '2em' }}
              onClick={() => goBack(lastLocation)}
            />
          </Container>
        </Transition>

        {!fetchError.loading && fetchError.data !== undefined && (
          <ErrorPortal
            negative
            header={fetchError.data[0]}
            content={fetchError.data[1]}
            onReload={() => tryReFetchElection(electionId)}
          />
        )}
      </>
    );
  }

  // Draw the voting form
  const election = (electionDetails as APISuccess<PublicElectionDetails>).data;
  return (
    <>
      <DashboardMenu />
      <Container textAlign="center" text style={{ marginTop: '8em' }}>
        <TransitionList animation="fade down" duration={500} totalDuration={800}>
          <Header size="large">{election.name}</Header>
          <Divider />

          {election.questions.map((question, index) => (
            <div key={question.id} className={styles['question-container']}>
              <QuestionBox questionIndex={index} cheatMode={cheatMode} disabled={votingStatus !== VotingStatus.Init} />
            </div>
          ))}

          <div className={styles['bottom-container']}>
            <Flex direction="column" justify="space-between" alignItems="center" style={{ minHeight: 80 }}>
              <Button
                primary
                size="large"
                icon="check square outline"
                content="Submit Vote"
                onClick={vote}
                disabled={!formValid || votingStatus !== VotingStatus.Init}
                loading={votingStatus === VotingStatus.Voting}
              />

              <Checkbox
                toggle
                label="Cheat Mode"
                checked={cheatMode}
                onChange={toggleCheatMode}
                disabled={votingStatus !== VotingStatus.Init}
              />
            </Flex>
          </div>
        </TransitionList>

        {votingStatus === VotingStatus.Voting ? (
          <Prompt message="Cancel voting?" />
        ) : (
          <Prompt message="Discard changes to voting form?" />
        )}

        {votingStatus !== VotingStatus.Init && <VotingModal />}
      </Container>
    </>
  );
};
