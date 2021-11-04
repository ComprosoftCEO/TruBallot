import { Divider, Label } from 'semantic-ui-react';
import { Flex } from 'components/shared';
import { PublicElectionDetails, ElectionStatus } from 'models/election';

export interface RegisteredLabelProps {
  election: PublicElectionDetails;
}

export const RegisteredLabel = ({ election }: RegisteredLabelProps): JSX.Element => (
  <>
    {election.status !== ElectionStatus.Draft && (
      <>
        <Divider horizontal />
        <Flex justify="space-between">
          <b>Registered?</b>
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
