import { useState } from 'react';
import { useHistory } from 'react-router-dom';
import pluralize from 'pluralize';
import { Button, Message, Popup, Segment, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { Flex } from 'components/shared';
import { HasVotedStatus, PublicElectionDetails } from 'models/election';
import { nestedSelectorHook } from 'redux/helpers';
import { usePermissions, useUserId } from 'redux/auth';
import { Permission } from 'models/auth';
import { FinishingModal } from './FinishingModal';
import {
  clearRequests,
  useElectionError,
  useIsLoading,
  MIN_VOTES_FOR_CLOSING,
  validateNumberOfVotes,
  closeVoting,
} from './controlsActions';

export interface VotingControlsProps {
  election: PublicElectionDetails;
}

const useSelector = nestedSelectorHook('manageElection');

export const VotingControls = ({ election }: VotingControlsProps) => {
  const [popupOpen, setPopupOpen] = useState(false);

  const openingVoting = useSelector((state) => state.openingVoting);
  const closingElection = useSelector((state) => state.closingVoting);

  const userId = useUserId();
  const permissions = usePermissions();

  const numberOfVotesValid = validateNumberOfVotes(election.questions);

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

        {!openingVoting.loading && openingVoting.success && openingVoting.data && (
          <Transition animation="fade down" duration={500} transitionOnMount>
            <Message positive onDismiss={clearRequests}>
              Election is open for voting
            </Message>
          </Transition>
        )}

        <Flex justify="space-around">
          {election.isRegistered &&
            election.hasVotedStatus !== HasVotedStatus.Yes &&
            permissions.has(Permission.Vote) && (
              <Button
                primary
                size="large"
                icon="check square outline"
                content="Vote"
                onClick={() => history.push(`/elections/${election.id}/vote`)}
                disabled={loading}
              />
            )}

          <Button
            size="large"
            icon="list ordered"
            content="Results"
            onClick={() => history.push(`/elections/${election.id}/results`)}
            disabled={loading}
          />

          {election.createdBy.id === userId && permissions.has(Permission.CreateElection) && (
            <Popup
              on="hover"
              wide
              position="right center"
              open={popupOpen && !loading && !numberOfVotesValid}
              onOpen={() => setPopupOpen(true)}
              onClose={() => setPopupOpen(false)}
              content={
                <Message
                  compact
                  icon="check square outline"
                  content={`Each question must have at least ${pluralize(
                    'vote',
                    MIN_VOTES_FOR_CLOSING,
                    true,
                  )} before election can be closed`}
                />
              }
              trigger={
                <div>
                  <Button
                    color="blue"
                    size="large"
                    icon="table"
                    content="Close Voting"
                    onClick={() => closeVoting(election.id)}
                    disabled={loading || !numberOfVotesValid}
                  />
                </div>
              }
            />
          )}
        </Flex>

        <FinishingModal open={closingElection.loading} />
      </Segment>
    </Transition>
  );
};
