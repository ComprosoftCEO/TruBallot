/*
 * State used for creating and editing the elections
 */
import { apiLoading, APIResult } from 'api/types';
import { PublicElectionDetails } from 'models/election';

export interface EditorState {
  electionDetails: APIResult<PublicElectionDetails>;

  // Values used by the editor
  name: string;
  isPublic: boolean;
  questions: string;
}

export const initialEditorState: EditorState = {
  electionDetails: apiLoading(),
  name: 'New Election',
  isPublic: false,
  questions: '',
};
