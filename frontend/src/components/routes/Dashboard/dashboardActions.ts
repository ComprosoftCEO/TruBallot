import { useEffect, useLayoutEffect } from 'react';
import { clearAuthTokens } from 'axios-jwt';
import pluralize from 'pluralize';
import { clearNestedState, mergeNestedState, nestedSelectorHook } from 'redux/helpers';
import { apiLoading, APIOption, apiSome, axiosApi, resolveOption } from 'api';
import { AllElectionsResult, ElectionStatus, PublicElectionList } from 'models/election';
import { history } from 'index';

const mergeGlobalsState = mergeNestedState('globals');
const mergeDashboardState = mergeNestedState('dashboard');
const useDashboardSelector = nestedSelectorHook('dashboard');

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('dashboard'), []);
};

export const useFetchAllElections = () => {
  useEffect(() => {
    mergeDashboardState(
      axiosApi
        .get<AllElectionsResult>('/elections')
        .then(...resolveOption)
        .then((data) => ({ data })),
    );
  }, []);
};

//
// All filters used by the dashboard
//
export enum DashboardFilter {
  MyElectionsAll,
  MyElectionsDraft,
  MyElectionsOpen,
  MyElectionsVoting,
  MyElectionsClosed,

  PublicElectionsAll,
  PublicElectionsOpen,
  PublicElectionsVoting,
  PublicElectionsClosed,

  RegistrationsAll,
  RegistrationsOpen,
  RegistrationsVoting,
  RegistrationsClosed,
}

type FilterFunction = (input: AllElectionsResult) => PublicElectionList[];

const DRAFT_STATUS: ElectionStatus[] = [ElectionStatus.Draft, ElectionStatus.InitFailed];
const OPEN_STATUS: ElectionStatus[] = [ElectionStatus.Registration];
const VOTING_STATUS: ElectionStatus[] = [ElectionStatus.Voting];
const CLOSED_STATUS: ElectionStatus[] = [ElectionStatus.Finished, ElectionStatus.CollectionFailed];

const filterMyElectionsAll: FilterFunction = (input) => input.userElections;
const filterMyElectionsDraft: FilterFunction = (input) =>
  input.userElections.filter((e) => DRAFT_STATUS.includes(e.status));
const filterMyElectionsOpen: FilterFunction = (input) =>
  input.userElections.filter((e) => OPEN_STATUS.includes(e.status));
const filterMyElectionsVoting: FilterFunction = (input) =>
  input.userElections.filter((e) => VOTING_STATUS.includes(e.status));
const filterMyElectionsClosed: FilterFunction = (input) =>
  input.userElections.filter((e) => CLOSED_STATUS.includes(e.status));

const filterPublicElectionsAll: FilterFunction = (input) => input.publicElections;
const filterPublicElectionsOpen: FilterFunction = (input) =>
  input.publicElections.filter((e) => OPEN_STATUS.includes(e.status));
const filterPublicElectionsVoting: FilterFunction = (input) =>
  input.publicElections.filter((e) => VOTING_STATUS.includes(e.status));
const filterPublicElectionsClosed: FilterFunction = (input) =>
  input.publicElections.filter((e) => CLOSED_STATUS.includes(e.status));

const filterRegistrationsAll: FilterFunction = (input) => input.registeredElections;
const filterRegistrationsOpen: FilterFunction = (input) =>
  input.registeredElections.filter((e) => OPEN_STATUS.includes(e.status));
const filterRegistrationsVoting: FilterFunction = (input) =>
  input.registeredElections.filter((e) => VOTING_STATUS.includes(e.status));
const filterRegistrationsClosed: FilterFunction = (input) =>
  input.registeredElections.filter((e) => CLOSED_STATUS.includes(e.status));

