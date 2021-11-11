import { getAccessToken } from 'axios-jwt';
import jwt from 'jsonwebtoken';
import { ClientToken, Permission } from 'models/auth';
import { getNestedState, mergeNestedState, nestedSelectorHook } from './helpers';

const mergeState = mergeNestedState('globals');
const useSelector = nestedSelectorHook('globals');
const getState = getNestedState('globals');

/**
 * Mark the store as being logged in from the access token
 *
 * Note: We are also allowed to pass a refresh token, since it
 *   contains the SAME FIELDS as the access token
 *
 * @param token Access token (or refresh token) JWT string
 */
export const logInStore = (token: string, isAccessToken = true): void => {
  // If we are passing in the refresh token, try to load the access token
  //  If the access token is INDEED null, then set it to null
  let accessToken = isAccessToken ? token : null;
  if (!isAccessToken) {
    const currentToken = getAccessToken();
    if (currentToken !== undefined) {
      accessToken = currentToken;
    }
  }

  const clientToken: ClientToken = jwt.decode(token) as ClientToken;
  mergeState({
    isLoggedIn: true,
    userId: clientToken.sub,
    name: clientToken.name,
    email: clientToken.email,
    permissions: new Set(clientToken.permissions),
    accessToken,
  });
};

/**
 * Mark the store as being logged out
 *
 * @param redirect If set, updates the redirect path for the login form
 */
export const logOutStore = (redirect?: string | null): void => {
  if (redirect !== undefined) {
    mergeState({ isLoggedIn: false, accessToken: null, redirect });
  } else {
    mergeState({ isLoggedIn: false, accessToken: null });
  }
};

/// Hook to get the user ID
export const useUserId = (): string => useSelector((state) => state.userId);

/// Get the user ID from the state
export const getUserId = (): string => getState().userId;

/// Hook to get the current user name
export const useUserName = (): string => useSelector((state) => state.name);

/// Hook to get the current user email
export const useUserEmail = (): string => useSelector((state) => state.email);

/// Hook to get user permissions
export const usePermissions = (): Set<Permission> => useSelector((state) => state.permissions);

/// Hook to get the current access token
export const useAccessToken = (): string | null => useSelector((state) => state.accessToken);
