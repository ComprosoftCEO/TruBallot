import { useHistory } from 'react-router-dom';
import { Button, Message, Segment, Transition } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { clearRequests, useIsLoading } from './controlsActions';

export interface FinishedControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const FinishedControls = ({ election }: FinishedControlsProps) => {
  const closingElection = useSelector((state) => state.closingVoting);

  const loading = useIsLoading();
  const history = useHistory();

  return (
    <Transition animation="zoom" duration={400} transitionOnMount>
      <Segment secondary>
        {!closingElection.loading && closingElection.success && closingElection.data && (
          <Transition animation="fade down" duration={500} transitionOnMount>
            <Message positive onDismiss={clearRequests}>
              Election is finished! You can now view the final tally.
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
        </Flex>
      </Segment>
    </Transition>
  );
};