const ALL_FILTERS: Record<DashboardFilter, FilterFunction> = {
  [DashboardFilter.MyElectionsAll]: filterMyElectionsAll,
  [DashboardFilter.MyElectionsDraft]: filterMyElectionsDraft,
  [DashboardFilter.MyElectionsOpen]: filterMyElectionsOpen,
  [DashboardFilter.MyElectionsVoting]: filterMyElectionsVoting,
  [DashboardFilter.MyElectionsClosed]: filterMyElectionsClosed,

  [DashboardFilter.PublicElectionsAll]: filterPublicElectionsAll,
  [DashboardFilter.PublicElectionsOpen]: filterPublicElectionsOpen,
  [DashboardFilter.PublicElectionsVoting]: filterPublicElectionsVoting,
  [DashboardFilter.PublicElectionsClosed]: filterPublicElectionsClosed,

  [DashboardFilter.RegistrationsAll]: filterRegistrationsAll,
  [DashboardFilter.RegistrationsOpen]: filterRegistrationsOpen,
  [DashboardFilter.RegistrationsVoting]: filterRegistrationsVoting,
  [DashboardFilter.RegistrationsClosed]: filterRegistrationsClosed,
};

/**
 * Get the list of filtered elections from the store given the filter
 *
 * @param filter Filter to apply to the list
 * @returns List of elections
 */
export const useFilteredElections = (filter?: DashboardFilter): APIOption<PublicElectionList[]> => {
  const publicElections = useDashboardSelector((state) => state.data);
  if (publicElections.loading) {
    return apiLoading();
  }

  // Apply the filter
  const filteredList: PublicElectionList[] =
    filter && ALL_FILTERS[filter] ? ALL_FILTERS[filter](publicElections.data) : [];

  return apiSome(filteredList);
};

/**
 * Log out of the web application
 */
export const logOut = () => {
  clearAuthTokens();
  mergeGlobalsState({ isLoggedIn: false });
  history.push('/');
};

/**
 * List of all headers for the various list types
 */
const ALL_TITLES: Record<DashboardFilter, string> = {
  [DashboardFilter.MyElectionsAll]: 'My Elections',
  [DashboardFilter.MyElectionsDraft]: 'My Elections (Drafts)',
  [DashboardFilter.MyElectionsOpen]: 'My Elections (Open)',
  [DashboardFilter.MyElectionsVoting]: 'My Elections (Voting)',
  [DashboardFilter.MyElectionsClosed]: 'My Elections (Closed)',

  [DashboardFilter.PublicElectionsAll]: 'Public Elections',
  [DashboardFilter.PublicElectionsOpen]: 'Public Elections (Open)',
  [DashboardFilter.PublicElectionsVoting]: 'Public Elections (Voting)',
  [DashboardFilter.PublicElectionsClosed]: 'Public Elections (Closed)',

  [DashboardFilter.RegistrationsAll]: 'Registered Elections',
  [DashboardFilter.RegistrationsOpen]: 'Registered Elections (Open)',
  [DashboardFilter.RegistrationsVoting]: 'Registered Elections (Voting)',
  [DashboardFilter.RegistrationsClosed]: 'Registered Elections (Closed)',
};

export const getListHeader = (filter: DashboardFilter): string => ALL_TITLES[filter];

/**
 * Get the text to show in the "meta" field of the card
 *
 * Specifically, this doesn't list the number of registrations before the election is published
 *
 * @param election Input election
 * @returns Meta string
 */
export const getCardMetaText = (election: PublicElectionList): string =>
  [ElectionStatus.Draft, ElectionStatus.Draft].includes(election.status)
    ? pluralize('Question', election.numQuestions, true)
    : `${pluralize('Question', election.numQuestions, true)}, ${election.numRegistered} Registered`;

/**
 * Should we show the "Create Election" card on this screen?
 *
 * @param filter Filter to apply to the data
 */
export const showCreateCard = (filter: DashboardFilter): boolean =>
  [DashboardFilter.MyElectionsAll, DashboardFilter.MyElectionsDraft].includes(filter);
