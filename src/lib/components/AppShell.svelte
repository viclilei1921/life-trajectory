<script lang="ts">
import type { Snippet } from 'svelte';
import { page } from '$app/stores';

interface Props {
  children: Snippet;
}

interface NavItem {
  href: string;
  label: string;
  match: 'records' | 'write' | 'settings';
}

let { children }: Props = $props();

const navItems: NavItem[] = [
  { href: '/', label: '记录', match: 'records' },
  { href: '/entries/new', label: '写下', match: 'write' },
  { href: '/settings', label: '设置', match: 'settings' }
];

function isActive(match: NavItem['match'], pathname: string): boolean {
  if (match === 'write') {
    return pathname === '/entries/new' || pathname.startsWith('/entries/new/');
  }
  if (match === 'settings') {
    return pathname.startsWith('/settings');
  }
  // records: home + entry detail (not create)
  return pathname === '/' || (pathname.startsWith('/entries/') && !pathname.startsWith('/entries/new'));
}
</script>

<div class="app-shell">
  <aside class="sidebar" aria-label="主导航">
    <a href="/" class="brand">生命轨迹</a>
    <nav class="nav nav-primary">
      {#each navItems.filter((i) => i.match !== 'settings') as item (item.href)}
        <a
          href={item.href}
          class="nav-link"
          class:active={isActive(item.match, $page.url.pathname)}
          aria-current={isActive(item.match, $page.url.pathname) ? 'page' : undefined}
        >
          {item.label}
        </a>
      {/each}
    </nav>
    <nav class="nav nav-footer">
      {#each navItems.filter((i) => i.match === 'settings') as item (item.href)}
        <a
          href={item.href}
          class="nav-link"
          class:active={isActive(item.match, $page.url.pathname)}
          aria-current={isActive(item.match, $page.url.pathname) ? 'page' : undefined}
        >
          {item.label}
        </a>
      {/each}
    </nav>
  </aside>

  <div class="content-column scrollbar-custom">
    <header class="mobile-header">
      <a href="/" class="brand">生命轨迹</a>
    </header>

    <main class="main">
      {@render children()}
    </main>
  </div>

  <nav class="bottom-nav" aria-label="主导航">
    {#each navItems as item (item.href)}
      <a
        href={item.href}
        class="bottom-link"
        class:active={isActive(item.match, $page.url.pathname)}
        aria-current={isActive(item.match, $page.url.pathname) ? 'page' : undefined}
      >
        {item.label}
      </a>
    {/each}
  </nav>
</div>

<style>
.app-shell {
  display: flex;
  flex-direction: column;
  height: var(--app-height);
  padding-top: var(--safe-top);
  /* reserve space for fixed bottom nav on mobile */
  padding-bottom: calc(var(--bottom-nav-height) + var(--safe-bottom));
  overflow: hidden;
}

.sidebar {
  display: none;
}

.content-column {
  display: flex;
  flex: 1;
  flex-direction: column;
  width: 100%;
  min-width: 0;
  min-height: 0;
  overflow-x: hidden;
  overflow-y: auto;
}

.mobile-header {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  height: var(--header-height);
  padding: 0 var(--page-gutter);
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.brand {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-text);
  text-decoration: none;
}

.main {
  flex: 1;
  width: 100%;
  min-width: 0;
  max-width: var(--max-content-width);
  padding: var(--space-lg) var(--page-gutter);
  margin: 0 auto;
  overflow-wrap: break-word;
}

.bottom-nav {
  position: fixed;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 50;
  display: flex;
  gap: var(--space-xs);
  align-items: stretch;
  justify-content: space-around;
  min-height: calc(var(--bottom-nav-height) + var(--safe-bottom));
  padding: 0 var(--space-sm) var(--safe-bottom);
  background: var(--color-surface);
  border-top: 1px solid var(--color-border);
}

.bottom-link {
  display: inline-flex;
  flex: 1;
  align-items: center;
  justify-content: center;
  min-height: var(--bottom-nav-height);
  padding: var(--space-xs) var(--space-sm);
  font-size: var(--text-body);
  font-weight: 400;
  color: var(--color-text-muted);
  text-decoration: none;
  border-top: 2px solid transparent;
}

.bottom-link.active {
  font-weight: 600;
  color: var(--color-text);
  border-top-color: var(--color-primary);
}

.nav-link {
  display: flex;
  align-items: center;
  min-height: var(--control-height);
  padding: var(--space-xs) var(--space-md);
  font-size: var(--text-body);
  color: var(--color-text);
  text-decoration: none;
  border-left: 3px solid transparent;
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
}

.nav-link:hover {
  color: var(--color-primary);
  background: var(--color-bg);
}

.nav-link.active {
  font-weight: 600;
  color: var(--color-text);
  background: var(--color-bg);
  border-left-color: var(--color-primary);
}

@media (min-width: 768px) {
  .app-shell {
    flex-direction: row;
    padding-top: 0;
    padding-bottom: 0;
  }

  .sidebar {
    display: flex;
    flex-shrink: 0;
    flex-direction: column;
    gap: var(--space-md);
    width: var(--sidebar-width);
    height: 100%;
    padding: var(--space-lg) var(--space-sm) var(--space-lg) var(--space-md);
    overflow: hidden;
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
  }

  .sidebar .brand {
    display: block;
    padding: var(--space-sm) var(--space-md);
    margin-bottom: var(--space-sm);
  }

  .nav {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .nav-primary {
    flex: 1;
    min-height: 0;
  }

  .nav-footer {
    margin-top: auto;
  }

  .mobile-header,
  .bottom-nav {
    display: none;
  }

  .main {
    padding: var(--space-xl) var(--page-gutter);
  }
}
</style>
