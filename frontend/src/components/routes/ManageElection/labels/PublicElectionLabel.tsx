import { useState } from 'react';
import { Label, Popup } from 'semantic-ui-react';
import { ClipboardLabel, Flex, PublicElectionMessge } from 'components/shared';
import { PublicElectionDetails } from 'models/election';

export interface PublicElectionLabelProps {
  election: PublicElectionDetails;
  disabled?: boolean;
  hidePopup?: boolean;
}

export const PublicElectionLabel = ({ election, disabled, hidePopup }: PublicElectionLabelProps) => {
  const [publicHover, setPublicHover] = useState(false);

  if (election.accessCode !== undefined) {
    return (
      <Flex justify="space-between">
        <b>Access Code:</b>
        <ClipboardLabel value={election.accessCode} disabled={disabled} />
      </Flex>
    );
  }

  return (
    <Flex justify="space-between">
      <b>Is Public?</b>
      <Popup
        on="hover"
        wide="very"
        position="right center"
        style={{ zIndex: 900 }}
        open={publicHover && !disabled && !hidePopup}
        onOpen={() => setPublicHover(true)}
        onClose={() => setPublicHover(false)}
        content={<PublicElectionMessge isPublic={election.isPublic} />}
        trigger={
          election.isPublic ? (
            <Label basic color="green" icon="lock open" content="Yes" />
          ) : (
            <Label basic color="olive" icon="lock" content="No" />
          )
        }
      />
    </Flex>
  );
};
