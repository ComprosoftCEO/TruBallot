import { Container, Menu, Image, Dropdown, Header, Divider, Icon, Card, Label, Popup } from 'semantic-ui-react';
import { ElectionStatusLabel, TransitionList } from 'components/shared';
import _ from 'lodash';
import { logOut, useClearState } from './dashboardActions';
import styles from './dashboard.module.scss';

export const Dashboard = () => {
  useClearState();

  return (
    <>
      <Menu fixed="top">
        <Container>
          <Menu.Item header>
            <Image size="mini" src="/truballot-icon.svg" style={{ marginRight: '1em', height: 35, top: -4 }} />
            TruBallot
          </Menu.Item>

          <Menu.Item as="a" content="Home" />

          <Dropdown item simple text="My Elections">
            <Dropdown.Menu>
              <Dropdown.Item icon="list" content="All" />

              <Dropdown.Divider />

              <Dropdown.Header content="Draft" />
              <Dropdown.Item icon={{ name: 'edit', color: 'yellow' }} content="Drafts" />

              <Dropdown.Divider />

              <Dropdown.Header content="Published" />
              <Dropdown.Item icon={{ name: 'clipboard', color: 'brown' }} content="Open for Registration" />
              <Dropdown.Item icon={{ name: 'check square', color: 'green' }} content="Voting" />
              <Dropdown.Item icon={{ name: 'clock', color: 'red' }} content="Closed" />

              <Dropdown.Divider />

              <Dropdown.Item icon={{ name: 'plus' }} text="Create Election" />
            </Dropdown.Menu>
          </Dropdown>

          <Dropdown item simple text="Public Elections">
            <Dropdown.Menu>
              <Dropdown.Item icon="list" content="All" />

              <Dropdown.Divider />

              <Dropdown.Item icon={{ name: 'clipboard', color: 'brown' }} content="Open for Registration" />
              <Dropdown.Item icon={{ name: 'check square', color: 'green' }} content="Voting" />
              <Dropdown.Item icon={{ name: 'clock', color: 'red' }} content="Closed" />
            </Dropdown.Menu>
          </Dropdown>

          <Dropdown item simple text="Registrations">
            <Dropdown.Menu>
              <Dropdown.Item icon="list" content="All" />

              <Dropdown.Divider />

              <Dropdown.Item icon={{ name: 'clipboard', color: 'brown' }} content="Open" />
              <Dropdown.Item icon={{ name: 'check square', color: 'green' }} content="Voting" />
              <Dropdown.Item icon={{ name: 'clock', color: 'red' }} content="Closed" />

              <Dropdown.Divider />

              <Dropdown.Item icon={{ name: 'lock' }} content="Access Code" />
            </Dropdown.Menu>
          </Dropdown>

          <Menu.Menu position="right">
            <Dropdown item simple text="Bryan McClain">
              <Dropdown.Menu>
                <Dropdown.Header content="Account" />
                <Dropdown.Item icon="cog" text="Preferences" />
                <Dropdown.Divider />
                <Dropdown.Item icon="log out" text="Log Out" onClick={logOut} />
              </Dropdown.Menu>
            </Dropdown>
          </Menu.Menu>
        </Container>
      </Menu>

      <Container style={{ marginTop: '7em' }}>
        <Header as="h1" textAlign="center">
          My Elections
        </Header>

        <Divider />

        <Card.Group stackable itemsPerRow="3" centered={[].length === 0 && false}>
          <Card as="a" className={styles['create-election']}>
            <Card.Content className={styles['create-election-content']}>
              <Card.Header>
                <Icon name="plus" />
                Create Election
              </Card.Header>
            </Card.Content>
          </Card>

          <TransitionList animation="fade down">
            {_.range(1, 20).map((i) => (
              <Card as="a">
                <Card.Content>
                  <Card.Header content="Vote for President" />
                  <Card.Meta content={`${i} Questions, ${2 * i} Registered`} />
                  <Card.Description>
                    <ElectionStatusLabel status={i % 6} />
                  </Card.Description>
                </Card.Content>
                {i % 6 === 0 && (
                  <Popup
                    on="hover"
                    size="mini"
                    content={<Label color="green" content="Voted" />}
                    position="right center"
                    trigger={<Label corner="right" color="green" icon="check square outline" />}
                  />
                )}
                <Card.Content extra>
                  <Icon name="user" />
                  <b>Creator: </b>
                  Bryan McClain
                </Card.Content>
              </Card>
            ))}
          </TransitionList>
        </Card.Group>
      </Container>
    </>
  );
};
