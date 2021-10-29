/*
 * State used for global confirmation dialog
 */
export type ConfirmSizeType = 'mini' | 'tiny' | 'small' | 'large' | 'fullscreen';

export interface ConfirmState {
  open: boolean;
  message: string | null;
  header: string | null;
  confirmButton: string | null;
  cancelButton: string | null;
  onConfirm: (() => void) | null;
  onCancel: (() => void) | null;
  size: ConfirmSizeType | null;
}

export const initialConfirmState: ConfirmState = {
  open: false,
  message: 'Are you sure?',
  header: null,
  confirmButton: 'Ok',
  cancelButton: 'Cancel',
  onConfirm: null,
  onCancel: null,
  size: null,
};
