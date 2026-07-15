const STORAGE_KEY = 'life-trajectory-theme';

export type ThemePreference = 'light' | 'dark' | 'system';
export type ResolvedTheme = 'light' | 'dark';

function readPreference(): ThemePreference {
  if (typeof localStorage === 'undefined') {
    return 'system';
  }
  const stored = localStorage.getItem(STORAGE_KEY);
  if (stored === 'light' || stored === 'dark' || stored === 'system') {
    return stored;
  }
  return 'system';
}

function resolveTheme(preference: ThemePreference): ResolvedTheme {
  if (preference === 'light' || preference === 'dark') {
    return preference;
  }
  if (typeof matchMedia === 'undefined') {
    return 'light';
  }
  return matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(resolved: ResolvedTheme) {
  if (typeof document === 'undefined') {
    return;
  }
  document.documentElement.setAttribute('data-theme', resolved);
  document.documentElement.style.colorScheme = resolved;
}

/**
 * @description 亮/暗主题偏好
 */
class ThemeStore {
  preference = $state<ThemePreference>('system');
  resolved = $state<ResolvedTheme>('light');

  #media: MediaQueryList | null = null;
  #onMediaChange = () => {
    if (this.preference === 'system') {
      this.#sync();
    }
  };

  init() {
    this.preference = readPreference();
    this.#sync();
    if (typeof matchMedia !== 'undefined') {
      this.#media = matchMedia('(prefers-color-scheme: dark)');
      this.#media.addEventListener('change', this.#onMediaChange);
    }
  }

  setPreference(preference: ThemePreference) {
    this.preference = preference;
    localStorage.setItem(STORAGE_KEY, preference);
    this.#sync();
  }

  #sync() {
    this.resolved = resolveTheme(this.preference);
    applyTheme(this.resolved);
  }
}

export const themeStore = new ThemeStore();
