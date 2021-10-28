/**
 * Dialog shown when the JWT token expires while using the web app
 */
import { useCallback } from 'react';
import { Image, Dimmer, Button, Icon, Divider, Segment, Message } from 'semantic-ui-react';
import { loginRedirect } from './errorDialogActions';

export interface SessionExpiredProps {
  clearError: () => void;
}

export const SessionExpired = ({ clearError }: SessionExpiredProps) => {
  const clearErrorAndRedirect = useCallback(() => {
    clearError();
    loginRedirect();
  }, [clearError]);

  return (
    <Dimmer active>
      <Segment>
        <Message info>
          <Message.Header>Oops! It looks like your login session has expired due to inactivity.</Message.Header>
          Please log back in to continue using the system.
        </Message>

        <Image src="/truballot-logo.svg" spaced size="large" centered style={{ padding: '20px 0' }} />

        <Divider horizontal />
        <Button color="red" fluid onClick={clearErrorAndRedirect}>
          <Icon name="sign in" />
          Log Back In <i>(Redirect)</i>
        </Button>

        <div style={{ height: 20 }} />
        <Button fluid icon="redo" content="Reload Page" onClick={window.location.reload} />

        <div style={{ height: 10 }} />
        <Button color="violet" fluid icon="home" content="Go Home" as="a" href="/" />
      </Segment>
    </Dimmer>
  );
};
