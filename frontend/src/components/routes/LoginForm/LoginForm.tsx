import { Grid, Transition, Image, Segment, Form, Message, Button } from 'semantic-ui-react';
import { useState } from '@hookstate/core';
import { StringInput } from 'components/input';
import { Link } from 'react-router-dom';

export const LoginForm = () => {
  const username = useState('');
  const password = useState('');

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
                  value={username.get()}
                  onChangeValue={username.set}
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
                  value={password.get()}
                  onChangeValue={password.set}
                />
              </Form.Field>

              <Button primary fluid size="large" icon="sign in" content="Login" />
            </Form>
          </Segment>

          <Message size="large">
            Don&apos;t have an account yet?
            <Link to="/register"> Register</Link>
          </Message>
        </Grid.Column>
      </Transition>
    </Grid>
  );
};
