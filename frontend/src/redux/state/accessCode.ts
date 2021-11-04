/*
 * State used on the access code interface
 */
import { APIResult, apiSuccess } from 'api';

export interface AccessCodeState {
  code: string;
  loadingElection: APIResult<{}>;
}

export const initialAccessCodeState: AccessCodeState = {
  code: '',
  loadingElection: apiSuccess({}),
};
