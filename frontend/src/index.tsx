import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.scss';

// Import our Semantic UI theme
// eslint-disable-next-line import/no-extraneous-dependencies
import 'semantic-ui-less/semantic.less';

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root'),
);
