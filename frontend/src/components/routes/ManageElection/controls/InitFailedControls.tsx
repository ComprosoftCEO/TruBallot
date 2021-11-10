import { Button, Message, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { usePermissions, useUserId } from 'redux/auth';
import { Permission } from 'models/auth';
import { clearRequests, openVoting, useElectionError, useIsLoading } from './controlsActions';
import { GeneratingModal } from './GeneratingModal';

export interface InitFailedControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const InitFailedControls = ({ election }: InitFailedControlsProps) => {
  const openingVoting = useSelector((state) => state.openingVoting);
  const userId = useUserId();
  const permissions = usePermissions();

  const loading = useIsLoading();
  const electionError = useElectionError();

  // Disable the controls if this election is not created by the current user
  //  (Or user does not have permission)
  if (userId !== election.createdBy.id || !permissions.has(Permission.CreateElection)) {
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
            onClick={() => openVoting(election.id, true)}
            disabled={loading}
          />
        </Flex>

        <GeneratingModal open={openingVoting.loading} />
      </Segment>
    </Transition>
  );
};
