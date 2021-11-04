/*
 * State used on the manage election interface
 */
import { apiLoading, APIResult, apiSuccess } from 'api';
import { PublicElectionDetails } from 'models/election';

export interface ManageElectionState {
  electionDetails: APIResult<PublicElectionDetails>;

  deletingElection: APIResult<boolean>;
  publishingElection: APIResult<boolean>;
  registering: APIResult<boolean>;
  openingVoting: APIResult<boolean>;
  closingVoting: APIResult<boolean>;
}

export const initialManageElectionState: ManageElectionState = {
  electionDetails: apiLoading(),

  deletingElection: apiSuccess(false),
  publishingElection: apiSuccess(false),
  registering: apiSuccess(false),
  openingVoting: apiSuccess(false),
  closingVoting: apiSuccess(false),
};
