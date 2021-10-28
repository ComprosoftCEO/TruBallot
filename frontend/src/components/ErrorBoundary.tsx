import React, { useState, useCallback } from 'react';
import { useState as useHookState } from '@hookstate/core';
import { store } from 'store';
import { history } from 'index';
import { getErrorInformation, GlobalErrorCode } from 'api';
import { ErrorOccured, SessionExpired } from 'components/errorDialogs';

export interface ErrorBoundaryProps {
  children?: React.ReactNode;
}

/// Error codes that indicate the JWT token has expired
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

  //
  // Set an error flag if an error has been set in the state
  //
  static getDerivedStateFromProps(nextProps: ErrorBoundaryComponentProps, prevState: ErrorBoundaryComponentState) {
    if (nextProps.globalError !== null) {
      return { ...prevState, hasError: true, lastError: nextProps.globalError };
    }

    return prevState;
  }

  static getDerivedStateFromError(error: Error | null) {
    return { hasError: true, lastError: error || new Error('Error swallowed during propagation.') };
  }

  //
  // Method called when an error is thrown
  //
  componentDidCatch(error: Error | null, errorInfo: object) {
    // We could also log the error to an error reporting service
    // eslint-disable-next-line no-console
    console.error(error, errorInfo);
  }

  //
  // Clear the error and redraw the error boundary
  //
  clearError = () => {
    const { forceRerender } = this.props;

    store.globals.merge({ globalError: null, redirect: history.location.pathname });
    forceRerender();
  };

  //
  // Render the component
  //
  render() {
    const { children } = this.props;
    const { hasError, lastError } = this.state;

    if (hasError) {
      const errorInformation = getErrorInformation(lastError);

      if (LOGIN_EXPIRED_CODES.includes(errorInformation.errorCode)) {
        // Session timeout
        return <SessionExpired clearError={this.clearError} />;
      }

      // Normal error
      return <ErrorOccured message={errorInformation.description} />;
    }

    // Render the coponent as normal
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
