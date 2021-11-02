import axios from 'axios';
import { applyAuthTokenInterceptor, TokenRefreshRequest, clearAuthTokens } from 'axios-jwt';
import { LoginResult } from 'models/auth';
import { API_BASE_URL } from 'env';
import { logInStore, logOutStore } from 'redux/auth';
import { resolveOptionUnwrapped } from 'api';

/**
 * Export Axios instances to access the API server and the collectors
 */
export const axiosApi = axios.create({ baseURL: API_BASE_URL });

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
    logInStore(response.data.clientToken);

    return {
      accessToken: response.data.clientToken,
      refreshToken: response.data.refreshToken,
    };
  } catch (error) {
    // Log out the user if some sort of refresh error occurs
    //   This prevents a nasty infinite loop if the API server ever goes down
    clearAuthTokens();
    logOutStore();

    throw error;
  }
};

// Add interceptor to the axios instance
applyAuthTokenInterceptor(axiosApi, { requestRefresh });
