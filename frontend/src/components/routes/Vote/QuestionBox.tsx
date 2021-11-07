import { Segment, Header, Form, Radio, Divider, Checkbox, Popup, Message } from 'semantic-ui-react';
import { useState } from 'react';
import { setChoice, toggleChoice, useQuestion } from './voteActions';
import styles from './vote.module.scss';

export interface QuestionBoxProps {
  questionIndex: number;
  cheatMode?: boolean;
  disabled?: boolean;
}

export const QuestionBox = ({ questionIndex, cheatMode, disabled }: QuestionBoxProps): JSX.Element => {
  const [popupOpen, setPopupOpen] = useState(false);
  const question = useQuestion(questionIndex);
  const formDisabled = disabled || question.hasVoted;

  const formComponent = (
    <Segment raised padded textAlign="left" disabled={formDisabled}>
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
                disabled={formDisabled}
              />
            ) : (
              <Radio
                className={styles['radio-overflow']}
                label={candidate}
                checked={question.choices.has(i)}
                onChange={() => setChoice(questionIndex, i)}
                disabled={formDisabled}
              />
            )}
          </Form.Field>
        ))}
      </Form>
    </Segment>
  );

  // Add a popup if the user has already voted on the question
  if (question.hasVoted) {
    return (
      <Popup
        wide
        on="hover"
        position="right center"
        open={popupOpen && !disabled}
        onOpen={() => setPopupOpen(true)}
        onClose={() => setPopupOpen(false)}
        content={<Message compact icon="check square outline" content="Already voted for this question" />}
        trigger={formComponent}
      />
    );
  }

  return formComponent;
};
