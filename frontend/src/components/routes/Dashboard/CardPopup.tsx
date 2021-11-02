import { ElectionStatus, PublicElectionList } from 'models/election';
import { Label, Popup, IconProps, SemanticShorthandItem, SemanticCOLORS } from 'semantic-ui-react';

export interface CardPopupProps {
  election: PublicElectionList;
}

const REGISTRATION_STATUS: ElectionStatus[] = [ElectionStatus.Registration, ElectionStatus.InitFailed];
const VOTING_STATUS: ElectionStatus[] = [ElectionStatus.Voting];
const CLOSED_STATUS: ElectionStatus[] = [ElectionStatus.CollectionFailed, ElectionStatus.Finished];

export const CardPopup = ({ election }: CardPopupProps) => {
  if (!election.isRegistered) {
    return null;
  }

  // Test for registration
  if (REGISTRATION_STATUS.includes(election.status)) {
    return <PopupLabel text="Registered" color="brown" icon="clipboard outline" />;
  }

  // Test for voting
  if (VOTING_STATUS.includes(election.status)) {
    return election.hasVoted ? (
      <PopupLabel text="Voted" color="green" icon="check square outline" />
    ) : (
      <PopupLabel text="Haven't Voted Yet" color="blue" icon="info" />
    );
  }

  // Test for closed
  if (CLOSED_STATUS.includes(election.status)) {
    return election.hasVoted ? (
      <PopupLabel text="Voted" color="green" icon="check square outline" />
    ) : (
      <PopupLabel text="Didn't Vote" color="olive" icon="cancel" />
    );
  }

  return null;
};

interface PopupProps {
  text: string;
  color: SemanticCOLORS;
  icon: SemanticShorthandItem<IconProps>;
}

function PopupLabel({ text, color, icon }: PopupProps) {
  return (
    <Popup
      on="hover"
      size="mini"
      content={<Label color={color} content={text} />}
      position="right center"
      trigger={<Label corner="right" color={color} icon={icon} />}
    />
  );
}
