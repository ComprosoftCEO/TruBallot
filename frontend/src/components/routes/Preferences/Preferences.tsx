import { useState } from 'react';
import zxcvbn from 'zxcvbn';
import { MINIMUM_PASSWORD_COMPLEXITY, SITE_SPECIFIC_WORDS } from 'helpers/passwordComplexity';
import { useTitle } from 'helpers/title';
import { nestedSelectorHook } from 'redux/helpers';
import { useLastLocation } from 'react-router-last-location';
import { goBack } from 'helpers/goBack';
import { Button, Container, Divider, Form, Grid, Header, Message, Popup, Segment, Transition } from 'semantic-ui-react';
import { StringInput } from 'components/input';
import { DashboardMenu, PasswordError } from 'components/shared';
import { getErrorInformation } from 'api';
import { Prompt } from 'react-router-dom';
import {
  cancelUpdatePreferences,
  cancelUpdatingPassword,
  clearUpdatePasswordSuccess,
  clearUpdatePreferencesSuccess,
  setConfirmPassword,
  setCurrentPassword,
  setNewName,
  setNewPassword,
  updatePassword,
  updatePreferences,
  useClearState,
} from './preferencesActions';

const useSelector = nestedSelectorHook('preferences');
const useGlobalsSelector = nestedSelectorHook('globals');

export const Preferences = () => {
  useClearState();
  useTitle('Preferences');

  const [hover, setHover] = useState(false);

  const newName = useSelector((state) => state.newName);
  const name = useGlobalsSelector((state) => state.name);
  const email = useGlobalsSelector((state) => state.email);
  const updatingPreferences = useSelector((state) => state.updatingPreferences);

  const currentPassword = useSelector((state) => state.currentPassword);
  const newPassword = useSelector((state) => state.newPassword);
  const confirmPassword = useSelector((state) => state.confirmPassword);
  const updatingPassword = useSelector((state) => state.updatingPassword);

  const modified = useSelector((state) => state.preferencesModified || state.passwordModified);
  const lastLocation = useLastLocation();

  // Password validator
  const passwordsMatch = newPassword === confirmPassword;
  const passwordStrength = zxcvbn(newPassword, [name, email, ...SITE_SPECIFIC_WORDS]);
  const showPasswordError =
    newPassword.length > 0 && (!passwordsMatch || passwordStrength.score < MINIMUM_PASSWORD_COMPLEXITY);

  const passwordFormValid =
    currentPassword.length > 0 &&
    newPassword.length > 0 &&
    passwordsMatch &&
    passwordStrength.score >= MINIMUM_PASSWORD_COMPLEXITY;

  // When is form loading?
  const loading = updatingPreferences.loading || updatingPassword.loading;

  return (
    <Grid textAlign="center">
      <Grid.Column style={{ maxWidth: 450 }}>
        <DashboardMenu />
        <Container style={{ marginTop: '8em' }} textAlign="center">
          <Header as="h1" textAlign="center">
            Account Preferences:
          </Header>

          <Segment raised>
            <Form size="large">
              <Form.Field required>
                <label style={{ textAlign: 'left' }}>Name</label>
                <StringInput
                  fluid
                  icon="user"
                  iconPosition="left"
                  placeholder="Name"
                  maxLength={255}
                  value={newName}
                  onChangeValue={setNewName}
                  disabled={loading}
                />
              </Form.Field>

              <Form.Field>
                <label style={{ textAlign: 'left' }}>Email</label>
                <Popup
                  on="hover"
                  position="right center"
                  content="Fixed Value"
                  trigger={
                    <StringInput
                      fluid
                      icon="mail"
                      iconPosition="left"
                      placeholder="Email"
                      readOnly
                      value={email}
                      disabled={loading}
                    />
                  }
                />
              </Form.Field>

              <Divider />

              {!updatingPreferences.loading && !updatingPreferences.success && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(updatingPreferences.error).description}
                  </Message>
                </Transition>
              )}

              {!updatingPreferences.loading && updatingPreferences.success && updatingPreferences.data && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message positive content="Account Preferences Updated!" onDismiss={clearUpdatePreferencesSuccess} />
                </Transition>
              )}

              <Button
                primary
                size="large"
                icon="save"
                content="Update"
                onClick={updatePreferences}
                disabled={newName.length === 0 || newName === name || loading}
                loading={updatingPreferences.loading}
              />

              <Button
                size="large"
                icon="cancel"
                content="Cancel"
                onClick={cancelUpdatePreferences}
                disabled={loading}
              />
            </Form>
          </Segment>

          <Header as="h1" textAlign="center" style={{ marginTop: '2em' }}>
            Change Password:
          </Header>

          <Segment raised>
            <Form size="large">
              <Form.Field required>
                <label style={{ textAlign: 'left' }}>Current Password</label>
                <StringInput
                  fluid
                  icon="lock"
                  iconPosition="left"
                  placeholder="Current Password"
                  type="password"
                  maxLength={255}
                  value={currentPassword}
                  onChangeValue={setCurrentPassword}
                  disabled={loading}
                />
              </Form.Field>

              <Popup
                wide
                on="hover"
                content={<PasswordError passwordsMatch={passwordsMatch} passwordStrength={passwordStrength} />}
                position="right center"
                open={hover && showPasswordError}
                onOpen={() => setHover(true)}
                onClose={() => setHover(false)}
                trigger={
                  <div>
                    <Form.Field required error={showPasswordError}>
                      <label style={{ textAlign: 'left' }}>New Password</label>
                      <StringInput
                        fluid
                        icon="lock"
                        iconPosition="left"
                        placeholder="New Password"
                        type="password"
                        maxLength={255}
                        value={newPassword}
                        onChangeValue={setNewPassword}
                        disabled={loading}
                      />
                    </Form.Field>

                    <Form.Field required error={showPasswordError}>
                      <StringInput
                        fluid
                        icon="lock"
                        iconPosition="left"
                        placeholder="Retype Password"
                        type="password"
                        maxLength={255}
                        value={confirmPassword}
                        onChangeValue={setConfirmPassword}
                        disabled={loading}
                      />
                    </Form.Field>
                  </div>
                }
              />

              <Divider />

              {!updatingPassword.loading && !updatingPassword.success && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(updatingPassword.error).description}
                  </Message>
                </Transition>
              )}

              {!updatingPassword.loading && updatingPassword.success && updatingPassword.data && (
                <Transition animation="fade down" duration={500} transitionOnMount>
                  <Message positive content="Account Password Changed!" onDismiss={clearUpdatePasswordSuccess} />
                </Transition>
              )}

              <Button
                primary
                size="large"
                icon="save"
                content="Update"
                onClick={updatePassword}
                disabled={loading || !passwordFormValid}
                loading={updatingPassword.loading}
              />

              <Button size="large" icon="cancel" content="Cancel" onClick={cancelUpdatingPassword} disabled={loading} />
            </Form>
          </Segment>

          <Button
            icon="arrow left"
            content="Go Back"
            style={{ marginTop: '2em' }}
            onClick={() => goBack(lastLocation)}
            disabled={loading}
          />
        </Container>

        <Prompt message="Discard changes to account preferences?" when={modified} />
      </Grid.Column>
    </Grid>
  );
};
