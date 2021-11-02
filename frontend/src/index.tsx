import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import { Router } from 'react-router-dom';
import { LastLocationProvider } from 'react-router-last-location';
import { createBrowserHistory } from 'history';

import { ErrorBoundary } from 'components/ErrorBoundary';
import { ConfirmDialog } from 'components/ConfirmDialog';
import { Routes } from 'components/Routes';
import { store } from 'redux/store';

// Import our styles
import './index.scss';

// Import our Semantic UI theme
// eslint-disable-next-line import/no-extraneous-dependencies
import 'semantic-ui-less/semantic.less';

// Create history for front-end routing
export const history = createBrowserHistory();

ReactDOM.render(
  <React.StrictMode>
    <Provider store={store}>
      <Router history={history}>
        <LastLocationProvider>
          <ErrorBoundary>
            <ConfirmDialog />
            <Routes />
          </ErrorBoundary>
        </LastLocationProvider>
      </Router>
    </Provider>
  </React.StrictMode>,
  document.getElementById('root'),
);
