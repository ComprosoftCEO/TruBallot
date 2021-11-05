import { Segment, Header, Form, Radio, Divider, Checkbox } from 'semantic-ui-react';
import { setChoice, toggleChoice, useQuestion } from './voteActions';
import styles from './vote.module.scss';

export interface QuestionBoxProps {
  questionIndex: number;
  cheatMode?: boolean;
  disabled?: boolean;
}

export const QuestionBox = ({ questionIndex, cheatMode, disabled }: QuestionBoxProps) => {
  const question = useQuestion(questionIndex);

  return (
    <Segment raised padded textAlign="left">
      <Header textAlign="center" className={styles['header-overflow']}>
        {`${questionIndex + 1}. ${question.name}`}
      </Header>

      <Divider />

      <Form>
        {question.candidates.map((candidate, i) => (
          // This is intended behavior, as the cnadidate order will NOT change
          // eslint-disable-next-line react/no-array-index-key
          <Form.Field key={`${i}-${candidate}`}>
            {cheatMode ? (
              <Checkbox
                className={styles['radio-overflow']}
                label={candidate}
                checked={question.choices.has(i)}
                onChange={() => toggleChoice(questionIndex, i)}
                disabled={disabled}
              />
            ) : (
              <Radio
                className={styles['radio-overflow']}
                label={candidate}
                checked={question.choices.has(i)}
                onChange={() => setChoice(questionIndex, i)}
                disabled={disabled}
              />
            )}
          </Form.Field>
        ))}
      </Form>
    </Segment>
  );
};
