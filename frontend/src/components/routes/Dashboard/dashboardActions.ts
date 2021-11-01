import { clearAuthTokens } from 'axios-jwt';
import { clearNestedState, mergeNestedState } from 'redux/helpers';
import { history } from 'index';
import { useLayoutEffect } from 'react';

const mergeGlobalsState = mergeNestedState('globals');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('dashboard'), []);
};

export const logOut = () => {
  clearAuthTokens();
  mergeGlobalsState({ isLoggedIn: false });
  history.push('/');
};
