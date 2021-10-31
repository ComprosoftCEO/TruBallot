import { useLayoutEffect } from 'react';
import ReCAPTCHA from 'react-google-recaptcha';
import axios from 'axios';
import { setAuthTokens } from 'axios-jwt';
import * as EmailValidator from 'email-validator';
import { API_BASE_URL } from 'env';
import { apiError, apiSuccess, resolveResult } from 'api';
import { LoginResult } from 'models/auth';
import { history } from 'index';
import { clearNestedState, getNestedState, mergeNestedState } from 'redux/helpers';

const getGlobalsState = getNestedState('globals');
const mergeGlobalsStore = mergeNestedState('globals');
const mergeLoginState = mergeNestedState('login');

export const useClearState = () => {
  useLayoutEffect(() => {
    clearNestedState('login');
  }, []);
};

export const isFormValid = (email: string, password: string): boolean =>
  email.length > 0 && password.length > 0 && EmailValidator.validate(email);

export const logInUser = async (recaptcha: ReCAPTCHA, email: string, password: string) => {
  // Make the reCAPTCHA request, and handle any errors
  let captcha: string | null = null;
  try {
    captcha = await recaptcha.executeAsync();
    if (captcha === null) {
      mergeLoginState({ password: '', loginError: apiError(new Error('reCAPTCHA Token Expired')) });
      recaptcha.reset();
    }
  } catch (e) {
    mergeLoginState({ password: '', loginError: apiError(e as Error) });
    recaptcha.reset();
  }

  // Attempt to log in with the API server
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

export const recaptchaCanceled = () => {
  mergeLoginState({ loginError: apiSuccess({}) });
};

/**
 * Log in the user and update the JWT storage
 *
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

  // Indiate that the user has logged in
  mergeGlobalsStore({ isLoggedIn: true });

  // Should the login redirect somewhere else?
  const { redirect } = getGlobalsState();
  const redirectQuery = new URLSearchParams(history.location.search).get('redirect');
  if (redirectQuery !== null && redirect !== null) {
    // Go to the redirect URL
    history.push(redirect);
  } else {
    // Go to the dashboard
    history.push('/');
  }
}
