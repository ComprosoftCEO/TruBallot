import { ErrorOccured } from 'components/errorDialogs';
import { DashboardMenu, ErrorPortal, Flex, TransitionList } from 'components/shared';
import { goBack } from 'helpers/goBack';
import { useLastLocation } from 'react-router-last-location';
import { nestedSelectorHook } from 'redux/helpers';
import { Button, Container, Divider, Header, Menu, Tab, Transition } from 'semantic-ui-react';
import {
  getFatalError,
  nextQuestion,
  prevQuestion,
  setCurrentTab,
  tryReFetchData,
  useClearState,
  useElectionId,
  useFetchData,
  useFetchError,
  useSetResultsTitle,
} from './resultsActions';
import { CandidatePane } from './panes/CandidatePane';
import { VotingVectorPane } from './panes/VotingVectorPane';
import { BallotsPane } from './panes/BallotsPane';

const useSelector = nestedSelectorHook('results');

export const Results = () => {
  useClearState();

  // Fetch the election to vote for
  const electionId = useElectionId();
  useFetchData(electionId);
  const fetchError = useFetchError();

  // Set the title based on the election
  const electionDetails = useSelector((state) => state.electionDetails);
  useSetResultsTitle(electionDetails);

  const numQuestions = useSelector((state) => state.questions.length);
  const questionIndex = useSelector((state) => state.currentQuestionIndex);
  const currentQuestion = useSelector((state) => state.questions[questionIndex]);
  const currentTab = useSelector((state) => state.currentTab);
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
          <Container textAlign="center" style={{ marginTop: '8em' }}>
            <Header size="large">Loading Election Results...</Header>

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
            onReload={() => tryReFetchData(electionId)}
          />
        )}
      </>
    );
  }

  return (
    <>
      <DashboardMenu />
      <Container textAlign="center" style={{ marginTop: '8em' }}>
        <TransitionList animation="fade down" duration={500} totalDuration={500}>
          <div>
            <Flex justify="space-between" alignItems="center">
              <Button
                icon="left arrow"
                labelPosition="left"
                content="Prev"
                onClick={prevQuestion}
                disabled={questionIndex === 0}
              />

              <Header size="large">{`${questionIndex + 1}. ${currentQuestion.name}`}</Header>

              <Button
                icon="right arrow"
                labelPosition="right"
                content="Next"
                onClick={nextQuestion}
                disabled={questionIndex === numQuestions - 1}
              />
            </Flex>

            <Divider section />
          </div>

          <Tab
            activeIndex={currentTab}
            onTabChange={setCurrentTab}
            panes={[
              {
                menuItem: currentQuestion.ballotValid && (
                  <Menu.Item key="candidates" icon="users" content="Candidates" />
                ),
                render: () => <CandidatePane />,
              },
              {
                menuItem: currentQuestion.forwardBallots !== undefined &&
                  currentQuestion.reverseBallots !== undefined && (
                    <Menu.Item key="vector" icon="list ordered" content="Voting Vector" />
                  ),
                render: () => <VotingVectorPane />,
              },
              {
                menuItem: { key: 'ballots', icon: 'check square outline', content: 'Ballots' },
                render: () => <BallotsPane />,
              },
              {
                menuItem: { key: 'raw', icon: 'table', content: 'Raw Values' },
                render: () => <Tab.Pane>Tab 1 Content</Tab.Pane>,
              },
            ]}
          />

          <div>
            <Button
              icon="arrow left"
              content="Go Back"
              style={{ marginTop: '2em' }}
              onClick={() => goBack(lastLocation)}
            />
          </div>
        </TransitionList>
      </Container>
    </>
  );
};
