import { ConfirmState, initialConfirmState } from 'redux/state';
import { mergeNestedState } from 'redux/helpers';

const mergeState = mergeNestedState('confirm');

export type ConfirmProperties = {
  [P in keyof Omit<ConfirmState, 'open'>]?: ConfirmState[P];
} & {
  override?: boolean; // If true, call "Ok", if false call "Cancel"
};

export const showConfirm = ({ override, ...props }: ConfirmProperties) => {
  if (typeof override !== 'undefined') {
    // Override showing the dialog and shortcut the action
    const action = override === true ? props.onConfirm : props.onCancel;
    if (action) {
      action();
    }
  } else {
    mergeState({ ...initialConfirmState, ...props, open: true });
  }
};
