import {
  UPDATE_NESTED_STATE_ACTION,
  REPLACE_NESTED_STATE_ACTION,
  DYNAMIC_UPDATE_NESTED_STATE_ACTION,
  DYNAMIC_REPLACE_NESTED_STATE_ACTION,
  UpdateStateAction,
} from 'redux/action-types';
import { RootState, initialState } from 'redux/state';

type StateReducer = (state: RootState, action: UpdateStateAction) => RootState;

const updateNestedStateReducer = (state: RootState, action: UpdateStateAction): RootState => {
  if (action.type !== UPDATE_NESTED_STATE_ACTION) {
    return state;
  }

  return { ...state, [action.nestedState]: { ...state[action.nestedState], ...action.payload } };
};

const replaceNestedStateReducer = (state: RootState, action: UpdateStateAction): RootState => {
  if (action.type !== REPLACE_NESTED_STATE_ACTION) {
    return state;
  }

  return { ...state, [action.nestedState]: { ...action.payload } };
};

const dynamicUpdateNestedStateReducer = (state: RootState, action: UpdateStateAction): RootState => {
  if (action.type !== DYNAMIC_UPDATE_NESTED_STATE_ACTION) {
    return state;
  }

  return {
    ...state,
    [action.nestedState]: { ...state[action.nestedState], ...action.action(state[action.nestedState]) },
  };
};

const dynamicReplaceNestedStateReducer = (state: RootState, action: UpdateStateAction): RootState => {
  if (action.type !== DYNAMIC_REPLACE_NESTED_STATE_ACTION) {
    return state;
  }

  return { ...state, [action.nestedState]: { ...action.action(state[action.nestedState]) } };
};

// Lookup table of reducers
const reducers: { [key: string]: StateReducer } = {
  [UPDATE_NESTED_STATE_ACTION]: updateNestedStateReducer,
  [REPLACE_NESTED_STATE_ACTION]: replaceNestedStateReducer,
  [DYNAMIC_UPDATE_NESTED_STATE_ACTION]: dynamicUpdateNestedStateReducer,
  [DYNAMIC_REPLACE_NESTED_STATE_ACTION]: dynamicReplaceNestedStateReducer,
};

export const rootReducers = (state: RootState = initialState, action: UpdateStateAction) =>
  reducers[action.type] ? reducers[action.type](state, action) : state;
