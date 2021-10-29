import { RootState } from 'redux/state';
import {
  UPDATE_NESTED_STATE_ACTION,
  DYNAMIC_UPDATE_NESTED_STATE_ACTION,
  REPLACE_NESTED_STATE_ACTION,
  DYNAMIC_REPLACE_NESTED_STATE_ACTION,
  UpdateNestedStateAction,
  ReplaceNestedStateAction,
  DynamicUpdateNestedStateAction,
  DynamicReplaceNestedStateAction,
} from './action-types';

/**
 * Helpful constructor functions
 */
export const updateNestedState = <NestedState extends keyof RootState>(
  nestedState: NestedState,
  payload: Partial<RootState[NestedState]>,
): UpdateNestedStateAction<NestedState> => ({ type: UPDATE_NESTED_STATE_ACTION, nestedState, payload });

export const dynamicUpdateNestedState = <NestedState extends keyof RootState>(
  nestedState: NestedState,
  action: (input: RootState[NestedState]) => Partial<RootState[NestedState]>,
): DynamicUpdateNestedStateAction<NestedState> => ({ type: DYNAMIC_UPDATE_NESTED_STATE_ACTION, nestedState, action });

export const replaceNestedState = <NestedState extends keyof RootState>(
  nestedState: NestedState,
  payload: RootState[NestedState],
): ReplaceNestedStateAction<NestedState> => ({ type: REPLACE_NESTED_STATE_ACTION, nestedState, payload });

export const dynamicReplaceNestedState = <NestedState extends keyof RootState>(
  nestedState: NestedState,
  action: (input: RootState[NestedState]) => RootState[NestedState],
): DynamicReplaceNestedStateAction<NestedState> => ({ type: DYNAMIC_REPLACE_NESTED_STATE_ACTION, nestedState, action });
