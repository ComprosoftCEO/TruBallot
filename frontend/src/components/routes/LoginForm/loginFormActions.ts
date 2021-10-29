import ReCAPTCHA from 'react-google-recaptcha';
import * as EmailValidator from 'email-validator';
import { isLoggedIn, setAuthTokens, clearAuthTokens, getAccessToken, getRefreshToken } from 'axios-jwt';
import axios from 'axios';
import { API_BASE_URL } from 'env';
import { resolveResult } from 'api';
import { LoginResult } from 'models/auth';
import { history } from 'index';
import { store } from 'store';

export const logInUser = async (recaptcha: ReCAPTCHA, username: string, password: string) => {
  const captcha = await recaptcha.executeAsync();
  const result = await axios
    .post<LoginResult>(`${API_BASE_URL}/auth`, { username, password, captcha })
    .then(...resolveResult);

  if (result.success) {
    handleLogin(result.data);
  } else {
    console.log(result);
    store.login.merge({ password: '', loginError: result });
  }
};

function handleLogin(loginResult: LoginResult) {
  // Store the tokens in the system
  setAuthTokens({
    accessToken: loginResult.clientToken,
    refreshToken: loginResult.refreshToken,
  });

  // Should the login redirect somewhere else?
  const redirect = store.globals.redirect.get();
  const redirectQuery = new URLSearchParams(history.location.search).get('redirect');
  if (redirectQuery !== null && redirect !== null) {
    // Go to the redirect URL
    history.push(redirect);
  } else {
    // Go to the dashboard
    history.push('/');
  }
}

export const isFormValid = (email: string, password: string): boolean =>
  email.length > 0 && password.length > 0 && EmailValidator.validate(email);
