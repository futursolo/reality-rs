import 'global-jsdom/register';
import { screen, within } from '@testing-library/dom';

const realityExports = {
  screen,
  within,
} as const;

declare module globalThis {
  let __realityExports: typeof realityExports;
}

globalThis.__realityExports = realityExports;

// web_sys also checks if globalThis is indeed instanceof Window.
Object.setPrototypeOf(globalThis, Window.prototype);
// We disable colors for now, this should be fixed in the future.
process.env.COLORS = 'false';

export {};
