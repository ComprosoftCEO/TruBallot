import React, { useState, useCallback } from 'react';
import { useState as useHookState } from '@hookstate/core';
import { Dimmer, Button, Segment, Message, Divider, Icon } from 'semantic-ui-react';
import { store } from 'state/store';
import { history } from 'App';
import { getErrorInformation, GlobalErrorCode } from 'api/error';

export interface ErrorBoundaryProps {
  children?: React.ReactNode;
}

const MISSING_ERROR = 'Error swallowed during propagation.';
const LOGIN_EXPIRED_CODES: (GlobalErrorCode | null)[] = [GlobalErrorCode.InvalidJWTToken];

interface ErrorBoundaryComponentProps {
  globalError: Error | null;

  // Used to force refresh the component to clear the error
  forceRerender: () => void;
}

interface ErrorBoundaryComponentState {
  hasError: boolean;
  lastError: Error | null;
}

class ErrorBoundaryComponent extends React.Component<ErrorBoundaryComponentProps, ErrorBoundaryComponentState> {
  constructor(props: ErrorBoundaryComponentProps) {
    super(props);
    this.state = { hasError: false, lastError: null };

    if (props.globalError !== null) {
      throw props.globalError;
    }
  }

  // Set an error flag if an error has been set in redux
  static getDerivedStateFromProps(nextProps: ErrorBoundaryComponentProps, prevState: ErrorBoundaryComponentState) {
    if (nextProps.globalError !== null) {
      return { ...prevState, hasError: true, lastError: nextProps.globalError };
    }

    return prevState;
  }

  static getDerivedStateFromError(error: Error | null) {
    return { hasError: true, lastError: error || new Error(MISSING_ERROR) };
  }

  componentDidCatch(error: Error | null, errorInfo: object) {
    // You can also log the error to an error reporting service
    // eslint-disable-next-line no-console
    console.error(error, errorInfo);
  }

  loginRedirect = () => {
    const { forceRerender } = this.props;

    // Clear the error and set the redirect
    store.globals.merge({ globalError: null, redirect: history.location.pathname });

    // Redirect to the login page and re-render the component to clear the error
    history.push('/login?redirect');
    forceRerender();
  };

  render() {
    const { children } = this.props;
    const { hasError, lastError } = this.state;

    if (hasError) {
      const errorInformation = getErrorInformation(lastError);

      if (LOGIN_EXPIRED_CODES.includes(errorInformation.errorCode)) {
        // Session timeout
        return (
          <Dimmer active>
            <Segment>
              <Message info>
                <Message.Header>
                  Oops! It looks like your login session has expired due to inactivity.
                  <br />
                  Please log back in to continue using the system.
                </Message.Header>
              </Message>
              <Divider horizontal />
              <Button fluid color="orange" onClick={this.loginRedirect}>
                <Icon name="sign in" />
                Log Back In
                <i> (Redirect)</i>
              </Button>

              <div style={{ height: '20px' }} />
              <Button fluid icon="redo" content="Reload Page" onClick={window.location.reload} />

              <div style={{ height: '10px' }} />
              <Button primary fluid icon="home" content="Go Home" as="a" href="/" />
            </Segment>
          </Dimmer>
        );
      }

      // Normal error
      return (
        <Dimmer active>
          <Segment>
            <Message negative>
              <Message.Header>Oops! Looks like something went wrong.</Message.Header>
              <b>Error: </b>
              {errorInformation.description}
            </Message>
            <Button fluid icon="redo" content="Reload Page" onClick={window.location.reload} />
            <div style={{ height: '10px' }} />
            <Button primary fluid icon="home" content="Go Home" as="a" href="/" />
          </Segment>
        </Dimmer>
      );
    }

    return children;
  }
}

// We use a separate ErrorBoundary component so we can force it to
//  re-render in order to clear the error
//
// The example code is from: https://github.com/bvaughn/react-error-boundary/issues/23
export const ErrorBoundary = ({ children }: ErrorBoundaryProps) => {
  const globalError = useHookState(store.globals.globalError);
  const [key, setKey] = useState<number>(0);
  const updateKey = useCallback(() => setKey(key + 1), [key]);

  return (
    <ErrorBoundaryComponent key={key} forceRerender={updateKey} globalError={globalError.get()}>
      {children}
    </ErrorBoundaryComponent>
  );
};
