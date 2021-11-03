import { Message, List } from 'semantic-ui-react';

export interface QuestionErrorsProps {
  errors: string[];
}

export const QuestionErrors = ({ errors }: QuestionErrorsProps) => (
  <Message negative>
    <List as="ul">
      {errors.map((error) => (
        <List.Item as="li" key={error}>
          {error}
        </List.Item>
      ))}
    </List>
  </Message>
);
