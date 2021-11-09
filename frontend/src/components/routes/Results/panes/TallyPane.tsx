/* eslint-disable react/no-array-index-key */
import pluralize from 'pluralize';
import { Header, Card, Tab, Label, Popup, Transition } from 'semantic-ui-react';
import { nestedSelectorHook } from 'redux/helpers';
import { useTabAnimation } from './panesActions';

const useSelector = nestedSelectorHook('results');

export const TallyPane = () => {
  const currentIndex = useSelector((state) => state.currentQuestionIndex);
  const question = useSelector((state) => state.questions[currentIndex]);
  const tabAnimation = useTabAnimation();

  const winningVotes = question.candidates.reduce((max, candidate) => Math.max(candidate.numVotes ?? 0, max), 0);
  const isTied = question.candidates.filter((candidate) => candidate.numVotes === winningVotes).length > 1;

  return (
    <Transition animation={tabAnimation} duration={300} transitionOnMount>
      <Tab.Pane>
        <Header>Final Tally:</Header>
        <Card.Group stackable itemsPerRow="3" centered={question.candidates.length < 3}>
          {question.candidates.map((candidate, i) => (
            <Card key={`${i}-${candidate}`} raised={candidate.numVotes === winningVotes}>
              <Card.Content>
                <Card.Header content={candidate.name} />
                <Card.Description content={pluralize('Vote', candidate.numVotes ?? 0, true)} />

                {candidate.numVotes === winningVotes &&
                  (isTied ? (
                    <Popup
                      on="hover"
                      size="mini"
                      content={<Label color="yellow" content="Tied" />}
                      position="right center"
                      trigger={<Label corner="right" color="yellow" icon="trophy" />}
                    />
                  ) : (
                    <Popup
                      on="hover"
                      size="mini"
                      content={<Label color="green" content="Winner" />}
                      position="right center"
                      trigger={<Label corner="right" color="green" icon="trophy" />}
                    />
                  ))}
              </Card.Content>
            </Card>
          ))}
        </Card.Group>
      </Tab.Pane>
    </Transition>
  );
};
