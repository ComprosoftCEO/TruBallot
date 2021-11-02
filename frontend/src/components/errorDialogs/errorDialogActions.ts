import { history } from 'index';
import { clearAuthTokens } from 'axios-jwt';
import { logOutStore } from 'redux/auth';

/// Redirect to log-in form before going back to this route
export const loginRedirect = () => {
  clearAuthTokens();

  logOutStore(history.location.pathname);
  history.push('/login?redirect');
};

export const goHomeLogin = () => {
  clearAuthTokens();

  logOutStore();
  window.location.href = '/';
};
