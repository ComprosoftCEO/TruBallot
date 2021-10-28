/**
 * Dialog shown if the user needs to log in before they can access a route
 */
import { Image, Dimmer, Button, Segment, Message, Divider, Icon } from 'semantic-ui-react';
import { loginRedirect, goHome } from './errorDialogActions';

export const PleaseLogIn = () => (
  <Dimmer active>
    <Segment>
      <Message info>
        <Message.Header>Oops! You need to be logged in to view this page.</Message.Header>
      </Message>

      <Image src="/truballot-logo.svg" spaced size="large" centered style={{ padding: '20px 0' }} />

      <Divider horizontal />
      <Button color="red" fluid onClick={loginRedirect}>
        <Icon name="sign in" />
        Log In First <i>(Redirect)</i>
      </Button>

      <div style={{ height: 10 }} />
      <Button primary fluid icon="home" content="Go Home" onClick={goHome} />
    </Segment>
  </Dimmer>
);
