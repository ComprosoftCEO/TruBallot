import { Prompt } from 'react-router-dom';
import { useLastLocation } from 'react-router-last-location';
import { nestedSelectorHook } from 'redux/helpers';
import { useTitle } from 'helpers/title';
import { Message, Button, Container, Divider, Form, Segment, Header, Transition } from 'semantic-ui-react';
import { DashboardMenu } from 'components/shared';
import { getErrorInformation } from 'api';
import { goBack } from 'helpers/goBack';
import { Editor } from './Editor';
import { useClearState, useIsFormValid, createElection } from './editorActions';

const useSelector = nestedSelectorHook('editor');

export const CreateElection = () => {
  useTitle('Create Election');
  useClearState();

  const modified = useSelector((store) => store.modified);
  const creatingElection = useSelector((store) => store.submitting);

  const formValid = useIsFormValid();
  const lastLocation = useLastLocation();

  return (
    <>
      <DashboardMenu />
      <Transition animation="scale" duration={300} transitionOnMount>
        <Container textAlign="center" text style={{ marginTop: '8em' }}>
          <Header size="large">Create Election:</Header>

          <Segment raised padded>
            <Form size="large">
              <Editor disabled={creatingElection.loading} />

              <Divider />

              {!creatingElection.loading && !creatingElection.success && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(creatingElection.error).description}
                  </Message>
                </Transition>
              )}

              <Button
                primary
                size="large"
                icon="edit outline"
                content="Create Draft"
                onClick={createElection}
                disabled={!formValid || creatingElection.loading}
                loading={creatingElection.loading}
              />
            </Form>
          </Segment>

          <Button
            icon="arrow left"
            content="Go Back"
            style={{ marginTop: '2em' }}
            onClick={() => goBack(lastLocation)}
            disabled={creatingElection.loading}
          />

          <Prompt message="Discard changes to new election?" when={modified} />
        </Container>
      </Transition>
    </>
  );
};
