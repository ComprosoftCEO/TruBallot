import { Divider, Label } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { PublicElectionDetails, ElectionStatus } from 'models/election';

export interface VotedLabelProps {
  election: PublicElectionDetails;
}

export const VotedLabel = ({ election }: VotedLabelProps): JSX.Element => (
  <>
    {election.status !== ElectionStatus.Draft && (
      <>
        <Divider horizontal />
        <Flex justify="space-between">
          <b>Voted?</b>
          {election.isRegistered ? (
            <Label basic color="green" icon="check" content="Yes" />
          ) : (
            <Label basic color="red" icon="cancel" content="No" />
          )}
        </Flex>
      </>
    )}
  </>
);
