import React from 'react';
import ReactDOM from 'react-dom';
import { Router } from 'react-router-dom';
import { createBrowserHistory } from 'history';

import { ErrorBoundary } from 'components/ErrorBoundary';
import { ConfirmDialog } from 'components/ConfirmDialog';
import { Routes } from 'components/Routes';

// Import our styles
import './index.scss';

// Import our Semantic UI theme
// eslint-disable-next-line import/no-extraneous-dependencies
import 'semantic-ui-less/semantic.less';

// Create history for front-end routing
export const history = createBrowserHistory();

ReactDOM.render(
  <React.StrictMode>
    <Router history={history}>
      <ErrorBoundary>
        <ConfirmDialog />
        <Routes />
      </ErrorBoundary>
    </Router>
  </React.StrictMode>,
  document.getElementById('root'),
);
