# Electronic Voting Frontend

React single-page application to display the frontend

<br />

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app).

<br />

## Main Frameworks Used

- **[React](https://reactjs.org/)** - Define how frontend components render given the state.
- **[JSX](https://reactjs.org/docs/introducing-jsx.html)** - Define react components using tags similar to HTML or XML
- **[TypeScript](https://www.typescriptlang.org/)** - Variant of JavaScript that adds static typing
- **[Semantic UI React](https://react.semantic-ui.com/)** - CSS interface library
- **[React Router](https://reactrouter.com/)** - Handles frontend routing for the single-page application
- **[Redux](https://redux.js.org/)** - Stores global application state
- **[SASS](https://sass-lang.com/)** - Improved syntax for CSS

**Other frameworks used for development:**

- **[npm](https://www.npmjs.com/)** - Node package manager, downloads and installs JavaScript packages
- **[Babel](https://babeljs.io/)** - Transpiler so newer JavaScript syntax works in older browsers
- **[ESLint](https://eslint.org/)** - Lints the TypeScript code to ensure good coding practices
- **[Prettier](https://prettier.io/)** - Automatically formats the code

<br />

## Compiling and Running

In the project directory, you can run:

```bash
npm start
```

This runs the app in the development mode on port `3030`.
Open [http://localhost:3030](http://localhost:3030) to view it in the browser.
The page will automatically reload if you make edits.
You will also see any lint errors in the console.

To compile the code for production, you can run:

```bash
npm run build
```

This command builds the app for production to the `build` folder.
It correctly bundles React in production mode and optimizes the build for the best performance.
The build is minified and the filenames include the hashes, all ready to be deployed!

<br />

## Environment Variables

To run the frontend, you will need to set several environment variables.
The easiest way is to use a `.env` file. You should either name the file:

- `.env.development` - Development environment variables
- `.env.production` - Production environment variables

You will then need to set the following environment values:

|             Variable             | Required |               Default Value                | Description                                                                                                       |
| :------------------------------: | :------: | :----------------------------------------: | :---------------------------------------------------------------------------------------------------------------- |
|      REACT_APP_API_BASE_URL      |    No    |                 `/api/v1`                  | URL to access the proxy for the API server and collectors. _See below for more details on configuring the proxy._ |
| REACT_APP_NOTIFICATIONS_BASE_URL |    No    | `ws://localhost:3010/api/v1/notifications` | Websocket URL to subscribe to the notification server                                                             |
|   REACT_APP_RECAPTCHA_SITE_KEY   | **Yes**  |                                            | Site key used by [Google reCAPTCHA](https://www.google.com/recaptcha/about/).                                     |

<br />

## Development Proxy

The development proxy is configured in `package.json`. By default, it assumes it can access the API servers and collectors under `localhost:3010`.
The API then uses `/api/v1` as a replacement for `localhost:3010/api/v1`. This can be changed by updating:

```json
{
  // ...
  "proxy": "http://localhost:3010"
  // ...
}
```

A basic development proxy can be configured using [NGINX](https://www.nginx.com/).
The code snippet below properly configures NGINX to access all services using `localhost:3010`.

```nginx
server {
        listen 3010;

        # Increase the reverse-proxy timeout
        proxy_connect_timeout 600;
        proxy_send_timeout 600;
        proxy_read_timeout 600;
        send_timeout 600;

        location /api/v1/ {
                proxy_pass http://localhost:3000/api/v1/;
        }

        location /api/v1/collector/1/ {
                proxy_pass http://localhost:3001/api/v1/collector/1/;
        }

        location /api/v1/collector/2/ {
                proxy_pass http://localhost:3002/api/v1/collector/2/;
        }

        location /api/v1/notifications {
                proxy_pass http://localhost:3005/api/v1/notifications;
                proxy_http_version 1.1;
                proxy_set_header Upgrade $http_upgrade;
                proxy_set_header Connection "Upgrade";
                proxy_set_header Host $host;
        }
}
```

Add this configuration as a file in `/etc/nginx/sites-available`, then create a symbolic link in `/etc/nginx/sites-enabled` to enable the configuration.
If NGINX is already running, use `sudo nginx -s reload` to reload the configuration in real time.

On a production server, the `REACT_APP_API_BASE_URL` will need to be updated to the public proxy URL running in the cloud.

<br />

## Codebase Structue

**Code Layout:**

- [`/src`](/src) - Main directory for all code files
- [`/public`](/public) - Other public files included in the website, like images, robots.txt, and the manifest

The main entry point for the entire application is `src/index.tsx`, which stores the Redux global state and loads the router.
This component also defines the [error boundary](https://reactjs.org/docs/error-boundaries.html) used for trapping JavaScript errors.
All of the routes for the application are defined in `src/components/Routes.tsx`.

**Subdirectories in `src/`:**

- [`/api`](/src/api) - Types used to simplify API requests to the backend server
- [`/components`](/src/components) - React components used to render the pages
- [`/helpers`](/src/helpers) - Miscellaneous functions and structures
- [`/models`](/src/models) - Defines the public JSON return types from the API server
- [`/notifications`](/src/notifications) - Functions and hooks for communicating with the notification server
- [`/protocol`](/src/protocol) - Functions specific to the electronic voting protocol
- [`/redux`](/src/redux) - Defines the global state for the application
- [`/semantic-ui`](/src/semantic-ui) - Custom [Semantic UI Theme](https://semantic-ui.com/usage/theming.html) for the website

**Subdirectories of `components/`:**

- [`/errorDialogs`](/src/components/errorDialogs) - Shared component for showing application errors
- [`/input`](/src/components/input) - Various custom input components, like text boxes
- [`/routes`](/src/components/routes) - The actual pages in the application
- [`/shared`](/src/components/shared) - Other components shared by multiple pages in the application

### Linting and Formatting

The code repository uses `TypeScript`, `ESLint`, and `Prettier` to lint the codebase and ensure a consistant format.
When working with this codebase, try to install plugins for these frameworks to automatically format your code when saving.
For example, [VSCode](https://code.visualstudio.com/) provides good integration through the following plugins:

- [TypeScript](https://code.visualstudio.com/docs/languages/typescript) - Support built automatically into VSCode
- [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
- [Prettier - Code Formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
- [SCSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=mrmlnc.vscode-scss)

### Notable Components

- `ConfirmDialog` - Used throughout the application to ask a yes-no question
- `ErrorBoundary` - Catches all errors thrown by the application
- `Flex` - Basically a `<div>` element with `display: flex` set in the style. Allows easy formatting of complex flexbox layouts.
- `TransitionList` - Apply a transition effect so children display one-by-one.

### Routes

All pages in the program are defined in `src/components/Routes.tsx`.

**General guidelines:**

- Routes are classified as either `logged out`, `logged in`, or `both`
- Each router entry is based on RouteProps from the React Router library. In general, routes define a path and a React component to render.
- Routes can also define a permission from the `Permission` enum needed to view a page.
- Each route is defined as a sub-folder inside `src/components/routes/`
- In general, the page is defined as `<Page>.tsx` with any actions as `<page>Actions.ts`. (_The component is capitalized, but the actions is lowercase_). Routes may define nested components if needed.
- Pages handle notifications events in `<page>Notifications.ts`.
- Changing pages should be handled by React Router `history.push()`, **NOT** by updating `document.location`.

State in pages is handled using React hooks, except in rare cases where stateful class components are used.
Most components store the global state using Redux, although trivial state values may be stored in local hooks.

For the most part, each route starts with the following three lines of code:

```javascript
// Set the title for the page
useTitle('Page Title');

// Some page-specific hook to reset the Redux state
useResetPageState();

// One or multiple page hooks to load data from the API server
useFetchPageData();
```

### Interfacing with API Server

The codebase provides a series of TypeScript interfaces and functions to assist with loading data from the backend API server.

**API Data Types:**

- `APIOption<T>` - Represents a type that is either loading or loaded. On failure to load, an error is thrown.
- `APIResult<T>` - Represents a type that is loading, success, or an error.

**API Functions:**

- `apiLoading()` - Create a new loading object
- `apiSome(data: T)` - Create a new loaded `APIOption<T>`
- `apiSuccess(data: T)` - Create a new success `APIResult<T>`
- `apiError(error: Error)` - Create a new error `APIResult<T>`

**API Constants:**

- `axiosApi` - Special instance of the [Axios Object](https://axios-http.com/docs/intro) that automatically refreshes expired JWT tokens.
- `resolveOption` - Use `Promise.then(...resolveOption)` to convert an Axios promise into `APIOption<T>`
- `resolveResult` - Use `Promise.then(...resolveResult)` to convert an Axios promise into `APIResult<T>`
- `resolveOptionUnwrapped` - Use `Promise.then(...resolveOptionUnwrapped)` to convert an Axios promise into `T`

Often, the application only cares if an API result loaded without caring about error handling.
This is the purpose of `APIOption<T>`, which either returns a loading or success response.
If an error occurs, it is thrown and caught by the `ErrorBoundary` component.

### Redux State Management

The [Redux JavaScript Library](https://redux.js.org/) defines a system for storing the global application state.
The global state is stored in an object called the `store`, with each page being a sub-property in the store object.
Any changes to the store must go through `reducers`, which define how the `store` changes for various actions.
Each `reducer` takes an `action`, which is just a plain JavaScript object, and changes the `store` to build the new state.
Each action is constructed using an `action creator`, which is a plain JavaScript function that returns an `action` object.

- `State` - Stores global state of program
- `Reducer` - Makes changes to state
- `Action` - Defines action for the reducer to run
- `Action Creator` - Builds the action objects

The global store itself is defined in [`/src/redux/store.ts`](/src/redux/store.ts), and the data type is defined in [`/src/redux/state/root.ts`](/src/redux/state/root.ts).
To isolate parts of the application, each page has a nested property inside the root state.
All of these nested states are defined in [`/src/redux/store`](/src/redux/store).

To simplify the use of Redux in the application, a universal action creator and reducer has been written to update any part of the state.
Rather, the application defines several higher-order functions for interacting with the store:

- `nestedSelectorHook(state: keyof RootState)` - Returns a function that can be used like [`useSelector`](https://react-redux.js.org/api/hooks), but with a nested state.
- `getNestedState(state: keyof RootState)` - Returns a function that can be used to get the nested state.
- `mergeNestedState(state: keyof RootState, object?: Partial<T>)` - If object is provided, merges the given properties in the nested state. If object is not provided, returns a higher-order function for merging the nested state.
- `setNestedState(state: keyof RootState, object?: T)` - If object is provided, sets the nested state. If object is not provided, returns a higher-order function for setting the nested state.
- `clearNestedState(state: keyof RootState, initial?: T)` - Clear the nested state inside the store. By default, it uses the original initial state.

In development mode, all of these functions are available from the global window object to help with debugging.

### Miscellaneous Types and Functions

- `passwordComplexity` - Constants for the password complexity checker
- `useTitle()` - Set the document title for the page
- `showConfirm()` - Show the yes-no confirmation dialog
- `isDev()` - Returns `true` if the server is running in development mode
