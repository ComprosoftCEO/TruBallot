import { TextArea, TextAreaProps } from 'semantic-ui-react';

type OmitTextAreaProps = Omit<TextAreaProps, 'value' | 'minLength' | 'maxLength' | 'disabled' | 'readOnly'>;

export interface TextAreaInputProps extends OmitTextAreaProps {
  value?: string;
  maxLength?: number;
  onChangeValue?: (newValue: string) => void;
  disabled?: boolean;
  readOnly?: boolean;
}

const onChangeEvent =
  (onChange: (newValue: string) => void) =>
  (event: React.FormEvent<HTMLTextAreaElement>, data: TextAreaProps): void => {
    if (typeof data.value !== 'undefined') {
      onChange(data.value.toString());
    }
  };

export const TextAreaInput = ({
  value = '',
  maxLength = 0,
  onChangeValue,
  disabled = false,
  readOnly,
  ...rest
}: TextAreaInputProps): JSX.Element => (
  <TextArea
    minLength={0}
    maxLength={maxLength > 0 ? maxLength : undefined}
    value={value}
    onChange={onChangeValue && onChangeEvent(onChangeValue)}
    disabled={disabled}
    readOnly={readOnly}
    // eslint-disable-next-line react/jsx-props-no-spreading
    {...rest}
  />
);
