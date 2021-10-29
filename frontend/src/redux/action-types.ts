import { Action } from 'redux';
import { RootState } from 'redux/state';

export const UPDATE_NESTED_STATE_ACTION = 'UPDATE_NESTED_STATE';
export const DYNAMIC_UPDATE_NESTED_STATE_ACTION = 'DYNAMIC_UPDATE_NESTED_STATE';
export const REPLACE_NESTED_STATE_ACTION = 'REPLACE_NESTED_STATE';
export const DYNAMIC_REPLACE_NESTED_STATE_ACTION = 'DYNAMIC_REPLACE_NESTED_STATE';

export interface UpdateNestedStateAction<NestedState extends keyof RootState> extends Action {
  type: typeof UPDATE_NESTED_STATE_ACTION;
  nestedState: NestedState;
  payload: Partial<RootState[NestedState]>;
}

export interface ReplaceNestedStateAction<NestedState extends keyof RootState> extends Action {
  type: typeof REPLACE_NESTED_STATE_ACTION;
  nestedState: NestedState;
  payload: RootState[NestedState];
}

export interface DynamicUpdateNestedStateAction<NestedState extends keyof RootState> extends Action {
  type: typeof DYNAMIC_UPDATE_NESTED_STATE_ACTION;
  nestedState: NestedState;
  action: (input: RootState[NestedState]) => Partial<RootState[NestedState]>;
}

export interface DynamicReplaceNestedStateAction<NestedState extends keyof RootState> extends Action {
  type: typeof DYNAMIC_REPLACE_NESTED_STATE_ACTION;
  nestedState: NestedState;
  action: (input: RootState[NestedState]) => RootState[NestedState];
}

export type UpdateStateAction =
  | UpdateNestedStateAction<keyof RootState>
  | ReplaceNestedStateAction<keyof RootState>
  | DynamicUpdateNestedStateAction<keyof RootState>
  | DynamicReplaceNestedStateAction<keyof RootState>;
