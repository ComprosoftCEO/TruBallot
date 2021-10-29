import { createStore, applyMiddleware } from 'redux';
import { logger } from 'redux-logger';
import { isDev } from 'env';

import { initialState } from './state';
import { rootReducers } from './reducers';

//
// Configure middleware
//
const middleware = [];
if (isDev()) {
  middleware.push(logger);
}

//
// All application state is stored in the global "store"
//
export const store = createStore(rootReducers, initialState, applyMiddleware(...middleware));

//
// Expose the store when on the development environment
//
if (isDev()) {
  Object.assign(window, { store });
}
