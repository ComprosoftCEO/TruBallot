import axios from 'axios';
import { applyAuthTokenInterceptor, TokenRefreshRequest } from 'axios-jwt';
import { LoginResult } from 'models/auth';
import { API_BASE_URL } from 'env';

/**
 * Export Axios instances to access the API server and the collectors
 */
export const axiosApi = axios.create({ baseURL: API_BASE_URL });

// Define the refresh function
const requestRefresh: TokenRefreshRequest = async (refreshToken) => {
  const response = await axios.post<LoginResult>(`${API_BASE_URL}/auth/refresh`, undefined, {
    headers: {
      Authorization: `Bearer ${refreshToken}`,
    },
  });

  return {
    accessToken: response.data.clientToken,
    refreshToken: response.data.refreshToken,
  };
};

// Add interceptor to the axios instance
applyAuthTokenInterceptor(axiosApi, { requestRefresh });
