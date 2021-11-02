import { useTitle } from 'helpers/title';
import { DashboardMenu } from './DashboardMenu';
import { DashboardHome } from './DashboardHome';
import { ElectionsList } from './ElectionsList';
import { DashboardFilter, getDashboardTitle, useClearState } from './dashboardActions';

export interface DashboardProps {
  filter?: DashboardFilter;
}

export const Dashboard = ({ filter }: DashboardProps) => {
  useTitle(getDashboardTitle(filter));
  useClearState();

  return (
    <>
      <DashboardMenu />
      {filter === undefined ? <DashboardHome /> : <ElectionsList filter={filter} />}
    </>
  );
};
