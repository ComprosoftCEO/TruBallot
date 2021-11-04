import { useHistory } from 'react-router-dom';
import { Button, Message, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { Permission } from 'models/auth';
import { FinishingModal } from './FinishingModal';
import {
  clearRequests,
  useElectionError,
  useIsLoading,
  useUserId,
  closeVoting,
  usePermissions,
} from './controlsActions';

export interface CollectionFailedControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const CollectionFailedControls = ({ election }: CollectionFailedControlsProps) => {
  const closingElection = useSelector((state) => state.closingVoting);

  const userId = useUserId();
  const permissions = usePermissions();

  const loading = useIsLoading();
  const electionError = useElectionError();
  const history = useHistory();

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
            size="large"
            icon="list ordered"
            content="Results"
            onClick={() => history.push(`/elections/${election.id}/results`)}
            disabled={loading}
          />

          {election.createdBy.id === userId && permissions.has(Permission.CreateElection) && (
            <Button
              color="blue"
              size="large"
              icon="table"
              content="Finish Collection"
              onClick={() => closeVoting(election.id)}
              disabled={loading}
            />
          )}
        </Flex>

        <FinishingModal open={closingElection.loading} />
      </Segment>
    </Transition>
  );
};
