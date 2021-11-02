import { useLayoutEffect } from 'react';
import ReCAPTCHA from 'react-google-recaptcha';
import axios from 'axios';
import { setAuthTokens } from 'axios-jwt';
import * as EmailValidator from 'email-validator';
import { API_BASE_URL } from 'env';
import { apiError, apiLoading, apiSuccess, resolveResult } from 'api';
import { LoginResult } from 'models/auth';
import { history } from 'index';
import { clearNestedState, getNestedState, mergeNestedState } from 'redux/helpers';
import { logInStore } from 'redux/auth';

const getGlobalsState = getNestedState('globals');
const mergeLoginState = mergeNestedState('login');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('login'), []);
};

export const setEmail = (email: string): void => mergeLoginState({ email, loginError: apiSuccess({}) });

export const setPassword = (password: string): void => mergeLoginState({ password, loginError: apiSuccess({}) });

export const isFormValid = (email: string, password: string): boolean =>
  email.length > 0 && password.length > 0 && EmailValidator.validate(email);

/**
 * Make the request to log the user into the system
 *
 * @param recaptcha Google reCAPTCHA component instance
 * @param email User email
 * @param password user passowrd
 */
export const logInUser = async (recaptcha: ReCAPTCHA, email: string, password: string): Promise<void> => {
  // Get the reCAPTCHA token, and handle any errors
  const captcha = recaptcha.getValue();
  if (captcha === null) {
    mergeLoginState({ password: '', loginError: apiError(new Error('reCAPTCHA token expired, please try again')) });
    recaptcha.reset();
    return;
  }

  // Attempt to log in with the API server
  mergeLoginState({ loginError: apiLoading() });
  const result = await axios
    .post<LoginResult>(`${API_BASE_URL}/auth/login`, { email, password, captcha })
    .then(...resolveResult);

  // Handle response
  if (result.success) {
    handleLogin(result.data);
  } else {
    mergeLoginState({ password: '', loginError: result });
    recaptcha.reset();
  }
};

/**
 * Handle the reCAPTCHA error in the store
 * @param recaptcha Google reCAPTCHA component instance
 */
export const handleRecaptchaError = (recaptcha: ReCAPTCHA): void => {
  mergeLoginState({ password: '', loginError: apiError(new Error('reCAPTCHA error, please try again')) });
  recaptcha.reset();
};

/**
 * Log in the user in the store and update the JWT storage
 * @param loginResult JWT tokens from the server
 */
function handleLogin(loginResult: LoginResult) {
  // Clear the username and password fields first
  mergeLoginState({ email: '', password: '' });

  // Store the tokens in the system
  setAuthTokens({
    accessToken: loginResult.clientToken,
    refreshToken: loginResult.refreshToken,
  });

  // Should the login redirect somewhere else?
  //   Fetch this before updating the location after the user logs in
  const { redirect } = getGlobalsState();
  const redirectQuery = new URLSearchParams(history.location.search).get('redirect');

  // Indiate that the user has logged in
  logInStore(loginResult.clientToken);

  // Go to the dashboard or redirect somewhere else
  if (redirectQuery !== null && redirect !== null) {
    // Go to the redirect URL
    history.push(redirect);
  } else {
    // Go to the dashboard
    history.push('/');
  }
}
