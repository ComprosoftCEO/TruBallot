import { Dimmer, Button, Segment, Message, Divider, Icon } from 'semantic-ui-react';
import { history } from 'App';
import { store } from 'state/store';

const loginRedirect = () => {
  store.globals.redirect.set(history.location.pathname);
  history.push('/login?redirect');
};

export const PleaseLogIn = () => (
  <Dimmer active>
    <Segment>
      <Message info>
        <Message.Header>Oops! You need to be logged in to view this page.</Message.Header>
      </Message>
      <Divider horizontal />
      <Button primary fluid color="orange" onClick={loginRedirect}>
        <Icon name="sign in" />
        Log In First
        <i> (Redirect)</i>
      </Button>

      <div style={{ height: '10px' }} />
      <Button fluid icon="home" content="Go Home" as="a" href="/" />
    </Segment>
  </Dimmer>
);
