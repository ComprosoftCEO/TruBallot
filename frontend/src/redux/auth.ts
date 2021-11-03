import jwt from 'jsonwebtoken';
import { ClientToken } from 'models/auth';
import { mergeNestedState } from './helpers';

const mergeState = mergeNestedState('globals');

/**
 * Mark the store as being logged in from the access token
 *
 * Note: We are also allowed to pass a refresh token, since it
 *   contains the SAME FIELDS as the access token
 *
 * @param accessToken Access token (or refresh token) JWT string
 */
export const logInStore = (accessToken: string): void => {
  const clientToken: ClientToken = jwt.decode(accessToken) as ClientToken;
  mergeState({
    isLoggedIn: true,
    userId: clientToken.sub,
    name: clientToken.name,
    email: clientToken.email,
    permissions: new Set(clientToken.permissions),
  });
};

/**
 * Mark the store as being logged out
 *
 * @param redirect If set, updates the redirect path for the login form
 */
export const logOutStore = (redirect?: string | null): void => {
  if (redirect !== undefined) {
    mergeState({ isLoggedIn: false, redirect });
  } else {
    mergeState({ isLoggedIn: false });
  }
};
