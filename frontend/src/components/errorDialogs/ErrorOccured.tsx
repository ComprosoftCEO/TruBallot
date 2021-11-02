/**
 * Dialog shown when an unexpected error occurs in the web application
 */
import { Dimmer, Message, Segment, Button, Divider, Image } from 'semantic-ui-react';

export interface ErrorOccuredProps {
  header?: string;
  message: string;
  allowReload?: boolean;
}

export const ErrorOccured = ({
  header = 'Oops! Looks like something went wrong.',
  message,
  allowReload,
}: ErrorOccuredProps) => (
  <Dimmer active>
    <Segment>
      <Message negative>
        <Message.Header>{header}</Message.Header>
        {message}
      </Message>

      <Image src="/truballot-logo.svg" spaced size="large" centered style={{ padding: '20px 0' }} />

      <Divider horizontal />

      {allowReload && (
        <>
          <Button fluid icon="redo" content="Reload Page" onClick={() => window.location.reload()} />
          <div style={{ height: 10 }} />
        </>
      )}

      <Button primary fluid icon="home" content="Go Home" as="a" href="/" />
    </Segment>
  </Dimmer>
);
