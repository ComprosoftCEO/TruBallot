/*
 * State used for creating and editing the elections
 */
import { apiLoading, APIResult, apiSuccess } from 'api/types';
import { PublicElectionDetails } from 'models/election';

export interface EditorState {
  // Values used by the editor
  name: string;
  isPublic: boolean;
  questions: string;
  modified: boolean;

  submitting: APIResult<{}>;

  // Values used when editing an existing election
  electionDetails: APIResult<PublicElectionDetails>;
  reloading: APIResult<{}>;
}

export const initialEditorState: EditorState = {
  name: '',
  isPublic: false,
  questions: '',
  modified: false,

  submitting: apiSuccess({}),

  electionDetails: apiLoading(),
  reloading: apiSuccess({}),
};
