/**
 * Global "Yes/No" confirmation dialog
 */
import { useState } from '@hookstate/core';
import { Confirm } from 'semantic-ui-react';
import { store } from 'store';

const hideDialog = () => {
  store.confirm.open.set(false);
};

const onConfirmAction = (action: () => void) => () => {
  hideDialog();
  action();
};

const onCancelAction = (action: () => void) => () => {
  hideDialog();
  action();
};

function testUndefined<T>(value?: T): T | undefined {
  return typeof value !== 'undefined' ? value : undefined;
}

export const ConfirmDialog = () => {
  const { open, message, header, confirmButton, cancelButton, onConfirm, onCancel, size } = useState(
    store.confirm,
  ).get();

  return (
    <Confirm
      open={open}
      content={testUndefined(message)}
      header={testUndefined(header)}
      confirmButton={testUndefined(confirmButton)}
      cancelButton={testUndefined(cancelButton)}
      onConfirm={onConfirm ? onConfirmAction(onConfirm) : hideDialog}
      onCancel={onCancel ? onCancelAction(onCancel) : hideDialog}
      size={size || undefined}
      style={{ whiteSpace: 'pre-wrap' }}
    />
  );
};
