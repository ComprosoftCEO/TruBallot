/*
 * State used for creating and editing the elections
 */
import { apiLoading, APIResult, apiSuccess } from 'api/types';
import { PublicElectionDetails } from 'models/election';

export interface EditorState {
  electionDetails: APIResult<PublicElectionDetails>;

  // Values used by the editor
  name: string;
  isPublic: boolean;
  questions: string;
  modified: boolean;

  submitting: APIResult<{}>;
}

export const initialEditorState: EditorState = {
  electionDetails: apiLoading(),
  name: '',
  isPublic: false,
  questions: '',
  modified: false,
  submitting: apiSuccess({}),
};
