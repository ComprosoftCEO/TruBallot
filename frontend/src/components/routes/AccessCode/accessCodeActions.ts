import { useLayoutEffect } from 'react';
import { apiLoading, apiSuccess, axiosApi, resolveResult } from 'api';
import { GetElectionByAccessCode } from 'models/election';
import { clearNestedState, getNestedState, mergeNestedState } from 'redux/helpers';
import { history } from 'index';

const mergeState = mergeNestedState('accessCode');
const getState = getNestedState('accessCode');

export const useClearState = (): void => {
  useLayoutEffect(() => clearNestedState('accessCode'), []);
};

export const setCode = (newCode: string): void =>
  mergeState({ code: newCode.toUpperCase(), loadingElection: apiSuccess({}) });

export const activateCode = async (): Promise<void> => {
  const { code } = getState();

  mergeState({ loadingElection: apiLoading() });

  const result = await axiosApi
    .get<GetElectionByAccessCode>('/elections/access-code', { params: { code } })
    .then(...resolveResult);

  if (result.success) {
    history.push(`/elections/${result.data.id}`);
  } else {
    mergeState({ loadingElection: result });
  }
};
