import { history } from 'index';
import { LastLocationType } from 'react-router-last-location';

/**
 * Go back to the previous page
 *
 * @param lastLocation React hook with for use with "react-router-last-location" library
 * @param defaultLocation Default location to navigate to if no page is found
 *                        (Set to "/" if not provided to go to the home page)
 */
export const goBack = (lastLocation: LastLocationType, defaultLocation = '/'): void => {
  if (lastLocation === null) {
    history.push(defaultLocation);
  } else {
    history.goBack();
  }
};
