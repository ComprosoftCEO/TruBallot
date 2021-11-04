import { Divider, Label } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { HasVotedStatus, PublicElectionDetails, ElectionStatus } from 'models/election';

export interface VotedLabelProps {
  election: PublicElectionDetails;
}

const HIDE_LABEL_STATUS: ElectionStatus[] = [
  ElectionStatus.Draft,
  ElectionStatus.Registration,
  ElectionStatus.InitFailed,
];

export const VotedLabel = ({ election }: VotedLabelProps): JSX.Element => (
  <>
    {!HIDE_LABEL_STATUS.includes(election.status) && election.isRegistered && (
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
