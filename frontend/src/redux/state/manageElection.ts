/*
 * State used on the manage election interface
 */
import { apiLoading, APIResult, apiSuccess } from 'api';
import { PublicElectionDetails } from 'models/election';

export interface ManageElectionState {
  electionDetails: APIResult<PublicElectionDetails>;

  publishingElection: APIResult<{}>;
  deletingElection: APIResult<boolean>;
  openingVoting: APIResult<boolean>;
  closingVoting: APIResult<boolean>;
}

export const initialManageElectionState: ManageElectionState = {
  electionDetails: apiLoading(),

  publishingElection: apiSuccess({}),
  deletingElection: apiSuccess(false),
  openingVoting: apiSuccess(false),
  closingVoting: apiSuccess(false),
};
