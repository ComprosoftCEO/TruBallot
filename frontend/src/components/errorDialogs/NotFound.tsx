/**
 * Dialog shown for a route that does not exist
 */
import { Button, Dimmer, Divider, Segment, Image, Message } from 'semantic-ui-react';
import { goHome } from './errorDialogActions';

export interface NotFoundProps {
  noPermission?: boolean;
}

export const NotFound = ({ noPermission }: NotFoundProps) => (
  <Dimmer active>
    <Segment>
      <Message negative>
        <Message.Header>Oops! Looks like there is nothing here.</Message.Header>
        {noPermission ? "You don't have permission to access this page" : 'Page Not Found'}
      </Message>
      <Image src={noPermission ? '/secret.png' : '/404.png'} spaced size="large" centered />

      <Divider horizontal />
      <Button primary fluid icon="home" content="Go Home" onClick={goHome} />
    </Segment>
  </Dimmer>
);
