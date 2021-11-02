import axios from 'axios';
import jwt from 'jsonwebtoken';
import { applyAuthTokenInterceptor, TokenRefreshRequest, clearAuthTokens } from 'axios-jwt';
import { ClientToken, LoginResult } from 'models/auth';
import { API_BASE_URL } from 'env';
import { mergeNestedState } from 'redux/helpers';
import { resolveOptionUnwrapped } from 'api';

/**
 * Export Axios instances to access the API server and the collectors
 */
export const axiosApi = axios.create({ baseURL: API_BASE_URL });

const mergeState = mergeNestedState('globals');

// Define the refresh function
const requestRefresh: TokenRefreshRequest = async (refreshToken) => {
  try {
    const response = await axios
      .post<LoginResult>(`${API_BASE_URL}/auth/refresh`, undefined, {
        headers: {
          Authorization: `Bearer ${refreshToken}`,
        },
      })
      .then(...resolveOptionUnwrapped);

    // Update the store with the new JWT token
    const clientToken: ClientToken = jwt.decode(response.data.clientToken) as ClientToken;
    mergeState({
      isLoggedIn: true,
      name: clientToken.name,
      email: clientToken.email,
      permissions: new Set(clientToken.permissions),
    });

    return {
      accessToken: response.data.clientToken,
      refreshToken: response.data.refreshToken,
    };
  } catch (error) {
    // Log out the user if some sort of refresh error occurs
    //   This prevents a nasty infinite loop if the API server ever goes down
    clearAuthTokens();
    mergeState({
      isLoggedIn: false,
    });

    throw error;
  }
};

// Add interceptor to the axios instance
applyAuthTokenInterceptor(axiosApi, { requestRefresh });
