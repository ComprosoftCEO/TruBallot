import { useCallback, useRef } from 'react';
import { useState } from '@hookstate/core';
import { Link } from 'react-router-dom';
import { Grid, Transition, Image, Segment, Form, Message, Button } from 'semantic-ui-react';
import { StringInput } from 'components/input';
import { RECAPTCHA_SITE_KEY } from 'env';
import ReCAPTCHA from 'react-google-recaptcha';
import { store } from 'store';
import { apiLoading, getErrorInformation } from 'api';
import { isFormValid, logInUser } from './loginFormActions';

export const LoginForm = () => {
  const login = useState(store.login);

  const { username, password } = login.get();
  const { loginError } = login;
  const formValid = isFormValid(username, password);

  const recaptchaRef = useRef<ReCAPTCHA | null>(null);
  const onSubmit = useCallback(() => {
    if (!loginError.loading.value && recaptchaRef.current !== null) {
      store.login.loginError.set(apiLoading);
      logInUser(recaptchaRef.current, username, password);
    }
  }, [loginError.loading, password, username]);

  return (
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
                  onChangeValue={login.username.set}
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
                  onChangeValue={login.password.set}
                />
              </Form.Field>

              {!loginError.value.loading && !loginError.value.success && (
                <Message negative>
                  <b>Error: </b>
                  {getErrorInformation(loginError.value.error).description}
                </Message>
              )}

              <Button
                primary
                fluid
                size="large"
                icon="sign in"
                content="Login"
                onClick={onSubmit}
                disabled={!formValid || loginError.value.loading}
                loading={loginError.value.loading}
              />
            </Form>
          </Segment>

          <Message size="large">
            Don&apos;t have an account yet?
            <Link to="/register"> Register</Link>
          </Message>

          <ReCAPTCHA ref={recaptchaRef} sitekey={RECAPTCHA_SITE_KEY} size="invisible" />
        </Grid.Column>
      </Transition>
    </Grid>
  );
};
