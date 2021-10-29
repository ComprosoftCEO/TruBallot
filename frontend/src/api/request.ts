import axios, { AxiosError, AxiosResponse } from 'axios';
import { applyAuthTokenInterceptor, TokenRefreshRequest } from 'axios-jwt';
import { LoginResult } from 'models/auth';
import { ErrorResponse } from 'api/error';
import { mergeNestedState } from 'redux/helpers';

/**
 * A type that is either loading or a success
 *  Any errors throw a nasty React error
 */
export type APIOption<T> = APILoading | APISome<T>;
export type APIOptionUnwrapped<T> = APISome<T>;

/**
 * Errors that are stored and can be handled
 */
export type APIResult<T> = APILoading | APISuccess<T> | APIError;
export type APIResultUnwrapped<T> = APISuccess<T> | APIError;

export interface APILoading {
  loading: true;
}

export interface APISome<T> {
  loading: false;
  data: T;
}

export interface APISuccess<T> {
  loading: false;
  success: true;
  data: T;
}

export interface APIError {
  loading: false;
  success: false;
  error: RequestError;
}

export type RequestError = AxiosError<ErrorResponse>;

// Functions to construct the different types
export const API_LOADING: APILoading = { loading: true };

export const apiLoading = (): APILoading => API_LOADING;
export const apiSome = <T>(data: T): APISome<T> => ({ loading: false, data });
export const apiSuccess = <T>(data: T): APISuccess<T> => ({ loading: false, success: true, data });
export const apiError = (error: RequestError): APIError => ({ loading: false, success: false, error });

/**
 * Export Axios instances to access the API server and the collectors
 */
const BASE_URL = 'http://localhost:3000/api/v1';

export const axiosApi = axios.create({ baseURL: BASE_URL });
export const axiosCollector1 = axios.create({ baseURL: 'http://localhost:3001/api/v1/collector/1' });
export const axiosCollector2 = axios.create({ baseURL: 'http://localhost:3002/api/v2/collector/2' });

// Define the refresh function
const requestRefresh: TokenRefreshRequest = async (refreshToken) => {
  const response = await axios.post<LoginResult>(`${BASE_URL}/auth/refresh`, undefined, {
    headers: {
      Authorization: `Bearer ${refreshToken}`,
    },
  });

  return {
    accessToken: response.data.clientToken,
    refreshToken: response.data.refreshToken,
  };
};

// Add interceptor to your axios instance
applyAuthTokenInterceptor(axiosApi, { requestRefresh });
applyAuthTokenInterceptor(axiosCollector1, { requestRefresh });
applyAuthTokenInterceptor(axiosCollector2, { requestRefresh });

/**
 * Functions for handling API errors
 *
 * Usage:
 *
 * axios.get(input).then(...resolveOption) -> returns APIOption<T>
 * axios.get(input).then(...resolveResult) -> returns APIResult<T>
 */
function resolveSome<T>(resp: AxiosResponse<T>): APISome<T> {
  return { loading: false, data: resp.data };
}

function resolveSomeUnwrapped<T>(resp: AxiosResponse<T>): T {
  return resp.data;
}

function resolveNone(error: AxiosError<ErrorResponse>): any {
  mergeNestedState('globals', { globalError: error });
  throw error;
}

function resolveSuccess<T>(resp: AxiosResponse<T>): APISuccess<T> {
  return { loading: false, success: true, data: resp.data };
}

function resolveError(error: AxiosError<ErrorResponse>): APIError {
  return { loading: false, success: false, error };
}

export const resolveOption: [typeof resolveSome, typeof resolveNone] = [resolveSome, resolveNone];
export const resolveResult: [typeof resolveSuccess, typeof resolveError] = [resolveSuccess, resolveError];

// Very similar to "resolveOption", but returns T instead of APISome<T>
export const resolveOptionUnwrapped: [typeof resolveSomeUnwrapped, typeof resolveNone] = [
  resolveSomeUnwrapped,
  resolveNone,
];
