import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { useHistory } from 'react-router-dom';
import { nestedSelectorHook } from 'redux/helpers';
import { Message, Segment, Transition } from 'semantic-ui-react';
import { clearRequests, useElectionError, useIsLoading, useUserId } from './controlsActions';

export interface RegistrationControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const RegistrationControls = ({ election }: RegistrationControlsProps) => {
  const publishingElection = useSelector((state) => state.publishingElection);
  const userId = useUserId();

  const loading = useIsLoading();
  const electionError = useElectionError();

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

        <Flex justify="space-around" />
      </Segment>
    </Transition>
  );
};
