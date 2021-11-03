/**
 * State for updating user preferences
 */
import { APIResult, apiSuccess } from 'api';

export interface PreferencesState {
  newName: string;
  preferencesModified: boolean;
  updatingPreferences: APIResult<boolean>;

  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
  passwordModified: boolean;
  updatingPassword: APIResult<boolean>;
}

export const initialPreferencesState: PreferencesState = {
  newName: '',
  preferencesModified: false,
  updatingPreferences: apiSuccess(false),

  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
  passwordModified: false,
  updatingPassword: apiSuccess(false),
};
