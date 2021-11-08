/* eslint-disable react/no-array-index-key */
import pluralize from 'pluralize';
import { nestedSelectorHook } from 'redux/helpers';
import { Header, Card, Tab, Label, Popup, Transition } from 'semantic-ui-react';
import { useTabAnimation } from './panesActions';

const useSelector = nestedSelectorHook('results');

export const CandidatePane = () => {
  const currentIndex = useSelector((state) => state.currentQuestionIndex);
  const question = useSelector((state) => state.questions[currentIndex]);
  const tabAnimation = useTabAnimation();

  const winningVotes = question.candidates.reduce((max, candidate) => Math.max(candidate.numVotes ?? 0, max), 0);

  return (
    <Transition animation={tabAnimation} duration={300} transitionOnMount>
      <Tab.Pane>
        <Header>Candidates:</Header>
        <Card.Group stackable itemsPerRow="3" centered={question.candidates.length < 3}>
          {question.candidates.map((candidate, i) => (
            <Card key={`${i}-${candidate}`} raised={candidate.numVotes === winningVotes}>
              <Card.Content>
                <Card.Header content={candidate.name} />
                <Card.Meta content={pluralize('Vote', candidate.numVotes ?? 0, true)} />

                {candidate.numVotes === winningVotes && (
                  <Popup
                    on="hover"
                    size="mini"
                    content={<Label color="green" content="Winner" />}
                    position="right center"
                    trigger={<Label corner="right" color="green" icon="trophy" />}
                  />
                )}
              </Card.Content>
            </Card>
          ))}
        </Card.Group>
      </Tab.Pane>
    </Transition>
  );
};
