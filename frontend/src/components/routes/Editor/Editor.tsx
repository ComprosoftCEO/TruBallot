import { StringInput, TextAreaInput } from 'components/input';
import { useTitle } from 'helpers/title';
import { Fragment, useState } from 'react';
import {
  Message,
  Button,
  Container,
  Divider,
  Form,
  Segment,
  Header,
  Checkbox,
  Grid,
  List,
  Popup,
} from 'semantic-ui-react';
import { PLACEHOLDER_TEXT, parseListString, useClearState } from './editorActions';
import styles from './editor.module.scss';

export const Editor = () => {
  useTitle('Create Election');
  useClearState();

  const [value, setValue] = useState('');
  const [publicCheck, setPublic] = useState(false);
  const items = parseListString(value);

  return (
    <Container textAlign="center" text>
      <Header size="large" style={{ marginTop: '3em' }}>
        Create Election:
      </Header>

      <Segment raised padded>
        <Form size="large">
          <Form.Field required style={{ marginBottom: 0 }}>
            <label style={{ textAlign: 'left' }}>Title</label>
          </Form.Field>

          <Form.Group inline sytle={{ alignItems: 'center' }}>
            <Form.Field style={{ flexGrow: 1 }}>
              <StringInput fluid icon="user" iconPosition="left" placeholder="Title" />
            </Form.Field>

            <Form.Field>
              <Popup
                on="hover"
                wide="very"
                position="right center"
                style={{ zIndex: 900 }}
                content={
                  <Message
                    compact
                    icon={publicCheck ? 'lock open' : 'lock'}
                    header={`${publicCheck ? 'Public' : 'Private'} Election`}
                    content={
                      publicCheck ? 'Open for anyone on the site to register' : 'Requires an access code to register'
                    }
                  />
                }
                trigger={
                  <Checkbox toggle label="Public?" checked={publicCheck} onChange={() => setPublic(!publicCheck)} />
                }
              />
            </Form.Field>
          </Form.Group>

          <Form.Field required>
            <label style={{ textAlign: 'left' }}>Questions</label>
            <Grid columns={2} stackable textAlign="center" divided>
              <Grid.Row stretched>
                <Grid.Column width="10">
                  <TextAreaInput
                    style={{ minHeight: 200, fontFamily: 'monospace' }}
                    placeholder={PLACEHOLDER_TEXT}
                    value={value}
                    onChangeValue={setValue}
                  />
                </Grid.Column>

                <Grid.Column width="6" textAlign="left">
                  <List ordered>
                    {items.map(([question, candidates], i) => (
                      // This is intended:
                      // eslint-disable-next-line react/no-array-index-key
                      <Fragment key={`${i}-${question}`}>
                        <List.Item className={styles['question-item']}>
                          {question}
                          <List.List as="ul" style={{ paddingInlineStart: 20, marginTop: 6 }}>
                            {candidates.map((candidate, j) => (
                              // This is intended:
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

          <Divider />

          <Button primary size="large" icon="plus" content="Create Election" />
        </Form>
      </Segment>

      <Button icon="arrow left" content="Go Back" />
    </Container>
  );
};
