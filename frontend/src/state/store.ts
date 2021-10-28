import { createState, DevTools } from '@hookstate/core';
import { DevToolsInitialize } from '@hookstate/devtools';
import { isDev } from 'env';

// List of all nested states
import { GlobalsState, INITIAL_GLOBALS_STATE } from './nested/globals';

export interface State {
  globals: GlobalsState;
}

export const INITIAL_STATE: State = {
  globals: INITIAL_GLOBALS_STATE,
};

export const store = createState<State>(INITIAL_STATE);

// Monitor the state on development environment
if (isDev()) {
  DevTools(store).label('State');
  DevToolsInitialize({
    monitored: ['State'],
    callstacksDepth: 30,
  });
}
