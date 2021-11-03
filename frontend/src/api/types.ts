import { AxiosError, AxiosResponse } from 'axios';
import { mergeNestedState } from 'redux/helpers';
import { ErrorResponse } from './error';

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
  error: RequestError | Error;
}

export type RequestError = AxiosError<ErrorResponse>;

// Functions to construct the different types
export const API_LOADING: APILoading = { loading: true };

export const apiLoading = (): APILoading => API_LOADING;
export const apiSome = <T>(data: T): APISome<T> => ({ loading: false, data });
export const apiSuccess = <T>(data: T): APISuccess<T> => ({ loading: false, success: true, data });
export const apiError = (error: RequestError | Error): APIError => ({ loading: false, success: false, error });

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

function resolveNone(error: AxiosError<ErrorResponse>): never {
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
