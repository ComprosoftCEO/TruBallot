/* eslint-disable react/no-array-index-key */
import { parseVotingVector } from 'protocol';
import { nestedSelectorHook } from 'redux/helpers';
import { Header, Card, Tab, Divider, Button, Transition } from 'semantic-ui-react';
import { setVectorTab, toggleShowVote, useTabAnimation } from './panesActions';

const useSelector = nestedSelectorHook('results');

const VOTE_STYLE = { outline: '5px solid red' };

export const VotingVectorPane = () => {
  const currentIndex = useSelector((state) => state.currentQuestionIndex);
  const question = useSelector((state) => state.questions[currentIndex]);
  const numRegistered = useSelector((state) =>
    state.electionParams.loading || !state.electionParams.success ? 0 : state.electionParams.data.numRegistered,
  );

  const showVote = useSelector((state) => state.questions[currentIndex].showVote);
  const encryptedLocation = useSelector((state) => state.encryptedLocation);

  const parsedForwardVector = parseVotingVector(
    BigInt(question.forwardBallots!),
    question.candidates.length,
    numRegistered,
  );

  const parsedReverseVector = parseVotingVector(
    BigInt(question.reverseBallots!),
    question.candidates.length,
    numRegistered,
    true,
  );

  const tabAnimation = useTabAnimation();
  const vectorTab = useSelector((state) => state.questions[currentIndex].vectorTab);

  return (
    <Transition animation={tabAnimation} duration={300} transitionOnMount>
      <Tab.Pane>
        <Tab
          activeIndex={vectorTab}
          onTabChange={setVectorTab}
          menu={{ secondary: true, pointing: true }}
          panes={[
            {
              menuItem: 'Forward Vector',
              render: () => (
                <Tab.Pane attached={false} secondary raised>
                  <Header>Forward Voting Vector:</Header>
                  <Divider />
                  <Card.Group stackable itemsPerRow="4">
                    {parsedForwardVector.map(({ candidatePicked, bits }, i) => (
                      <Card
                        key={i}
                        style={
                          showVote && encryptedLocation === BigInt(numRegistered - (i + 1)) ? VOTE_STYLE : undefined
                        }
                      >
                        <Card.Content>
                          <Card.Header
                            content={
                              typeof candidatePicked === 'number'
                                ? question.candidates[candidatePicked].name
                                : `{${candidatePicked === null ? 'Empty' : 'Invalid'}}`
                            }
                          />
                        </Card.Content>
                        <Card.Content extra>{bits}</Card.Content>
                      </Card>
                    ))}
                  </Card.Group>
                </Tab.Pane>
              ),
            },

            {
              menuItem: 'Reverse Vector',
              render: () => (
                <Tab.Pane attached={false} secondary raised>
                  <Header>Reverse Voting Vector:</Header>
                  <Divider />
                  <Card.Group stackable itemsPerRow="4">
                    {parsedReverseVector.map(({ candidatePicked, bits }, i) => (
                      <Card key={i} style={showVote && encryptedLocation === BigInt(i) ? VOTE_STYLE : undefined}>
                        <Card.Content>
                          <Card.Header
                            content={
                              typeof candidatePicked === 'number'
                                ? question.candidates[candidatePicked].name
                                : `{${candidatePicked === null ? 'Empty' : 'Invalid'}}`
                            }
                          />
                        </Card.Content>
                        <Card.Content extra>{bits}</Card.Content>
                      </Card>
                    ))}
                  </Card.Group>
                </Tab.Pane>
              ),
            },
          ]}
        />

        {encryptedLocation !== null && (
          <>
            <Divider horizontal />
            <Button
              primary
              size="large"
              icon={showVote ? 'eye slash' : 'eye'}
              content={showVote ? 'Hide My Vote' : 'Show My Vote'}
              onClick={() => toggleShowVote(currentIndex)}
            />
          </>
        )}
      </Tab.Pane>
    </Transition>
  );
};
