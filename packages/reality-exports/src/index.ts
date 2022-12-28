import 'global-jsdom/register';
// import { screen, within } from '@testing-library/dom';

const realityExports = {
  // screen,
  // within,
  test: 2,
} as const;

declare module globalThis {
  let __realityExports: typeof realityExports;
}

globalThis.__realityExports = realityExports;

export {};
