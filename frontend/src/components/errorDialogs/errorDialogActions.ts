import { history } from 'index';
import { clearAuthTokens } from 'axios-jwt';
import { mergeNestedState } from 'redux/helpers';

const mergeGlobalsState = mergeNestedState('globals');

/// Redirect to log-in form before going back to this route
export const loginRedirect = () => {
  clearAuthTokens();

  mergeGlobalsState({ redirect: history.location.pathname, isLoggedIn: false });
  history.push('/login?redirect');
};

export const goHomeLogin = () => {
  mergeGlobalsState({ isLoggedIn: false });
  history.push('/');
};

export const goHome = () => {
  history.push('/');
};
