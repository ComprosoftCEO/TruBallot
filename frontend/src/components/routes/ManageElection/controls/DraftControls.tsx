import { useHistory } from 'react-router-dom';
import { Button, Message, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { usePermissions, useUserId } from 'redux/auth';
import { Permission } from 'models/auth';
import { clearRequests, deleteElection, publishElection, useElectionError, useIsLoading } from './controlsActions';

export interface DraftControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const DraftControls = ({ election }: DraftControlsProps) => {
  const deletingElection = useSelector((state) => state.deletingElection);
  const publishingElection = useSelector((state) => state.publishingElection);

  const userId = useUserId();
  const permissions = usePermissions();

  const loading = useIsLoading();
  const electionError = useElectionError();
  const history = useHistory();

  // Disable the controls if this election is not created by the current user
  //   In reality, we can ONLY read a draft election if we own it, so this
  //   check is probably unnecessary
  //
  // User also must have permission to create an election
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
            primary
            size="large"
            icon="edit outline"
            content="Edit"
            onClick={() => history.push(`/elections/${election.id}/edit`)}
            disabled={loading}
          />

          <Button
            negative
            size="large"
            icon="trash"
            content="Delete"
            onClick={() => deleteElection(election.id)}
            disabled={loading}
            loading={deletingElection.loading}
          />

          <Button
            color="blue"
            size="large"
            icon="send"
            content="Publish"
            onClick={() => publishElection(election.id)}
            disabled={loading}
            loading={publishingElection.loading}
          />
        </Flex>
      </Segment>
    </Transition>
  );
};
