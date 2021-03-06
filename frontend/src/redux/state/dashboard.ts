/*
 * State used by the main dashboard
 */
import { apiLoading, APIResult } from 'api';
import { AllElectionsResult } from 'models/election';

export interface DashboardState {
  data: APIResult<AllElectionsResult>;
}

export const initialDashboardState: DashboardState = {
  data: apiLoading(),
};
