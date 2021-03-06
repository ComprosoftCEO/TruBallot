import { useLastLocation } from 'react-router-last-location';
import { APISuccess } from 'api';
import { ErrorOccured } from 'components/errorDialogs';
import { DashboardMenu, ElectionStatusLabel, ErrorPortal, Flex, TransitionList } from 'components/shared';
import { goBack } from 'helpers/goBack';
import { nestedSelectorHook } from 'redux/helpers';
import { useUserId } from 'redux/auth';
import {
  Button,
  Container,
  Divider,
  Grid,
  Header,
  Label,
  List,
  Menu,
  Segment,
  Tab,
  Transition,
} from 'semantic-ui-react';
import { ElectionStatus, HasVotedStatus, PublicElectionDetails } from 'models/election';
import { PublicCollectorList } from 'models/mediator';
import { PublicElectionLabel } from './labels/PublicElectionLabel';
import { RegisteredLabel } from './labels/RegisteredLabel';
import { VotedLabel } from './labels/VotedLabel';
import { DraftControls } from './controls/DraftControls';
import { RegistrationControls } from './controls/RegistrationControls';
import { InitFailedControls } from './controls/InitFailedControls';
import { VotingControls } from './controls/VotingControls';
import { CollectionFailedControls } from './controls/CollectionFailedControls';
import { FinishedControls } from './controls/FinishedControls';
import {
  getFatalError,
  tryReFetchElection,
  useClearState,
  useElectionId,
  useFetchElection,
  useFetchError,
  useSetElectionTitle,
} from './manageElectionActions';
import { useManageElectionNotifications } from './manageElectionNotifications';
import { useIsLoading } from './controls/controlsActions';
import styles from './manageElection.module.scss';

const useSelector = nestedSelectorHook('manageElection');

export const ManageElection = () => {
  useClearState();

  // Fetch the election to manage
  const electionId = useElectionId();
  useFetchElection(electionId);
  useManageElectionNotifications(electionId);

  const fetchError = useFetchError();

  // Set the title based on the election
  const electionDetails = useSelector((store) => store.electionDetails);
  const electionCollectors = useSelector((store) => store.electionCollectors);
  useSetElectionTitle(electionDetails);

  const userId = useUserId();
  const isLoading = useIsLoading();
  const lastLocation = useLastLocation();

  // Hide everything once the electio is deleted
  const electionDeleted = useSelector((store) => store.deletingElection);
  if (!electionDeleted.loading && electionDeleted.success && electionDeleted.data) {
    return null;
  }

  // Test for a fatal error when or after loading the resource
  const fatalError = getFatalError(electionDetails, userId);
  if (fatalError !== undefined) {
    return <ErrorOccured header="Error: Cannot view election!" message={fatalError} />;
  }

  // Show the blank loading form
  if (fetchError.loading || fetchError.data !== undefined) {
    return (
      <>
        <DashboardMenu />
        <Transition animation="scale" duration={300} transitionOnMount>
          <Container textAlign="center" text style={{ marginTop: '8em' }}>
            <Header size="large">Loading Election...</Header>

            <Segment raised padded loading style={{ minHeight: 200 }} />

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

  // Render the form as normal
  const election = (electionDetails as APISuccess<PublicElectionDetails>).data;
  const collectors = (electionCollectors as APISuccess<PublicCollectorList[]>).data;
  return (
    <>
      <DashboardMenu />

      <Container text style={{ marginTop: '8em', minWidth: 750 }} textAlign="center">
        <TransitionList animation="fade down" duration={500} totalDuration={300}>
          <Header size="large">{election.name}</Header>

          <Segment raised padded>
            <Grid columns={2} stackable textAlign="center" divided>
              <Grid.Row>
                <Grid.Column width="6">
                  <Flex justify="space-between">
                    <b>Status:</b>
                    <ElectionStatusLabel status={election.status} />
                  </Flex>

                  <Divider horizontal />
                  <PublicElectionLabel
                    election={election}
                    disabled={isLoading}
                    hidePopup={election.status !== ElectionStatus.Draft}
                  />

                  <RegisteredLabel election={election} />
                  <VotedLabel election={election} />
                </Grid.Column>

                <Grid.Column width="10" textAlign="left" stretched>
                  <Tab
                    className={styles['outer-scroll-list']}
                    panes={[
                      {
                        menuItem: { key: 'questions', icon: 'ordered list', content: 'Questions' },
                        render: () => (
                          <Tab.Pane className={styles['scroll-list']}>
                            <List ordered>
                              {election.questions.map((question) => (
                                <List.Item key={question.id} className={styles['question-item']}>
                                  {question.name}
                                  <List.List as="ul" className={styles.candidate}>
                                    {question.candidates.map((candidate, j) => (
                                      // This is intended for the preview to work correctly:
                                      // eslint-disable-next-line react/no-array-index-key
                                      <List.Item as="li" key={`${j}-${candidate}`}>
                                        {candidate}
                                      </List.Item>
                                    ))}
                                  </List.List>
                                </List.Item>
                              ))}
                            </List>
                          </Tab.Pane>
                        ),
                      },

                      {
                        menuItem: election.status !== ElectionStatus.Draft && (
                          <Menu.Item key="registered">
                            Registered
                            <Label content={election.registered.length.toString()} />
                          </Menu.Item>
                        ),
                        render: () => (
                          <Tab.Pane className={styles['scroll-list']}>
                            <List>
                              {election.registered.map((user) => (
                                <List.Item key={user.id} className={styles['question-item']}>
                                  <Flex justify="space-between" alignItems="center">
                                    {user.name}
                                    {user.hasVotedStatus === HasVotedStatus.Yes ? (
                                      <Label color="green" icon="check square outline" content="Voted" />
                                    ) : (
                                      <Label
                                        basic
                                        color="orange"
                                        icon="question"
                                        content="Partial Vote"
                                        style={{ visibility: user.hasVotedStatus ? undefined : 'hidden' }}
                                      />
                                    )}
                                  </Flex>
                                </List.Item>
                              ))}
                            </List>
                          </Tab.Pane>
                        ),
                      },

                      {
                        menuItem: collectors.length > 0 && (
                          <Menu.Item key="collectors">
                            Collectors
                            <Label content={collectors.length.toString()} />
                          </Menu.Item>
                        ),

                        render: () => (
                          <Tab.Pane className={styles['scroll-list']}>
                            <List as="ul">
                              {collectors.map((collector) => (
                                <List.Item key={collector.id} className={styles['question-item']} as="li">
                                  {collector.name}
                                </List.Item>
                              ))}
                            </List>
                          </Tab.Pane>
                        ),
                      },
                    ]}
                  />
                </Grid.Column>
              </Grid.Row>
            </Grid>

            {
              // Show the controls
              {
                [ElectionStatus.Draft]: <DraftControls election={election} />,
                [ElectionStatus.Registration]: <RegistrationControls election={election} />,
                [ElectionStatus.InitFailed]: <InitFailedControls election={election} />,
                [ElectionStatus.Voting]: <VotingControls election={election} />,
                [ElectionStatus.CollectionFailed]: <CollectionFailedControls election={election} />,
                [ElectionStatus.Finished]: <FinishedControls election={election} />,
              }[election.status]
            }
          </Segment>

          <div>
            <Button
              icon="arrow left"
              content="Go Back"
              style={{ marginTop: '2em' }}
              onClick={() => goBack(lastLocation)}
              disabled={isLoading}
            />
          </div>
        </TransitionList>
      </Container>
    </>
  );
};
