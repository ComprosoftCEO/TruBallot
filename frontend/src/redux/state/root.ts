/**
 * The root state holds all of the nested states
 */
import { GlobalsState, initialGlobalsState } from './globals';
import { ConfirmState, initialConfirmState } from './confirm';
import { LoginState, initialLoginState } from './login';
import { RegisterState, initialRegisterState } from './register';
import { DashboardState, initialDashboardState } from './dashboard';
import { EditorState, initialEditorState } from './editor';

export interface RootState {
  globals: GlobalsState;
  confirm: ConfirmState;
  login: LoginState;
  register: RegisterState;
  dashboard: DashboardState;
  editor: EditorState;
}

export const initialState: RootState = {
  globals: initialGlobalsState,
  confirm: initialConfirmState,
  login: initialLoginState,
  register: initialRegisterState,
  dashboard: initialDashboardState,
  editor: initialEditorState,
};
