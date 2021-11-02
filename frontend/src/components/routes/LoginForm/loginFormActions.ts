import { useLayoutEffect } from 'react';
import ReCAPTCHA from 'react-google-recaptcha';
import axios from 'axios';
import jwt from 'jsonwebtoken';
import { setAuthTokens } from 'axios-jwt';
import * as EmailValidator from 'email-validator';
import { API_BASE_URL } from 'env';
import { apiError, apiLoading, apiSuccess, resolveResult } from 'api';
import { ClientToken, LoginResult } from 'models/auth';
import { history } from 'index';
import { clearNestedState, getNestedState, mergeNestedState } from 'redux/helpers';

const getGlobalsState = getNestedState('globals');
const mergeGlobalsState = mergeNestedState('globals');
const mergeLoginState = mergeNestedState('login');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('login'), []);
};

export const setEmail = (email: string): void => mergeLoginState({ email, loginError: apiSuccess({}) });

export const setPassword = (password: string): void => mergeLoginState({ password, loginError: apiSuccess({}) });

export const isFormValid = (email: string, password: string): boolean =>
  email.length > 0 && password.length > 0 && EmailValidator.validate(email);

export const logInUser = async (recaptcha: ReCAPTCHA, email: string, password: string) => {
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

export const handleRecaptchaError = (recaptcha: ReCAPTCHA): void => {
  mergeLoginState({ password: '', loginError: apiError(new Error('reCAPTCHA error, please try again')) });
  recaptcha.reset();
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

  // Should the login redirect somewhere else?
  //   Fetch this before updating the location after the user logs in
  const { redirect } = getGlobalsState();
  const redirectQuery = new URLSearchParams(history.location.search).get('redirect');

  // Indiate that the user has logged in
  const clientToken: ClientToken = jwt.decode(loginResult.clientToken) as ClientToken;
  mergeGlobalsState({
    isLoggedIn: true,
    name: clientToken.name,
    email: clientToken.email,
    permissions: new Set(clientToken.permissions),
  });

  // Go to the dashboard or redirect somewhere else
  if (redirectQuery !== null && redirect !== null) {
    // Go to the redirect URL
    history.push(redirect);
  } else {
    // Go to the dashboard
    history.push('/');
  }
}
