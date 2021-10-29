import { history } from 'index';
import { store } from 'store';
import { clearAuthTokens } from 'axios-jwt';

/// Redirect to log-in form before going back to this route
export const loginRedirect = () => {
  clearAuthTokens();

  store.globals.merge({ redirect: history.location.pathname, isLoggedIn: false });
  history.push('/login?redirect');
};

export const goHome = () => {
  history.push('/');
};
