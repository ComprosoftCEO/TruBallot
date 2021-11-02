/**
 * Actual field inputs that are used to create or edit an election
 *
 * This returns a fragment and should be wrapped inside a <Form> element
 */
import { Fragment, useState } from 'react';
import { Message, Form, Checkbox, Grid, List, Popup } from 'semantic-ui-react';
import { StringInput, TextAreaInput } from 'components/input';
import { nestedSelectorHook } from 'redux/helpers';
import { QuestionErrors } from './QuestionErrors';
import {
  PLACEHOLDER_TEXT,
  parseListString,
  setQuestions,
  setName,
  toggleIsPublic,
  validateQuestionList,
} from './editorActions';
import styles from './editor.module.scss';

const useSelector = nestedSelectorHook('editor');

export interface EditorProps {
  disabled?: boolean;
}

export const Editor = ({ disabled }: EditorProps) => {
  // A bit of local state to manage the popups
  const [publicHover, setPublicHover] = useState(false);
  const [questionsHover, setQuestionsHover] = useState(false);

  const name = useSelector((state) => state.name);
  const isPublic = useSelector((state) => state.isPublic);
  const questions = useSelector((state) => state.questions);

  const questionItems = parseListString(questions);
  const questionErrors = validateQuestionList(questionItems);
  const hasQuestionError = questions.length > 0 && questionErrors.length > 0;

  return (
    <>
      <Form.Field required style={{ marginBottom: 0 }}>
        <label style={{ textAlign: 'left' }}>Title</label>
      </Form.Field>

      <Form.Group inline sytle={{ alignItems: 'center' }}>
        <Form.Field style={{ flexGrow: 1 }}>
          <StringInput
            fluid
            icon="user"
            iconPosition="left"
            placeholder="My Election"
            value={name}
            maxLength={255}
            onChangeValue={setName}
            disabled={disabled}
          />
        </Form.Field>

        <Form.Field>
          <Popup
            on="hover"
            wide="very"
            position="right center"
            open={publicHover && !disabled}
            onOpen={() => setPublicHover(true)}
            onClose={() => setPublicHover(false)}
            content={
              <Message
                compact
                icon={isPublic ? 'lock open' : 'lock'}
                header={`${isPublic ? 'Public' : 'Private'} Election`}
                content={isPublic ? 'Open for anyone on the site to register' : 'Requires an access code to register'}
              />
            }
            trigger={
              <Checkbox toggle label="Public?" checked={isPublic} onChange={toggleIsPublic} disabled={disabled} />
            }
          />
        </Form.Field>
      </Form.Group>

      <Form.Field required error={hasQuestionError}>
        <label style={{ textAlign: 'left' }}>Questions</label>
        <Grid columns={2} stackable textAlign="center" divided>
          <Grid.Row stretched>
            <Grid.Column width="10">
              <Popup
                on="hover"
                wide="very"
                position="left center"
                open={hasQuestionError && questionsHover && !disabled}
                onOpen={() => setQuestionsHover(true)}
                onClose={() => setQuestionsHover(false)}
                content={<QuestionErrors errors={questionErrors} />}
                trigger={
                  <TextAreaInput
                    className={styles.textarea}
                    placeholder={PLACEHOLDER_TEXT}
                    value={questions}
                    onChangeValue={setQuestions}
                    disabled={disabled}
                  />
                }
              />
            </Grid.Column>

            <Grid.Column width="6" textAlign="left">
              <List ordered>
                {questionItems.map(([question, candidates], i) => (
                  // This is intended for the preview to work correctly
                  // eslint-disable-next-line react/no-array-index-key
                  <Fragment key={`${i}-${question}`}>
                    <List.Item className={styles['question-item']}>
                      {question}
                      <List.List as="ul" className={styles.candidate}>
                        {candidates.map((candidate, j) => (
                          // This is intended for the preview to work correctly:
                          // eslint-disable-next-line react/no-array-index-key
                          <List.Item as="li" key={`${j}-${candidate}`}>
                            {candidate}
                          </List.Item>
                        ))}
                      </List.List>
                    </List.Item>
                  </Fragment>
                ))}
              </List>
            </Grid.Column>
          </Grid.Row>
        </Grid>
      </Form.Field>
    </>
  );
};
