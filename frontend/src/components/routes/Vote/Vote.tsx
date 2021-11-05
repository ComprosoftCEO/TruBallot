import { useLastLocation } from 'react-router-last-location';
import { Button, Checkbox, Container, Divider, Header, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { ErrorOccured } from 'components/errorDialogs';
import { DashboardMenu, ErrorPortal, Flex, TransitionList } from 'components/shared';
import { goBack } from 'helpers/goBack';
import { nestedSelectorHook } from 'redux/helpers';
import {
  getFatalError,
  toggleCheatMode,
  tryReFetchElection,
  useClearState,
  useElectionId,
  useFetchElection,
  useIsFormValid,
  useSetVoteTitle,
} from './voteActions';
import { QuestionBox } from './QuestionBox';
import styles from './vote.module.scss';

const useSelector = nestedSelectorHook('vote');

export const Vote = () => {
  useClearState();

  // Fetch the election to vote for
  const electionId = useElectionId();
  useFetchElection(electionId);

  // Set the title based on the election
  const electionDetails = useSelector((state) => state.electionDetails);
  useSetVoteTitle(electionDetails);

  const cheatMode = useSelector((state) => state.cheatMode);
  const formValid = useIsFormValid();

  const lastLocation = useLastLocation();

  // Test for a fatal error when or after loading the resource
  const fatalError = getFatalError(electionDetails);
  if (fatalError !== undefined) {
    return <ErrorOccured header="Error: Cannot vote in election!" message={fatalError} />;
  }

  // Show the blank loading form
  if (electionDetails.loading || !electionDetails.success) {
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

        {!electionDetails.loading && !electionDetails.success && (
          <ErrorPortal
            negative
            header="Failed to load election"
            content={getErrorInformation(electionDetails.error).description}
            onReload={() => tryReFetchElection(electionId)}
          />
        )}
      </>
    );
  }

  // Draw the voting form
  const election = electionDetails.data;
  return (
    <>
      <DashboardMenu />
      <Container textAlign="center" text style={{ marginTop: '8em' }}>
        <TransitionList animation="fade down" duration={500} totalDuration={800}>
          <Header size="large">{election.name}</Header>
          <Divider />

          {election.questions.map((question, index) => (
            <div key={question.id} className={styles['question-container']}>
              <QuestionBox questionIndex={index} cheatMode={cheatMode} />
            </div>
          ))}

          <div className={styles['bottom-container']}>
            <Flex direction="column" justify="space-between" alignItems="center" style={{ minHeight: 80 }}>
              <Button primary size="large" icon="check square outline" content="Submit Vote" disabled={!formValid} />
              <Checkbox toggle label="Cheat Mode" checked={cheatMode} onChange={toggleCheatMode} />
            </Flex>
          </div>
        </TransitionList>
      </Container>
    </>
  );
};
