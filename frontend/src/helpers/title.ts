import { useLayoutEffect } from 'react';

const BASE_TITLE = 'TruBallot: An Open Way To Vote';

const buildTitle = (title: string) => (title.trim().length > 0 ? `${title.trim()} | ${BASE_TITLE}` : BASE_TITLE);

export const useTitle = (title = '') =>
  useLayoutEffect(() => {
    document.title = buildTitle(title);
  }, [title]);

export const setTitle = (title = '') => {
  document.title = buildTitle(title);
};
