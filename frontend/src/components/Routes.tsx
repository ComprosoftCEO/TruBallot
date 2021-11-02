/**
 * Handles all of the front-end routing for the application
 */
/* eslint-disable react/jsx-props-no-spreading */
/* eslint-disable react/no-array-index-key */
import { useLayoutEffect } from 'react';
import { Switch, Route, Redirect, RouteProps } from 'react-router-dom';
import { getAccessToken } from 'axios-jwt';
import { Permission, ClientToken } from 'models/auth';
import { mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import jwt from 'jsonwebtoken';

import { NotFound, PleaseLogIn } from './errorDialogs';
import { LoginForm } from './routes/LoginForm';
import { Register } from './routes/Register';
import { Dashboard, DashboardFilter } from './routes/Dashboard';

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
];

/// Routes that always appear
const BOTH_ENTRIES: RouterEntry[] = [];

const LoggedOutSwitch = (
  <Switch>
    {LOGGED_OUT_ENTRIES.map(({ redirect, children, ...entry }, index) => (
      <Route key={index} {...entry}>
        {redirect ? <Redirect to={redirect} /> : children}
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
const mergeState = mergeNestedState('globals');

//
// Main component for front-end routing
//
export const Routes = () => {
  const permissions = useSelector((store) => store.permissions);
  const loggedIn = useSelector((store) => store.isLoggedIn);

  // Test if the page is logged in when it first loads
  useLayoutEffect(loadAccessToken, []);

  return loggedIn ? LoggedInSwitch(permissions) : LoggedOutSwitch;
};

/**
 * Load the access token and the permissions when the page first loads
 */
function loadAccessToken(): void {
  // Parse the access token if the user is currently logged in
  const accessToken = getAccessToken();
  if (accessToken !== undefined) {
    const clientToken: ClientToken = jwt.decode(accessToken) as ClientToken;

    // User is logged in
    mergeState({
      isLoggedIn: true,
      name: clientToken.name,
      email: clientToken.email,
      permissions: new Set(clientToken.permissions),
    });
  } else {
    // User is logged out
    mergeState({ isLoggedIn: false });
  }
}
