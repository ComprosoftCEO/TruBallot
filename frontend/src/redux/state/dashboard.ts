/*
 * Dashboard state
 */
import { apiLoading, APIOption } from 'api';
import { AllElectionsResult } from 'models/election';

export interface DashboardState {
  data: APIOption<AllElectionsResult>;
}

export const initialDashboardState: DashboardState = {
  data: apiLoading(),
};
