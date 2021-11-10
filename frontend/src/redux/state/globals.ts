/*
 * Global state shared by all screens
 *  (Things like permissions, errors, or login redirection)
 */
import { Permission } from 'models/auth';

export interface GlobalsState {
  isLoggedIn: boolean;

  userId: string;
  name: string;
  email: string;
  permissions: Set<Permission>;
  accessToken: string | null;

  globalError: Error | null;
  redirect: string | null;
}

export const initialGlobalsState: GlobalsState = {
  isLoggedIn: false,

  userId: '',
  name: '',
  email: '',
  permissions: new Set(),
  accessToken: null,

  globalError: null,
  redirect: null,
};
