/**
 * Handles all of the front-end routing for the application
 */
/* eslint-disable react/jsx-props-no-spreading */
/* eslint-disable react/no-array-index-key */
import { useLayoutEffect } from 'react';
import { Switch, Route, Redirect, RouteProps } from 'react-router-dom';
import { getRefreshToken } from 'axios-jwt';
import { Permission } from 'models/auth';
import { nestedSelectorHook } from 'redux/helpers';

import { logInStore, logOutStore } from 'redux/auth';
import { NotFound, PleaseLogIn } from './errorDialogs';
import { LoginForm } from './routes/LoginForm';
import { Register } from './routes/Register';
import { Dashboard, DashboardFilter } from './routes/Dashboard';
import { CreateElection, EditElection } from './routes/Editor';
import { ManageElection } from './routes/ManageElection';
import { AccessCode } from './routes/AccessCode';
import { Vote } from './routes/Vote';
import { Results } from './routes/Results';
import { Preferences } from './routes/Preferences';

interface RouterEntry extends RouteProps {
  redirect?: string;
  permission?: Permission;
}

/// Routes that only appear when the user is NOT logged in
const LOGGED_OUT_ENTRIES: RouterEntry[] = [
  { path: '/', exact: true, redirect: '/login' },
  { path: '/login', exact: true, component: LoginForm },
  { path: '/register', exact: true, component: Register },
];

/// Rotues that only appear when the user IS logged in
const LOGGED_IN_ENTRIES: RouterEntry[] = [
  { path: '/', exact: true, redirect: '/dashboard' },
  { path: '/login', redirect: '/dashboard' },

  // Dashboard routes
  { path: '/dashboard', exact: true, component: Dashboard },

  // Dashboard my elections
  { path: '/dashboard/my-elections', exact: true, children: <Dashboard filter={DashboardFilter.MyElectionsAll} /> },
  {
    path: '/dashboard/my-elections/drafts',
    exact: true,
    children: <Dashboard filter={DashboardFilter.MyElectionsDraft} />,
  },
  {
    path: '/dashboard/my-elections/open',
    exact: true,
    children: <Dashboard filter={DashboardFilter.MyElectionsOpen} />,
  },
  {
    path: '/dashboard/my-elections/voting',
    exact: true,
    children: <Dashboard filter={DashboardFilter.MyElectionsVoting} />,
  },
  {
    path: '/dashboard/my-elections/closed',
    exact: true,
    children: <Dashboard filter={DashboardFilter.MyElectionsClosed} />,
  },

  // Dashboard public elections
  {
    path: '/dashboard/public-elections',
    exact: true,
    children: <Dashboard filter={DashboardFilter.PublicElectionsAll} />,
  },
  {
    path: '/dashboard/public-elections/open',
    exact: true,
    children: <Dashboard filter={DashboardFilter.PublicElectionsOpen} />,
  },
  {
    path: '/dashboard/public-elections/voting',
    exact: true,
    children: <Dashboard filter={DashboardFilter.PublicElectionsVoting} />,
  },
  {
    path: '/dashboard/public-elections/closed',
    exact: true,
    children: <Dashboard filter={DashboardFilter.PublicElectionsClosed} />,
  },

  // Dashboard registrations
  {
    path: '/dashboard/registrations',
    exact: true,
    children: <Dashboard filter={DashboardFilter.RegistrationsAll} />,
  },
  {
    path: '/dashboard/registrations/open',
    exact: true,
    children: <Dashboard filter={DashboardFilter.RegistrationsOpen} />,
  },
  {
    path: '/dashboard/registrations/voting',
    exact: true,
    children: <Dashboard filter={DashboardFilter.RegistrationsVoting} />,
  },
  {
    path: '/dashboard/registrations/closed',
    exact: true,
    children: <Dashboard filter={DashboardFilter.RegistrationsClosed} />,
  },

  // Election
  { path: '/elections/create', exact: true, component: CreateElection, permission: Permission.CreateElection },
  { path: '/elections/access-code', exact: true, component: AccessCode },
  { path: '/elections/:electionId', exact: true, component: ManageElection },
  { path: '/elections/:electionId/edit', exact: true, component: EditElection, permission: Permission.CreateElection },
  { path: '/elections/:electionId/vote', exact: true, component: Vote, permission: Permission.Vote },
  { path: '/elections/:electionId/results', exact: true, component: Results },

  // User preferences
  { path: '/preferences', exact: true, component: Preferences },
];

/// Routes that always appear
const BOTH_ENTRIES: RouterEntry[] = [];

const LoggedOutSwitch = (
  <Switch>
    {LOGGED_OUT_ENTRIES.map(({ redirect, children, ...entry }, index) => (
      <Route key={index} {...entry}>
        {redirect ? <Redirect to={{ pathname: redirect, state: { preventLastLocation: true } }} /> : children}
      </Route>
    ))}
    {LOGGED_IN_ENTRIES.filter((entry) => typeof entry.redirect === 'undefined').map((entry, index) => (
      <Route key={index} {...entry}>
        <PleaseLogIn />
      </Route>
    ))}
    {BOTH_ENTRIES.map((entry, index) => (
      <Route key={index} {...entry} />
    ))}
    <Route>
      <NotFound />
    </Route>
  </Switch>
);

const LoggedInSwitch = (permissions: Set<Permission>) => (
  <Switch>
    {LOGGED_IN_ENTRIES.map(({ redirect, permission, children, ...entry }, index) => {
      // Hide routes if user doesn't have permissions
      if (typeof permission !== 'undefined' && !permissions.has(permission)) {
        return (
          <Route key={index} {...entry}>
            <NotFound noPermission />
          </Route>
        );
      }

      return (
        <Route key={index} {...entry}>
          {redirect ? <Redirect to={redirect} /> : children}
        </Route>
      );
    })}
    {BOTH_ENTRIES.map((entry, index) => (
      <Route key={index} {...entry} />
    ))}
    <Route>
      <NotFound />
    </Route>
  </Switch>
);

const useSelector = nestedSelectorHook('globals');

//
// Main component for front-end routing
//
export const Routes = () => {
  const permissions = useSelector((store) => store.permissions);
  const loggedIn = useSelector((store) => store.isLoggedIn);

  // Test if the page is logged in when it first loads
  useLayoutEffect(() => {
    const refreshToken = getRefreshToken();
    if (refreshToken !== undefined) {
      logInStore(refreshToken);
    } else {
      logOutStore();
    }
  }, []);

  return loggedIn ? LoggedInSwitch(permissions) : LoggedOutSwitch;
};
