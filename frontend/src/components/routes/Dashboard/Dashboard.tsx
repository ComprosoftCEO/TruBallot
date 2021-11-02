import { DashboardMenu } from './DashboardMenu';
import { DashboardHome } from './DashboardHome';
import { ElectionsList } from './ElectionsList';
import { DashboardFilter, useClearState, useFetchAllElections } from './dashboardActions';

export interface DashboardProps {
  filter?: DashboardFilter;
}

export const Dashboard = ({ filter }: DashboardProps) => {
  useClearState();
  useFetchAllElections();

  return (
    <>
      <DashboardMenu />
      {filter === undefined ? <DashboardHome /> : <ElectionsList filter={filter} />}
    </>
  );
};
