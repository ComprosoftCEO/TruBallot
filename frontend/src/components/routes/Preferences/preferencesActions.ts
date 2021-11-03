import { apiLoading, apiSuccess, axiosApi, resolveResult } from 'api';
import { setAuthTokens } from 'axios-jwt';
import { LoginResult } from 'models/auth';
import { useLayoutEffect } from 'react';
import { logInStore } from 'redux/auth';
import { getNestedState, mergeNestedState } from 'redux/helpers';
import { initialPreferencesState } from 'redux/state/preferences';

const mergeState = mergeNestedState('preferences');
const getState = getNestedState('preferences');
const getGlobalsState = getNestedState('globals');

export const useClearState = (): void => {
  useLayoutEffect(() => {
    const { name } = getGlobalsState();
    mergeState({ ...initialPreferencesState, newName: name });
  }, []);
};

//
// Setters
//
export const setNewName = (newName: string): void =>
  mergeState({ newName, updatingPreferences: apiSuccess(false), preferencesModified: true });

export const setCurrentPassword = (currentPassword: string): void =>
  mergeState({ currentPassword, updatingPassword: apiSuccess(false), passwordModified: true });

export const setNewPassword = (newPassword: string): void =>
  mergeState({ newPassword, updatingPassword: apiSuccess(false), passwordModified: true });

export const setConfirmPassword = (confirmPassword: string): void =>
  mergeState({ confirmPassword, updatingPassword: apiSuccess(false), passwordModified: true });

/**
 * Account preferences
 */
export const updatePreferences = async (): Promise<void> => {
  const { newName: name } = getState();

  mergeState({ updatingPreferences: apiLoading() });

  const result = await axiosApi.patch<LoginResult>('/account', { name }).then(...resolveResult);

  if (result.success) {
    // Update the tokens to match the user preferences
    setAuthTokens({ accessToken: result.data.clientToken, refreshToken: result.data.refreshToken });
    logInStore(result.data.clientToken);
    mergeState({ preferencesModified: false, updatingPreferences: apiSuccess(true) });
  } else {
    mergeState({ updatingPreferences: result });
  }
};

export const cancelUpdatePreferences = (): void => {
  const { name } = getGlobalsState();
  mergeState({ updatingPreferences: apiSuccess(false), newName: name, preferencesModified: false });
};

export const clearUpdatePreferencesSuccess = (): void => {
  mergeState({ updatingPreferences: apiSuccess(false) });
};

/**
 * Account Password
 */
export const updatePassword = async (): Promise<void> => {
  const { currentPassword, newPassword } = getState();

  mergeState({ updatingPassword: apiLoading() });

  const result = await axiosApi.put('/account/password', { currentPassword, newPassword }).then(...resolveResult);

  if (result.success) {
    mergeState({
      updatingPassword: apiSuccess(true),
      passwordModified: false,
      currentPassword: '',
      newPassword: '',
      confirmPassword: '',
    });
  } else {
    mergeState({ updatingPassword: result });
  }
};

export const cancelUpdatingPassword = (): void => {
  mergeState({
    updatingPassword: apiSuccess(false),
    passwordModified: false,
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
  });
};

export const clearUpdatePasswordSuccess = (): void => {
  mergeState({ updatingPassword: apiSuccess(false) });
};
