import { Container, Menu, Image, Dropdown } from 'semantic-ui-react';
import { useHistory } from 'react-router-dom';
import { nestedSelectorHook } from 'redux/helpers';
import { logOut } from './dashboardActions';

const useGlobalsSelector = nestedSelectorHook('globals');

export const DashboardMenu = () => {
  const history = useHistory();
  const name = useGlobalsSelector((state) => state.name);

  return (
    <Menu fixed="top">
      <Container>
        <Menu.Item header>
          <Image size="mini" src="/truballot-icon.svg" style={{ marginRight: '1em', height: 35, top: -4 }} />
          TruBallot
        </Menu.Item>

        <Menu.Item as="a" content="Home" onClick={() => history.push('/')} />

        <Dropdown item simple text="My Elections">
          <Dropdown.Menu>
            <Dropdown.Item icon="list" content="All" onClick={() => history.push('/dashboard/my-elections')} />

            <Dropdown.Divider />

            <Dropdown.Header content="Draft" />
            <Dropdown.Item
              icon={{ name: 'edit', color: 'yellow' }}
              content="Drafts"
              onClick={() => history.push('/dashboard/my-elections/drafts')}
            />

            <Dropdown.Divider />

            <Dropdown.Header content="Published" />
            <Dropdown.Item
              icon={{ name: 'clipboard', color: 'brown' }}
              content="Open for Registration"
              onClick={() => history.push('/dashboard/my-elections/open')}
            />
            <Dropdown.Item
              icon={{ name: 'check square', color: 'green' }}
              content="Voting"
              onClick={() => history.push('/dashboard/my-elections/voting')}
            />
            <Dropdown.Item
              icon={{ name: 'clock', color: 'red' }}
              content="Closed"
              onClick={() => history.push('/dashboard/my-elections/closed')}
            />

            <Dropdown.Divider />

            <Dropdown.Item
              icon={{ name: 'plus' }}
              text="Create Election"
              onClick={() => history.push('/elections/create')}
            />
          </Dropdown.Menu>
        </Dropdown>

        <Dropdown item simple text="Public Elections">
          <Dropdown.Menu>
            <Dropdown.Item icon="list" content="All" onClick={() => history.push('/dashboard/public-elections')} />

            <Dropdown.Divider />

            <Dropdown.Item
              icon={{ name: 'clipboard', color: 'brown' }}
              content="Open for Registration"
              onClick={() => history.push('/dashboard/public-elections/open')}
            />
            <Dropdown.Item
              icon={{ name: 'check square', color: 'green' }}
              content="Voting"
              onClick={() => history.push('/dashboard/public-elections/voting')}
            />
            <Dropdown.Item
              icon={{ name: 'clock', color: 'red' }}
              content="Closed"
              onClick={() => history.push('/dashboard/public-elections/closed')}
            />
          </Dropdown.Menu>
        </Dropdown>

        <Dropdown item simple text="Registrations">
          <Dropdown.Menu>
            <Dropdown.Item icon="list" content="All" onClick={() => history.push('/dashboard/registrations')} />

            <Dropdown.Divider />

            <Dropdown.Item
              icon={{ name: 'clipboard', color: 'brown' }}
              content="Open"
              onClick={() => history.push('/dashboard/registrations/open')}
            />
            <Dropdown.Item
              icon={{ name: 'check square', color: 'green' }}
              content="Voting"
              onClick={() => history.push('/dashboard/registrations/voting')}
            />
            <Dropdown.Item
              icon={{ name: 'clock', color: 'red' }}
              content="Closed"
              onClick={() => history.push('/dashboard/registrations/closed')}
            />

            <Dropdown.Divider />

            <Dropdown.Item
              icon={{ name: 'lock' }}
              content="Access Code"
              onClick={() => history.push('/elections/access-code')}
            />
          </Dropdown.Menu>
        </Dropdown>

        <Menu.Menu position="right">
          <Dropdown item simple text={name}>
            <Dropdown.Menu>
              <Dropdown.Header content="Account" />
              <Dropdown.Item icon="cog" text="Preferences" onClick={() => history.push('/preferences')} />

              <Dropdown.Divider />

              <Dropdown.Item icon="log out" text="Log Out" onClick={logOut} />
            </Dropdown.Menu>
          </Dropdown>
        </Menu.Menu>
      </Container>
    </Menu>
  );
};
