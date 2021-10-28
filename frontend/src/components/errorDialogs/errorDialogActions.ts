import { history } from 'index';
import { store } from 'store';

/// Redirect to log-in form before going back to this route
export const loginRedirect = () => {
  store.globals.redirect.set(history.location.pathname);
  history.push('/login?redirect');
};
