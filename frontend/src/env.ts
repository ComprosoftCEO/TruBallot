/*
 * All functions related to environment variables
 */

/// Test for development environment
export function isDev(): boolean {
  return !process.env.NODE_ENV || process.env.NODE_ENV === 'development';
}

// API server URLs
export const API_BASE_URL: string = process.env.REACT_APP_API_BASE_URL ?? '/api/v1';
export const NOTIFICATIONS_BASE_URL: string =
  process.env.REACT_APP_NOTIFICATIONS_BASE_URL ?? 'ws://localhost:3010/api/v1/notifications';

// reCAPTCHA site key must be set
//   https://www.google.com/recaptcha/about/
export const RECAPTCHA_SITE_KEY: string = process.env.REACT_APP_RECAPTCHA_SITE_KEY ?? '';
if (RECAPTCHA_SITE_KEY === '') {
  throw new Error('Recaptcha site key must be set');
}
