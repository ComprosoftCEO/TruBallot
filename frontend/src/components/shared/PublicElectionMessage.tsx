import { Message } from 'semantic-ui-react';

export interface PublicElectionMessageProps {
  isPublic: boolean;
}

export const PublicElectionMessge = ({ isPublic }: PublicElectionMessageProps) =>
  isPublic ? (
    <Message compact icon="lock open" header="Public Election" content="Open for anyone on the site to register" />
  ) : (
    <Message compact icon="lock" header="Private Election" content="Requires an access code to register" />
  );
