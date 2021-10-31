/*
 * Login form state
 */
import { APIResult, apiSuccess } from 'api';

export interface RegisterState {
  name: string;
  email: string;
  password: string;
  confirm: string;
  modified: boolean;
  registrationError: APIResult<{}>;
}

export const initialRegisterState: RegisterState = {
  name: '',
  email: '',
  password: '',
  confirm: '',
  modified: false,
  registrationError: apiSuccess({}),
};
