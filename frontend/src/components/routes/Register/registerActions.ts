import { apiError, apiSuccess, resolveResult } from 'api';
import { useLayoutEffect } from 'react';
import { clearNestedState, getNestedState, mergeNestedState } from 'redux/helpers';
import { showConfirm } from 'showConfirm';
import { history } from 'index';
import { ReCAPTCHA } from 'react-google-recaptcha';
import { API_BASE_URL } from 'env';
import { LoginResult } from 'models/auth';
import axios from 'axios';
import { setAuthTokens } from 'axios-jwt';

const getRegisterState = getNestedState('register');
const mergeGlobalsState = mergeNestedState('globals');
const mergeRegisterState = mergeNestedState('register');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('register'), []);
};

export const recaptchaCanceled = () => {
  mergeRegisterState({ registrationError: apiSuccess({}) });
};

export const setName = (name: string): void => mergeRegisterState({ name, modified: true });
export const setEmail = (email: string): void => mergeRegisterState({ email, modified: true });
export const setPassword = (password: string): void => mergeRegisterState({ password, modified: true });
export const setConfirm = (confirm: string): void => mergeRegisterState({ confirm, modified: true });

export const goBack = () => {
  const { modified } = getRegisterState();

  showConfirm({
    message: 'Discard changes?',
    override: !modified || undefined,
    onConfirm: () => {
      history.push('/');
    },
  });
};

export const registerUser = async (recaptcha: ReCAPTCHA, name: string, email: string, password: string) => {
  // Make the reCAPTCHA request, and handle any errors
  let captcha: string | null = null;
  try {
    captcha = await recaptcha.executeAsync();
    if (captcha === null) {
      mergeRegisterState({ registrationError: apiError(new Error('reCAPTCHA Token Expired')) });
      recaptcha.reset();
    }
  } catch (e) {
    mergeRegisterState({ registrationError: apiError(e as Error) });
    recaptcha.reset();
  }

  // Attempt to log in with the API server
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
  mergeGlobalsState({ isLoggedIn: true });

  // Go to the dashboard
  history.push('/');
}
