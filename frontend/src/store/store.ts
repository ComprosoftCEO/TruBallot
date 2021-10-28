import { createState, DevTools } from '@hookstate/core';
import { DevToolsInitialize } from '@hookstate/devtools';
import { isDev } from 'env';

// List of all nested states
import { GlobalsState, initialGlobalsState } from './nested/globals';
import { ConfirmState, initialConfirmState } from './nested/confirm';

export interface State {
  globals: GlobalsState;
  confirm: ConfirmState;
}

export const initialState: State = {
  globals: initialGlobalsState,
  confirm: initialConfirmState,
};

//
// All application state is stored in the global "store"
//
export const store = createState<State>(initialState);

//
// Expose the state when on the development environment
//
if (isDev()) {
  DevTools(store).label('State');
  DevToolsInitialize({
    monitored: ['State'],
    callstacksDepth: 30,
  });

  Object.assign(window, { store });
}
