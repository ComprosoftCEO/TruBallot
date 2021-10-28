import { Input, InputProps } from 'semantic-ui-react';

type OmitInputProps = Omit<
  InputProps,
  'type' | 'value' | 'minLength' | 'maxLength' | 'disabled' | 'readOnly' | 'password'
>;

export interface StringInputProps extends OmitInputProps {
  value?: string;
  maxLength?: number;
  onChangeValue?: (newValue: string) => void;
  disabled?: boolean;
  readOnly?: boolean;
  password?: boolean;
}

const onChangeEvent =
  (onChange: (newValue: string) => void) =>
  (event: React.ChangeEvent<HTMLInputElement>): void =>
    onChange(event.target.value);

export const StringInput = ({
  value = '',
  maxLength = 0,
  onChangeValue,
  disabled = false,
  password,
  readOnly,
  ...rest
}: StringInputProps): JSX.Element => (
  <Input
    type={password ? 'password' : 'text'}
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
