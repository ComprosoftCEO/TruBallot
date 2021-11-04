import { Button, Message, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { clearRequests, openVoting, useElectionError, useIsLoading, useUserId } from './controlsActions';
import { GeneratingModal } from './GeneratingModal';

export interface InitFailedControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const InitFailedControls = ({ election }: InitFailedControlsProps) => {
  const openingVoting = useSelector((state) => state.openingVoting);
  const userId = useUserId();

  const loading = useIsLoading();
  const electionError = useElectionError();

  // Disable the controls if this election is not created by the current user
  if (userId !== election.createdBy.id) {
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

        <Flex justify="space-around">
          <Button
            color="blue"
            size="large"
            icon="list ordered"
            content="Finish Initialization"
            onClick={() => openVoting(election.id)}
            disabled={loading}
          />
        </Flex>

        <GeneratingModal open={openingVoting.loading} />
      </Segment>
    </Transition>
  );
};
