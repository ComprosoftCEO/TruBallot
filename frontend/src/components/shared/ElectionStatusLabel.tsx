import { ElectionStatus } from 'models/election';
import React from 'react';
import { Label } from 'semantic-ui-react';

export interface ElectionStatusLabelProps {
  status: ElectionStatus;
}

const DRAFT_LABEL = <Label basic icon="edit" color="yellow" content="Draft" />;
const REGISTRATION_LABEL = <Label basic icon="clipboard outline" color="brown" content="Open for Registration" />;
const VOTING_INIT_LABEL = <Label basic icon="check square outline" color="olive" content="Initializing Voting" />;
const VOTING_LABEL = <Label basic icon="check square outline" color="green" content="Voting" />;
const COLLECTION_LABEL = <Label basic icon="clock outline" color="orange" content="Collecting Votes" />;
const FINISHED_LABEL = <Label basic icon="clock outline" color="red" content="Closed" />;

const ALL_LABELS: Record<ElectionStatus, JSX.Element> = {
  [ElectionStatus.Draft]: DRAFT_LABEL,
  [ElectionStatus.Registration]: REGISTRATION_LABEL,
  [ElectionStatus.InitFailed]: VOTING_INIT_LABEL,
  [ElectionStatus.Voting]: VOTING_LABEL,
  [ElectionStatus.CollectionFailed]: COLLECTION_LABEL,
  [ElectionStatus.Finished]: FINISHED_LABEL,
};

export const ElectionStatusLabel = ({ status }: ElectionStatusLabelProps): JSX.Element => ALL_LABELS[status];
