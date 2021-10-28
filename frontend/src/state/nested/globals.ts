import { Permission } from 'models/auth';

export interface GlobalsState {
  permissions: Set<Permission>;
  globalError: Error | null;
  redirect: string | null;
}

export const INITIAL_GLOBALS_STATE: GlobalsState = {
  permissions: new Set(),
  globalError: null,
  redirect: null,
};
