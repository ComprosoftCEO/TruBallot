import { Divider, Label } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { HasVotedStatus, PublicElectionDetails, ElectionStatus } from 'models/election';

export interface VotedLabelProps {
  election: PublicElectionDetails;
}

export const VotedLabel = ({ election }: VotedLabelProps): JSX.Element => (
  <>
    {election.status !== ElectionStatus.Draft && election.isRegistered && (
      <>
        <Divider horizontal />
        <Flex justify="space-between">
          <b>Voted?</b>
          {
            {
              [HasVotedStatus.No]: <Label basic color="red" icon="cancel" content="No" />,
              [HasVotedStatus.Partial]: <Label basic color="orange" icon="info" content="Partial" />,
              [HasVotedStatus.Yes]: <Label basic color="green" icon="check" content="Yes" />,
            }[election.hasVoted]
          }
        </Flex>
      </>
    )}
  </>
);
