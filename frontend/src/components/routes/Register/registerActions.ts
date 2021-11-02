import { apiError, apiLoading, apiSuccess, resolveResult } from 'api';
import { useLayoutEffect } from 'react';
import { clearNestedState, mergeNestedState } from 'redux/helpers';
import { history } from 'index';
import { ReCAPTCHA } from 'react-google-recaptcha';
import { API_BASE_URL } from 'env';
import { ClientToken, LoginResult } from 'models/auth';
import axios from 'axios';
import jwt from 'jsonwebtoken';
import { setAuthTokens } from 'axios-jwt';

const mergeGlobalsState = mergeNestedState('globals');
const mergeRegisterState = mergeNestedState('register');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('register'), []);
};

export const setName = (name: string): void =>
  mergeRegisterState({ name, modified: true, registrationError: apiSuccess({}) });

export const setEmail = (email: string): void =>
  mergeRegisterState({ email, modified: true, registrationError: apiSuccess({}) });

export const setPassword = (password: string): void =>
  mergeRegisterState({ password, modified: true, registrationError: apiSuccess({}) });

export const setConfirm = (confirm: string): void =>
  mergeRegisterState({ confirm, modified: true, registrationError: apiSuccess({}) });

export const goBack = () => {
  history.push('/');
};

export const registerUser = async (recaptcha: ReCAPTCHA, name: string, email: string, password: string) => {
  // Make the reCAPTCHA request, and handle any errors
  const captcha = recaptcha.getValue();
  if (captcha === null) {
    mergeRegisterState({ registrationError: apiError(new Error('reCAPTCHA token expired, please try again')) });
    recaptcha.reset();
    return;
  }

  // Attempt to register with the API server
  mergeRegisterState({ registrationError: apiLoading() });
  const result = await axios
    .post<LoginResult>(`${API_BASE_URL}/account/register`, { name, email, password, captcha })
    .then(...resolveResult);

  // Handle response
  if (result.success) {
    handleLogin(result.data);
  } else {
    mergeRegisterState({ registrationError: result });
    recaptcha.reset();
  }
};

export const handleRecaptchaError = (recaptcha: ReCAPTCHA): void => {
  mergeRegisterState({ registrationError: apiError(new Error('reCAPTCHA error, please try again')) });
  recaptcha.reset();
};

/**
 * Log in the user and update the JWT storage
 *
 * @param loginResult JWT tokens from the server
 */
function handleLogin(loginResult: LoginResult) {
  // Clear the username and password fields first
  mergeRegisterState({ name: '', email: '', password: '' });

  // Store the tokens in the system
  setAuthTokens({
    accessToken: loginResult.clientToken,
    refreshToken: loginResult.refreshToken,
  });

  // Indiate that the user has logged in
  const clientToken: ClientToken = jwt.decode(loginResult.clientToken) as ClientToken;
  mergeGlobalsState({
    isLoggedIn: true,
    name: clientToken.name,
    email: clientToken.email,
    permissions: new Set(clientToken.permissions),
  });

  // Go to the dashboard
  history.push('/');
}
