/*
 * State used on the manage election interface
 */
import { apiLoading, APIResult, apiSuccess } from 'api';
import { PublicElectionDetails } from 'models/election';
import { PublicCollectorList } from 'models/mediator';

export interface ManageElectionState {
  electionDetails: APIResult<PublicElectionDetails>;
  allCollectors: APIResult<PublicCollectorList[]>;
  electionCollectors: APIResult<PublicCollectorList[]>;

  deletingElection: APIResult<boolean>;
  publishingElection: APIResult<boolean>;
  registering: APIResult<boolean>;
  openingVoting: APIResult<boolean>;
  closingVoting: APIResult<boolean>;

  pickCollectorsModalOpen: boolean;
  collectorsSelected: Set<string>;
}

export const initialManageElectionState: ManageElectionState = {
  electionDetails: apiLoading(),
  allCollectors: apiLoading(),
  electionCollectors: apiLoading(),

  deletingElection: apiSuccess(false),
  publishingElection: apiSuccess(false),
  registering: apiSuccess(false),
  openingVoting: apiSuccess(false),
  closingVoting: apiSuccess(false),

  pickCollectorsModalOpen: false,
  collectorsSelected: new Set(),
};
