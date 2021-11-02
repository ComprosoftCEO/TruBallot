import { Prompt } from 'react-router-dom';
import { useLastLocation } from 'react-router-last-location';
import { nestedSelectorHook } from 'redux/helpers';
import { useTitle } from 'helpers/title';
import { Message, Button, Container, Divider, Form, Segment, Header, Transition } from 'semantic-ui-react';
import { getErrorInformation } from 'api';
import { DashboardMenu, ErrorPortal } from 'components/shared';
import { ErrorOccured } from 'components/errorDialogs';
import { Editor } from './Editor';
import {
  useClearState,
  useIsFormValid,
  goBack,
  useFetchElection,
  useElectionId,
  saveElection,
  reloadElection,
  clearReloadError,
  getFatalError,
  tryReFetchElection,
} from './editorActions';

const useSelector = nestedSelectorHook('editor');
const useGlobalsSelector = nestedSelectorHook('globals');

export const EditElection = () => {
  useTitle('Edit Election');
  useClearState();

  // Fetch the election to edit
  const electionId = useElectionId();
  useFetchElection(electionId);

  const election = useSelector((store) => store.electionDetails);
  const reloadingElection = useSelector((store) => store.reloading);
  const modified = useSelector((store) => store.modified);
  const updatingElection = useSelector((store) => store.submitting);

  const formValid = useIsFormValid();
  const lastLocation = useLastLocation();

  // Test for a fatal error when or after loading the resource
  const userId = useGlobalsSelector((state) => state.userId);
  const fatalError = getFatalError(election, userId);
  if (fatalError !== undefined) {
    return <ErrorOccured header="Error: Cannot edit election!" message={fatalError} />;
  }

  // Show the blank loading form
  if (election.loading || !election.success) {
    return (
      <>
        <DashboardMenu />
        <Transition animation="scale" duration={300} transitionOnMount>
          <Container textAlign="center" text style={{ marginTop: '8em' }}>
            <Header size="large">Edit Election:</Header>

            <Segment raised padded loading style={{ minHeight: 400 }} />

            <Button
              icon="arrow left"
              content="Go Back"
              onClick={() => goBack(lastLocation)}
              disabled={updatingElection.loading}
            />
          </Container>
        </Transition>

        {!election.loading && !election.success && (
          <ErrorPortal
            negative
            header="Failed to load election"
            content={getErrorInformation(election.error).description}
            onReload={() => tryReFetchElection(electionId)}
          />
        )}
      </>
    );
  }

  // Show the main form since everything loaded successfully
  return (
    <>
      <DashboardMenu />
      <Transition animation="scale" duration={300} transitionOnMount>
        <Container textAlign="center" text style={{ marginTop: '8em' }}>
          <Header size="large">Edit Election:</Header>

          <Segment raised padded>
            <Form size="large">
              <Editor disabled={updatingElection.loading || reloadingElection.loading} />

              <Divider />

              {!updatingElection.loading && !updatingElection.success && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(updatingElection.error).description}
                  </Message>
                </Transition>
              )}

              <Button
                primary
                size="large"
                icon="save"
                content="Save Changes"
                onClick={() => saveElection(electionId)}
                disabled={!formValid || !modified || updatingElection.loading || reloadingElection.loading}
                loading={updatingElection.loading}
              />

              <Button
                size="large"
                icon="redo"
                content="Reload"
                onClick={() => reloadElection(electionId)}
                disabled={updatingElection.loading || reloadingElection.loading}
                loading={reloadingElection.loading}
              />
            </Form>
          </Segment>

          <Button
            icon="arrow left"
            content="Go Back"
            onClick={() => goBack(lastLocation)}
            disabled={updatingElection.loading || reloadingElection.loading}
          />

          <Prompt message="Discard changes to election?" when={modified} />
        </Container>
      </Transition>

      {!reloadingElection.loading && !reloadingElection.success && (
        <ErrorPortal
          negative
          header="Failed to reload election"
          content={getErrorInformation(reloadingElection.error).description}
          onReload={clearReloadError}
        />
      )}
    </>
  );
};
