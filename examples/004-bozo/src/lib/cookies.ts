const MAX_AGE_SECONDS = 60 * 60 * 24 * 30; // 30 days

export const EPOCH_COOKIE_PREFIX = 'bozo-last-epoch-';

export function makeEpochCookieName(address: string) {
  return `${EPOCH_COOKIE_PREFIX}${address.toLowerCase()}`;
}

export function readCookie(name: string): string | null {
  if (typeof document === 'undefined') {
    return null;
  }

  const cookieEntry = document.cookie
    .split('; ')
    .find((cookie) => cookie.startsWith(`${name}=`));

  if (!cookieEntry) {
    return null;
  }

  const [, value = ''] = cookieEntry.split('=');
  return decodeURIComponent(value);
}

export function writeCookie(name: string, value: string, maxAgeSeconds: number = MAX_AGE_SECONDS) {
  if (typeof document === 'undefined') {
    return;
  }

  document.cookie = `${name}=${encodeURIComponent(value)}; path=/; max-age=${maxAgeSeconds}; SameSite=Strict`;
}
