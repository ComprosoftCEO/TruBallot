import { nestedSelectorHook } from 'redux/helpers';
import { Header, Card, Tab, Transition, Grid, List } from 'semantic-ui-react';
import { useElectionId } from '../resultsActions';
import { useTabAnimation } from './panesActions';
import { VerifyBallotCard } from './VerifyBallotCard';

const useSelector = nestedSelectorHook('results');

export const BallotsPane = () => {
  const questionIndex = useSelector((state) => state.currentQuestionIndex);
  const question = useSelector((state) => state.questions[questionIndex]);

  const electionId = useElectionId();
  const tabAnimation = useTabAnimation();

  return (
    <Transition animation={tabAnimation} duration={300} transitionOnMount>
      <Tab.Pane>
        <Grid columns={2} stackable textAlign="center" divided>
          <Grid.Row>
            <Grid.Column width="12">
              <Header>Ballots:</Header>
              <Card.Group stackable itemsPerRow="2" centered={question.userBallots.length < 2}>
                {question.userBallots.map((ballot, index) => (
                  <VerifyBallotCard
                    key={ballot.id}
                    questionIndex={questionIndex}
                    ballot={ballot}
                    ballotIndex={index}
                    electionId={electionId}
                  />
                ))}
              </Card.Group>
            </Grid.Column>

            <Grid.Column width="4" textAlign="left">
              <Header textAlign="center">Didn&apos;t Vote:</Header>
              <List bulleted>
                {question.noVotes.map((user) => (
                  <List.Item key={user.id}>{user.name}</List.Item>
                ))}
              </List>
            </Grid.Column>
          </Grid.Row>
        </Grid>
      </Tab.Pane>
    </Transition>
  );
};