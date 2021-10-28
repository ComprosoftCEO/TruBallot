import './App.scss';
import { createBrowserHistory } from 'history';
import { Router } from 'react-router-dom';
import { ErrorBoundary } from 'components/ErrorBoundary';
import { Routes } from 'components/routes';

export const history = createBrowserHistory();

function App() {
  return (
    <Router history={history}>
      <ErrorBoundary>
        <Routes />
      </ErrorBoundary>
    </Router>
  );
}

export default App;
