/*
 * Global state shared by all screens
 *  (Things like permissions, errors, or login redirection)
 */
import { Permission } from 'models/auth';

export interface GlobalsState {
  isLoggedIn: boolean;
  name: string;
  email: string;
  permissions: Set<Permission>;
  globalError: Error | null;
  redirect: string | null;
}

export const initialGlobalsState: GlobalsState = {
  isLoggedIn: false,
  name: '',
  email: '',
  permissions: new Set(),
  globalError: null,
  redirect: null,
};
