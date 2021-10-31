/*
 * Login form state
 */
import { APIResult, apiSuccess } from 'api';

export interface LoginState {
  email: string;
  password: string;
  loginError: APIResult<{}>;
}

export const initialLoginState: LoginState = {
  email: '',
  password: '',
  loginError: apiSuccess({}),
};
