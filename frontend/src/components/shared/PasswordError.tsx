import React from 'react';
import { ZXCVBNResult } from 'zxcvbn';
import { Message, MessageProps, List } from 'semantic-ui-react';

export interface PasswordErrorProps extends Omit<MessageProps, 'negative' | 'content'> {
  passwordsMatch: boolean;
  passwordStrength: ZXCVBNResult;
}

export const PasswordError = ({ passwordsMatch, passwordStrength, style, ...rest }: PasswordErrorProps) => {
  const feedback = passwordStrength.feedback.warning || 'Password has the following errors:';

  const suggestions = passwordsMatch
    ? [...passwordStrength.feedback.suggestions]
    : ['Passwords do not match', ...passwordStrength.feedback.suggestions];

  return (
    // eslint-disable-next-line react/jsx-props-no-spreading
    <Message negative style={{ textAlign: 'left', ...style }} {...rest}>
      <p>
        <b>Warning: </b>
        {feedback}
      </p>
      {suggestions.length > 0 && (
        <List as="ul">
          {suggestions.map((suggestion) => (
            <List.Item as="li" key={suggestion}>
              {suggestion}
            </List.Item>
          ))}
        </List>
      )}
    </Message>
  );
};
