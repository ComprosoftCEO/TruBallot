/**
 * The root state holds all of the nested states
 */
import { GlobalsState, initialGlobalsState } from './globals';
import { ConfirmState, initialConfirmState } from './confirm';
import { LoginState, initialLoginState } from './login';
import { RegisterState, initialRegisterState } from './register';
import { DashboardState, initialDashboardState } from './dashboard';
import { EditorState, initialEditorState } from './editor';
import { PreferencesState, initialPreferencesState } from './preferences';
import { AccessCodeState, initialAccessCodeState } from './accessCode';
import { ManageElectionState, initialManageElectionState } from './manageElection';
import { VoteState, initialVoteState } from './vote';

export interface RootState {
  globals: GlobalsState;
  confirm: ConfirmState;
  login: LoginState;
  register: RegisterState;
  dashboard: DashboardState;
  editor: EditorState;
  preferences: PreferencesState;
  accessCode: AccessCodeState;
  manageElection: ManageElectionState;
  vote: VoteState;
}

export const initialState: RootState = {
  globals: initialGlobalsState,
  confirm: initialConfirmState,
  login: initialLoginState,
  register: initialRegisterState,
  dashboard: initialDashboardState,
  editor: initialEditorState,
  preferences: initialPreferencesState,
  accessCode: initialAccessCodeState,
  manageElection: initialManageElectionState,
  vote: initialVoteState,
};
