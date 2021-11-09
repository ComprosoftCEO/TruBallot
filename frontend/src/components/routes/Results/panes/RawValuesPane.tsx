import { Tab, Transition, Table, Divider, Button, Icon } from 'semantic-ui-react';
import Latex from 'react-latex';
import { nestedSelectorHook } from 'redux/helpers';
import { exportResultsJSON, setRawTab, useTabAnimation } from './panesActions';

const useSelector = nestedSelectorHook('results');
const useGlobalsSelector = nestedSelectorHook('globals');

export const RawValuesPane = () => {
  const questionIndex = useSelector((state) => state.currentQuestionIndex);
  const question = useSelector((state) => state.questions[questionIndex]);

  const generator = useSelector((state) => state.generator);
  const prime = useSelector((state) => state.prime);

  const rawTab = useSelector((state) => state.questions[questionIndex].rawTab);
  const currentUserId = useGlobalsSelector((state) => state.userId);
  const tabAnimation = useTabAnimation();

  return (
    <Transition animation={tabAnimation} duration={300} transitionOnMount>
      <Tab.Pane>
        <Tab
          activeIndex={rawTab}
          onTabChange={setRawTab}
          menu={{ secondary: true, pointing: true }}
          panes={[
            {
              menuItem: 'Ballots',
              render: () => (
                <Transition animation="fade down" duration={300} transitionOnMount>
                  <Tab.Pane attached={false} secondary raised>
                    <div style={{ overflowX: 'auto' }}>
                      <Table celled>
                        <Table.Header>
                          <Table.Row>
                            <Table.HeaderCell textAlign="center" singleLine>
                              User
                            </Table.HeaderCell>
                            <Table.HeaderCell textAlign="center" singleLine>
                              <Latex>$p_i$</Latex>
                            </Table.HeaderCell>
                            <Table.HeaderCell textAlign="center" singleLine>
                              <Latex>{String.raw`$p_i^\prime$`}</Latex>
                            </Table.HeaderCell>
                            <Table.HeaderCell textAlign="center" singleLine>
                              <Latex>{String.raw`$g^{s_{ii}}$`}</Latex>
                            </Table.HeaderCell>
                            <Table.HeaderCell textAlign="center" singleLine>
                              <Latex>{String.raw`$g^{s_{ii}^\prime}$`}</Latex>
                            </Table.HeaderCell>
                            <Table.HeaderCell textAlign="center" singleLine>
                              <Latex>{String.raw`$g^{s_{ii} s_{ii}^\prime}$`}</Latex>
                            </Table.HeaderCell>
                          </Table.Row>
                        </Table.Header>

                        <Table.Body>
                          {question.ballots.map((ballot) => (
                            <Table.Row key={ballot.id}>
                              <Table.Cell singleLine>
                                {ballot.id === currentUserId ? (
                                  <>
                                    <Icon name="user" />
                                    <u>Me</u>
                                  </>
                                ) : (
                                  ballot.name
                                )}
                              </Table.Cell>
                              <Table.Cell singleLine>{ballot.forwardBallot}</Table.Cell>
                              <Table.Cell singleLine>{ballot.reverseBallot}</Table.Cell>
                              <Table.Cell singleLine>{ballot.gS}</Table.Cell>
                              <Table.Cell singleLine>{ballot.gSPrime}</Table.Cell>
                              <Table.Cell singleLine>{ballot.gSSPrime}</Table.Cell>
                            </Table.Row>
                          ))}
                        </Table.Body>
                      </Table>
                    </div>
                  </Tab.Pane>
                </Transition>
              ),
            },

            {
              menuItem: 'Parameters',
              render: () => (
                <div>
                  <Transition animation="fade down" duration={300} transitionOnMount>
                    <Tab.Pane attached={false} secondary raised>
                      <div style={{ overflowX: 'auto' }}>
                        <Table celled>
                          <Table.Header>
                            <Table.Row>
                              <Table.HeaderCell textAlign="center" singleLine>
                                Parameter
                              </Table.HeaderCell>
                              <Table.HeaderCell textAlign="center" singleLine>
                                Value
                              </Table.HeaderCell>
                            </Table.Row>
                          </Table.Header>
                          <Table.Body>
                            <Table.Row>
                              <Table.Cell textAlign="center">
                                <Latex>$g$</Latex>
                              </Table.Cell>
                              <Table.Cell>{generator.toString(10)}</Table.Cell>
                            </Table.Row>

                            <Table.Row>
                              <Table.Cell textAlign="center">
                                <Latex>$p$</Latex>
                              </Table.Cell>
                              <Table.Cell>{prime.toString(10)}</Table.Cell>
                            </Table.Row>

                            {question.forwardBallots && (
                              <Table.Row>
                                <Table.Cell textAlign="center">
                                  <Latex
                                    displayMode
                                  >{String.raw`$\sum_{i \in \left\{ \textrm{Voted} \right\}}{p_i}$`}</Latex>
                                  <i>Forward Ballot Sum</i>
                                </Table.Cell>
                                <Table.Cell>{question.forwardBallots}</Table.Cell>
                              </Table.Row>
                            )}

                            {question.reverseBallots && (
                              <Table.Row>
                                <Table.Cell textAlign="center">
                                  <Latex
                                    displayMode
                                  >{String.raw`$\sum_{i \in \left\{ \textrm{Voted} \right\}}{p_i^\prime}$`}</Latex>
                                  <i>Reverse Ballot Sum</i>
                                </Table.Cell>
                                <Table.Cell>{question.reverseBallots}</Table.Cell>
                              </Table.Row>
                            )}

                            {question.forwardCancelationShares !== undefined && (
                              <Table.Row>
                                <Table.Cell textAlign="center" style={{ minWidth: 320 }}>
                                  <Latex displayMode>
                                    {String.raw`$\sum_{i \in \left\{ \textrm{No Vote} \right\}}
                                {\left( -S_{i,C_1} - S_{i,C_2} + \tilde{S}_{i,C_1} + \tilde{S}_{i,C_2} \right)}$`}
                                  </Latex>
                                  <i>Forward Cancelation Shares</i>
                                </Table.Cell>
                                <Table.Cell>{question.forwardCancelationShares}</Table.Cell>
                              </Table.Row>
                            )}

                            {question.reverseCancelationShares !== undefined && (
                              <Table.Row>
                                <Table.Cell textAlign="center" style={{ minWidth: 320 }}>
                                  <Latex displayMode>
                                    {String.raw`$\sum_{i \in \left\{ \textrm{No Vote} \right\}}
                                {\left( -S_{i,C_1}^\prime - S_{i,C_2}^\prime + \tilde{S}_{i,C_1}^\prime + \tilde{S}_{i,C_2}^\prime \right)}$`}
                                  </Latex>
                                  <i>Reverse Cancelation Shares</i>
                                </Table.Cell>
                                <Table.Cell>{question.reverseCancelationShares}</Table.Cell>
                              </Table.Row>
                            )}
                          </Table.Body>
                        </Table>
                      </div>
                    </Tab.Pane>
                  </Transition>
                </div>
              ),
            },
          ]}
        />

        {question.forwardBallots !== undefined && question.reverseBallots !== undefined && (
          <>
            <Divider horizontal />
            <Button primary size="large" icon="download" content="Export Results JSON" onClick={exportResultsJSON} />
          </>
        )}
      </Tab.Pane>
    </Transition>
  );
};
