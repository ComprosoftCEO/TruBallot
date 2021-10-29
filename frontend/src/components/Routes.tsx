/**
 * Handles all of the front-end routing for the application
 */
/* eslint-disable react/jsx-props-no-spreading */
/* eslint-disable react/no-array-index-key */
import { Switch, Route, Redirect, RouteProps } from 'react-router-dom';
import { useState } from '@hookstate/core';
import { isLoggedIn } from 'axios-jwt';
import { Permission } from 'models/auth';
import { store } from 'store';

import { useEffect, useLayoutEffect } from 'react';
import { NotFound, PleaseLogIn } from './errorDialogs';
import { LoginForm } from './routes/LoginForm';

interface RouterEntry extends RouteProps {
  redirect?: string;
  permission?: Permission;
}

/// Routes that only appear when the user is NOT logged in
const LOGGED_OUT_ENTRIES: RouterEntry[] = [
  { path: '/', exact: true, redirect: '/login' },
  { path: '/dashboard', redirect: '/login' },
  { path: '/login', exact: true, component: LoginForm },
];

/// Rotues that only appear when the user IS logged in
const LOGGED_IN_ENTRIES: RouterEntry[] = [];

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

//
// Main component for front-end routing
//
export const Routes = () => {
  const permissions = useState(store.globals.permissions);

  // Test if the page is logged in when it first loads
  const loggedIn = useState(isLoggedIn);
  return loggedIn.get() ? LoggedInSwitch(permissions.get()) : LoggedOutSwitch;
};
