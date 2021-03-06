import { useHistory } from 'react-router-dom';
import { Container, Header, Divider, Icon, Card, Transition } from 'semantic-ui-react';
import { ElectionStatusLabel, TransitionList, ErrorPortal } from 'components/shared';
import { getErrorInformation } from 'api';
import { useUserId } from 'redux/auth';
import {
  DashboardFilter,
  getCardMetaText,
  getListHeader,
  reloadAllElections,
  showCreateCard,
  useFetchAllElections,
  useFilteredElections,
} from './dashboardActions';
import { useElectionListNotifications } from './dashboardNotifications';
import { CardPopup } from './CardPopup';
import styles from './dashboard.module.scss';

export interface ElectionListProps {
  filter: DashboardFilter;
}

export const ElectionsList = ({ filter }: ElectionListProps) => {
  useFetchAllElections();

  const filteredElections = useFilteredElections(filter);
  useElectionListNotifications(filteredElections);

  const userId = useUserId();
  const history = useHistory();

  return (
    <Container style={{ marginTop: '7em' }}>
      <Header as="h1" textAlign="center">
        {getListHeader(filter)}
      </Header>

      <Divider />

      <Card.Group stackable itemsPerRow="3">
        {showCreateCard(filter) && (
          <Transition animation="browse" duration={400} transitionOnMount>
            <Card
              as="a"
              className={styles['create-election-container']}
              onClick={() => history.push('/elections/create')}
            >
              <Card.Content className={`${styles['card-container-content']} ${styles['create-election']}`}>
                <Card.Header>
                  <Icon name="plus" />
                  Create Election
                </Card.Header>
              </Card.Content>
            </Card>
          </Transition>
        )}

        {!filteredElections.loading && filteredElections.success && (
          <TransitionList animation="fade down" totalDuration={1000}>
            {filteredElections.data.map((election) => (
              <Card as="a" key={election.id} onClick={() => history.push(`/elections/${election.id}`)}>
                <Card.Content>
                  <Card.Header content={election.name} />
                  <Card.Meta content={getCardMetaText(election)} />
                  <Card.Description>
                    <ElectionStatusLabel status={election.status} />
                  </Card.Description>
                </Card.Content>
                <CardPopup election={election} />
                <Card.Content extra>
                  <Icon name="user" />
                  <b>Creator: </b>
                  {election.createdBy.id === userId ? <u>Me</u> : election.createdBy.name}
                </Card.Content>
              </Card>
            ))}
          </TransitionList>
        )}
      </Card.Group>

      {!filteredElections.loading && !filteredElections.success && (
        <ErrorPortal
          negative
          header="Failed to load elections"
          content={getErrorInformation(filteredElections.error).description}
          onReload={reloadAllElections}
        />
      )}
    </Container>
  );
};
