import { useCallback, useRef, useState } from 'react';
import { Prompt } from 'react-router-dom';
import ReCAPTCHA from 'react-google-recaptcha';
import zxcvbn from 'zxcvbn';
import * as EmailValidator from 'email-validator';
import { getErrorInformation } from 'api';
import { StringInput } from 'components/input';
import { RECAPTCHA_SITE_KEY } from 'env';
import { nestedSelectorHook } from 'redux/helpers';
import { useTitle } from 'helpers/title';
import { Grid, Image, Transition, Segment, Form, Message, Button, Header, Popup, Divider } from 'semantic-ui-react';
import { MINIMUM_PASSWORD_COMPLEXITY, SITE_SPECIFIC_WORDS } from 'helpers/passwordComplexity';
import { PasswordError } from 'components/shared';
import {
  useClearState,
  goBack,
  setEmail,
  setName,
  setPassword,
  setConfirm,
  registerUser,
  handleRecaptchaError,
} from './registerActions';

const useSelector = nestedSelectorHook('register');

export const Register = () => {
  useTitle('Register');
  useClearState();

  const [hover, setHover] = useState(false);

  const name = useSelector((state) => state.name);
  const email = useSelector((state) => state.email);
  const password = useSelector((state) => state.password);
  const confirm = useSelector((state) => state.confirm);
  const modified = useSelector((state) => state.modified);
  const registerError = useSelector((state) => state.registrationError);

  // Form validation
  const emailValid = EmailValidator.validate(email);
  const showEmailError = email.length > 0 && !emailValid;

  const passwordsMatch = password === confirm;
  const passwordStrength = zxcvbn(password, [name, email, ...SITE_SPECIFIC_WORDS]);
  const showPasswordError =
    password.length > 0 && (!passwordsMatch || passwordStrength.score < MINIMUM_PASSWORD_COMPLEXITY);

  const formValid =
    name.length > 0 &&
    email.length > 0 &&
    emailValid &&
    password.length > 0 &&
    confirm.length > 0 &&
    passwordsMatch &&
    passwordStrength.score >= MINIMUM_PASSWORD_COMPLEXITY;

  // Handle reCAPTCHA component
  const recaptchaRef = useRef<ReCAPTCHA | null>(null);
  const onSubmit = useCallback(() => {
    if (!registerError.loading && recaptchaRef.current !== null) {
      recaptchaRef.current.execute();
    }
  }, [recaptchaRef, registerError.loading]);

  return (
    <>
      <Grid textAlign="center" style={{ height: '100vh' }} verticalAlign="middle">
        <Transition animation="fade left" transitionOnMount>
          <Grid.Column style={{ maxWidth: 450 }}>
            <Image src="/truballot-logo.svg" />

            <div style={{ height: 50 }} />

            <Header size="large">Register New Account:</Header>

            <Segment raised>
              <Form size="large">
                <Form.Field required>
                  <label style={{ textAlign: 'left' }}>Name</label>
                  <StringInput
                    fluid
                    icon="user"
                    iconPosition="left"
                    placeholder="Name"
                    value={name}
                    maxLength={255}
                    onChangeValue={setName}
                    disabled={registerError.loading}
                  />
                </Form.Field>

                <Form.Field error={showEmailError} required>
                  <label style={{ textAlign: 'left' }}>Email</label>
                  <StringInput
                    fluid
                    icon="mail"
                    iconPosition="left"
                    placeholder="Email"
                    value={email}
                    maxLength={255}
                    onChangeValue={setEmail}
                    disabled={registerError.loading}
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
                        <label style={{ textAlign: 'left' }}>Password</label>
                        <StringInput
                          fluid
                          icon="lock"
                          iconPosition="left"
                          placeholder="Password"
                          type="password"
                          value={password}
                          maxLength={255}
                          onChangeValue={setPassword}
                          disabled={registerError.loading}
                        />
                      </Form.Field>

                      <Form.Field error={showPasswordError}>
                        <StringInput
                          fluid
                          icon="lock"
                          iconPosition="left"
                          placeholder="Retype Password"
                          type="password"
                          value={confirm}
                          maxLength={255}
                          onChangeValue={setConfirm}
                          disabled={registerError.loading}
                        />
                      </Form.Field>
                    </div>
                  }
                />

                <Divider />

                {!registerError.loading && !registerError.success && (
                  <Message negative>
                    <b>Error: </b>
                    {getErrorInformation(registerError.error).description}
                  </Message>
                )}

                <Button
                  primary
                  fluid
                  size="large"
                  icon="signup"
                  content="Register"
                  onClick={onSubmit}
                  disabled={!formValid || registerError.loading}
                  loading={registerError.loading}
                />
              </Form>
            </Segment>

            <Button icon="arrow left" content="Go Back" onClick={goBack} disabled={registerError.loading} />
          </Grid.Column>
        </Transition>
      </Grid>

      <ReCAPTCHA
        ref={recaptchaRef}
        sitekey={RECAPTCHA_SITE_KEY}
        size="invisible"
        onChange={() => registerUser(recaptchaRef.current!, name, email, password)}
        onErrored={() => handleRecaptchaError(recaptchaRef.current!)}
      />

      <Prompt message="Discard changes to the registration form?" when={modified} />
    </>
  );
};
