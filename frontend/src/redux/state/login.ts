/*
 * Login form state
 */
import { APIResult, apiSuccess } from 'api';

export interface LoginState {
  username: string;
  password: string;
  loginError: APIResult<{}>;
}

export const initialLoginState: LoginState = {
  username: '',
  password: '',
  loginError: apiSuccess({}),
};
