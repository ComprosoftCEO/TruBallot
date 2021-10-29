import { AxiosError } from 'axios';

/**
 * Error response data structure
 */
export interface ErrorResponse {
  statusCode: number;
  description: string;
  errorCode: GlobalErrorCode;
  developerNotes?: string;
}

/**
 * List of all error codes that the API backend can return
 */
export enum GlobalErrorCode {
  UnknownError = 0,
  DatabaseConnectionError,
  DatabaseQueryError,
  MissingAppData,
  JSONPayloadError,
  FormPayloadError,
  URLPathError,
  QueryStringError,
  StructValidationError,
  InvalidEmailPassword,
  InvalidJWTToken,
  UserEmailExists,
  PasswordComplexityError,
  RecaptchaError,
  ForbiddenResourceAction,
  NoSuchResource,
  ElectionNotOwnedByUser,
  ElectionNotDraft,
  WrongElectionStatus,
  AccessCodeNotFound,
  NotRegistered,
  AlreadyRegistered,
  RegistrationClosed,
  NotEnoughRegistered,
  ElectionNotInitialized,
  CollectorURLNotSet,
  RegisterElectionError,
  VerificationError,
  AlreadyVoted,
  VerifyVoteError,
  VoteInvalid,
  NotOpenForVoting,
  NotEnoughVotes,
  CancelationSharesError,
  ElectionNotFinished,
  NoNotifyPermission,
  NoSubscribePermission,
  NotificationError,
}

/**
 * Convert any Axios error into error information
 */
export interface ErrorInformation {
  description: string;
  statusCode: number | null;
  errorCode: GlobalErrorCode | null;
}

/**
 * Extract error information from any API error object.
 *
 * @param e The input error object
 * @returns ErrorInformation
 */
export const getErrorInformation = (
  e: AxiosError<ErrorResponse> | ErrorResponse | Error | null | undefined,
): ErrorInformation => {
  // Handle null and undefined inputs
  if (e === null || e === undefined) {
    return {
      description: 'Unknown Error',
      statusCode: null,
      errorCode: null,
    };
  }

  // Test for ErrorResponse input
  if (!(e as AxiosError).isAxiosError) {
    const response = e as ErrorResponse;
    if (response.description !== undefined) {
      return {
        description: response.description,
        statusCode: response?.statusCode ?? null,
        errorCode: response?.errorCode ?? null,
      };
    }
  }

  // Test for Axios response in error
  const response = (e as AxiosError<ErrorResponse>)?.response;
  if (response !== undefined) {
    // Test for ErrorResponse in response body
    const { data } = response;
    if (data.description !== undefined) {
      return {
        description: data.description,
        statusCode: response.status,
        errorCode: data?.errorCode ?? null,
      };
    }

    // Return the status text of the response error
    return {
      description: `${response.status} ${response.statusText}`,
      statusCode: response.status,
      errorCode: null,
    };
  }

  // Treat as a normal error object
  return {
    description: (e as Error).toString(),
    statusCode: null,
    errorCode: null,
  };
};
