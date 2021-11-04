import { useLastLocation } from 'react-router-last-location';
import { getErrorInformation } from 'api';
import { Button, Container, Divider, Form, Grid, Header, Message, Segment, Transition } from 'semantic-ui-react';
import { StringInput } from 'components/input';
import { DashboardMenu, TransitionList } from 'components/shared';
import { goBack } from 'helpers/goBack';
import { useTitle } from 'helpers/title';
import { nestedSelectorHook } from 'redux/helpers';
import { activateCode, setCode, useClearState } from './accessCodeActions';

const useSelector = nestedSelectorHook('accessCode');

// Expected length for the access codes
const CODE_LENGTH = 6;

export const AccessCode = () => {
  useTitle('Access Code');
  useClearState();

  const code = useSelector((state) => state.code);
  const loadingElection = useSelector((state) => state.loadingElection);
  const lastLocation = useLastLocation();

  return (
    <Grid textAlign="center">
      <Grid.Column style={{ maxWidth: 450 }}>
        <DashboardMenu />

        <Container style={{ marginTop: '8em' }} textAlign="center">
          <TransitionList animation="fade down" duration={500} totalDuration={300}>
            <div>
              <Header as="h1" textAlign="center">
                Access Code:
              </Header>

              <Segment raised>
                <Form size="large">
                  <Form.Field required>
                    <StringInput
                      fluid
                      icon="key"
                      iconPosition="left"
                      placeholder={'#'.repeat(CODE_LENGTH)}
                      maxLength={CODE_LENGTH}
                      value={code}
                      onChangeValue={setCode}
                      disabled={loadingElection.loading}
                    />
                  </Form.Field>

                  <Divider />

                  {!loadingElection.loading && !loadingElection.success && (
                    <Transition animation="fade down" duration={500} transitionOnMount>
                      <Message negative>
                        <b>Error: </b>
                        {getErrorInformation(loadingElection.error).description}
                      </Message>
                    </Transition>
                  )}

                  <Button
                    primary
                    size="large"
                    icon="send"
                    content="Enter"
                    onClick={activateCode}
                    disabled={code.length !== CODE_LENGTH || loadingElection.loading}
                    loading={loadingElection.loading}
                  />
                </Form>
              </Segment>
            </div>

            <div>
              <Button
                icon="arrow left"
                content="Go Back"
                style={{ marginTop: '2em' }}
                onClick={() => goBack(lastLocation)}
                disabled={loadingElection.loading}
              />
            </div>
          </TransitionList>
        </Container>
      </Grid.Column>
    </Grid>
  );
};
