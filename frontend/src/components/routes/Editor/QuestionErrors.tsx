import { Message, List } from 'semantic-ui-react';

export interface QuestionErrorsProps {
  errors: string[];
}

export const QuestionErrors = ({ errors }: QuestionErrorsProps) => (
  <Message negative>
    <Message.Header>Question Errors:</Message.Header>
    <List as="ul" style={{ marginTop: 8 }}>
      {errors.map((error) => (
        <List.Item as="li" key={error}>
          {error}
        </List.Item>
      ))}
    </List>
  </Message>
);
