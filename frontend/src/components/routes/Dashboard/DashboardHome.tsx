import { useHistory } from 'react-router-dom';
import { Card, Container, Divider, Header, Icon, Image, Segment } from 'semantic-ui-react';
import { TransitionList } from 'components/shared';
import { nestedSelectorHook } from 'redux/helpers';
import { Permission } from 'models/auth';
import styles from './dashboard.module.scss';

const useGlobalsSelector = nestedSelectorHook('globals');

export const DashboardHome = () => {
  const history = useHistory();

  const name = useGlobalsSelector((state) => state.name);
  const permissions = useGlobalsSelector((state) => state.permissions);

  return (
    <Container style={{ marginTop: '7em' }} textAlign="center">
      <TransitionList animation="scale" totalDuration={300}>
        <Image src="/truballot-logo.svg" spaced size="big" centered style={{ padding: '20px 0' }} />

        <Divider />

        <Header as="h1" textAlign="center" style={{ marginTop: '2em' }}>
          {`Welcome, ${name}`}
        </Header>

        <Segment secondary>
          <Card.Group stackable itemsPerRow="3">
            {permissions.has(Permission.CreateElection) && (
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
            )}

            <Card as="a" onClick={() => history.push('/dashboard/my-elections')} raised>
              <Card.Content className={styles['card-container-content']}>
                <Card.Header>
                  <Icon name="edit outline" />
                  My Elections
                </Card.Header>
              </Card.Content>
            </Card>

            <Card as="a" onClick={() => history.push('/dashboard/public-elections')} raised>
              <Card.Content className={styles['card-container-content']}>
                <Card.Header>
                  <Icon name="users" />
                  Public Elections
                </Card.Header>
              </Card.Content>
            </Card>

            <Card as="a" onClick={() => history.push('/dashboard/registrations')} raised>
              <Card.Content className={styles['card-container-content']}>
                <Card.Header>
                  <Icon name="clipboard" />
                  Registered Elections
                </Card.Header>
              </Card.Content>
            </Card>

            <Card as="a" onClick={() => history.push('/elections/access-code')} raised>
              <Card.Content className={styles['card-container-content']}>
                <Card.Header>
                  <Icon name="key" />
                  Access Code
                </Card.Header>
              </Card.Content>
            </Card>
          </Card.Group>
        </Segment>
      </TransitionList>
    </Container>
  );
};
