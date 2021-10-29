import { useCallback, useRef } from 'react';
import { Link } from 'react-router-dom';
import { Grid, Transition, Image, Segment, Form, Message, Button } from 'semantic-ui-react';
import { StringInput } from 'components/input';
import { RECAPTCHA_SITE_KEY } from 'env';
import ReCAPTCHA from 'react-google-recaptcha';
import { apiLoading, getErrorInformation } from 'api';
import { mergeNestedState, nestedSelectorHook, setNestedProperty } from 'redux/helpers';
import { isFormValid, logInUser, useClearState } from './loginFormActions';

const useSelector = nestedSelectorHook('login');
const mergeState = mergeNestedState('login');
const setProperty = setNestedProperty('login');

export const LoginForm = () => {
  useClearState();

  const username = useSelector((state) => state.username);
  const password = useSelector((state) => state.password);
  const loginError = useSelector((state) => state.loginError);
  const formValid = isFormValid(username, password);

  // Handle reCAPTCHA component
  const recaptchaRef = useRef<ReCAPTCHA | null>(null);
  const onSubmit = useCallback(() => {
    if (!loginError.loading && recaptchaRef.current !== null) {
      mergeState({ loginError: apiLoading() });
      logInUser(recaptchaRef.current, username, password);
    }
  }, [loginError.loading, password, username]);

  return (
    <>
      <Grid textAlign="center" style={{ height: '100vh' }} verticalAlign="middle">
        <Transition animation="scale" duration={300} transitionOnMount>
          <Grid.Column style={{ maxWidth: 450 }}>
            <Image src="/truballot-logo.svg" />

            <div style={{ height: 50 }} />

            <Segment raised>
              <Form size="large">
                <Form.Field>
                  <label style={{ textAlign: 'left' }}>Email</label>
                  <StringInput
                    fluid
                    icon="user"
                    iconPosition="left"
                    placeholder="Email"
                    value={username}
                    onChangeValue={setProperty('username')}
                  />
                </Form.Field>
                <Form.Field>
                  <label style={{ textAlign: 'left' }}>Password</label>
                  <StringInput
                    fluid
                    icon="lock"
                    iconPosition="left"
                    placeholder="Password"
                    type="password"
                    value={password}
                    onChangeValue={setProperty('password')}
                  />
                </Form.Field>

                {!loginError.loading && !loginError.success && (
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(loginError.error).description}
                  </Message>
                )}

                <Button
                  primary
                  fluid
                  size="large"
                  icon="sign in"
                  content="Login"
                  onClick={onSubmit}
                  disabled={!formValid || loginError.loading}
                  loading={loginError.loading}
                />
              </Form>
            </Segment>

            <Message size="large">
              Don&apos;t have an account yet?
              <Link to="/register"> Register</Link>
            </Message>
          </Grid.Column>
        </Transition>
      </Grid>
      <ReCAPTCHA ref={recaptchaRef} sitekey={RECAPTCHA_SITE_KEY} size="invisible" />
    </>
  );
};
