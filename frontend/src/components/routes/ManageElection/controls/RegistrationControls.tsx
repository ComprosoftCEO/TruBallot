import { useState } from 'react';
import pluralize from 'pluralize';
import { Button, Message, Popup, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { usePermissions, useUserId } from 'redux/auth';
import { Permission } from 'models/auth';
import {
  clearRequests,
  openVoting,
  register,
  unregister,
  useElectionError,
  useIsLoading,
  MIN_REGISTERED_FOR_VOTING,
} from './controlsActions';
import { GeneratingModal } from './GeneratingModal';

export interface RegistrationControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const RegistrationControls = ({ election }: RegistrationControlsProps) => {
  const [popupOpen, setPopupOpen] = useState(false);

  const publishingElection = useSelector((state) => state.publishingElection);
  const registering = useSelector((state) => state.registering);
  const openingVoting = useSelector((state) => state.openingVoting);

  const userId = useUserId();
  const permissions = usePermissions();

  const loading = useIsLoading();
  const electionError = useElectionError();

  // Render nothing if permissions hide all controls
  if (!(permissions.has(Permission.CreateElection) || permissions.has(Permission.Register))) {
    return null;
  }

  return (
    <Transition animation="zoom" duration={400} transitionOnMount>
      <Segment secondary>
        {electionError !== undefined && (
          <Transition animation="fade down" duration={500} transitionOnMount>
            <Message negative onDismiss={clearRequests}>
              <b>Error: </b>
              {getErrorInformation(electionError.error).description}
            </Message>
          </Transition>
        )}

        {!publishingElection.loading && publishingElection.success && publishingElection.data && (
          <Transition animation="fade down" duration={500} transitionOnMount>
            <Message positive onDismiss={clearRequests}>
              Election has been published
            </Message>
          </Transition>
        )}

        {!registering.loading && registering.success && registering.data && (
          <Transition animation="fade down" duration={500} transitionOnMount>
            <Message positive onDismiss={clearRequests}>
              {election.isRegistered
                ? 'Successfully registered for election'
                : 'Successfully unregistered from election'}
            </Message>
          </Transition>
        )}

        <Flex justify="space-around">
          {permissions.has(Permission.Register) &&
            (election.isRegistered ? (
              <Button
                negative
                size="large"
                icon="cancel"
                content="Unregister"
                onClick={() => unregister(election.id)}
                disabled={loading}
                loading={registering.loading}
              />
            ) : (
              <Button
                primary
                size="large"
                icon="check square outline"
                content="Register"
                onClick={() => register(election.id)}
                disabled={loading}
                loading={registering.loading}
              />
            ))}

          {election.createdBy.id === userId && permissions.has(Permission.CreateElection) && (
            <Popup
              on="hover"
              wide
              position="right center"
              open={popupOpen && !loading && election.registered.length < MIN_REGISTERED_FOR_VOTING}
              onOpen={() => setPopupOpen(true)}
              onClose={() => setPopupOpen(false)}
              content={
                <Message
                  compact
                  icon="users"
                  content={`At least ${pluralize(
                    'registered user',
                    MIN_REGISTERED_FOR_VOTING,
                    true,
                  )} are needed before voting can begin`}
                />
              }
              trigger={
                <div>
                  <Button
                    color="blue"
                    size="large"
                    icon="list ordered"
                    content="Open Voting"
                    onClick={() => openVoting(election.id)}
                    disabled={loading || election.registered.length < MIN_REGISTERED_FOR_VOTING}
                  />
                </div>
              }
            />
          )}
        </Flex>

        <GeneratingModal open={openingVoting.loading} />
      </Segment>
    </Transition>
  );
};
