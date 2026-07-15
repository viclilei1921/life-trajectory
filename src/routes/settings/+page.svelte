<script lang="ts">
import ErrorBanner from '$lib/components/ErrorBanner.svelte';
import UiButton from '$lib/components/UiButton.svelte';
import UiInput from '$lib/components/UiInput.svelte';
import { type ThemePreference, themeStore } from '$lib/stores/theme.svelte';
import { vaultStore } from '$lib/stores/vault.svelte';
import { formatAppError } from '$lib/utils/errors';

const themeOptions: { value: ThemePreference; label: string }[] = [
  { value: 'light', label: '浅色' },
  { value: 'dark', label: '深色' },
  { value: 'system', label: '跟随系统' }
];

let setupPassword = $state('');
let setupConfirm = $state('');
let currentPassword = $state('');
let newPassword = $state('');
let newConfirm = $state('');
let actionError = $state<string | null>(null);
let actionLoading = $state(false);

const modeLabel = $derived(vaultStore.status?.mode === 'password' ? '密码保护' : '无加密');

const unlockLabel = $derived(vaultStore.status?.isUnlocked ? '已解锁' : '已锁定');

async function handleSetupPassword() {
  if (setupPassword.length < 4) {
    actionError = '密码至少需要 4 个字符';
    return;
  }
  if (setupPassword !== setupConfirm) {
    actionError = '两次输入的密码不一致';
    return;
  }

  actionLoading = true;
  actionError = null;
  try {
    await vaultStore.setupPassword(setupPassword);
    setupPassword = '';
    setupConfirm = '';
  } catch (e) {
    actionError = formatAppError(e);
  } finally {
    actionLoading = false;
  }
}

async function handleLock() {
  actionLoading = true;
  actionError = null;
  try {
    await vaultStore.lock();
  } catch (e) {
    actionError = formatAppError(e);
  } finally {
    actionLoading = false;
  }
}

async function handleChangePassword() {
  if (newPassword.length < 4) {
    actionError = '密码至少需要 4 个字符';
    return;
  }
  if (newPassword !== newConfirm) {
    actionError = '两次输入的密码不一致';
    return;
  }

  actionLoading = true;
  actionError = null;
  try {
    await vaultStore.changePassword(currentPassword, newPassword);
    currentPassword = '';
    newPassword = '';
    newConfirm = '';
  } catch (e) {
    actionError = formatAppError(e);
  } finally {
    actionLoading = false;
  }
}
</script>

<svelte:head>
  <title>设置 · 生命轨迹</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1>保险库设置</h1>
    <p class="subtitle">管理外观、数据加密与访问保护</p>
  </header>

  {#if vaultStore.error}
    <ErrorBanner message={vaultStore.error} ondismiss={() => (vaultStore.error = null)} />
  {/if}

  {#if actionError}
    <ErrorBanner message={actionError} ondismiss={() => (actionError = null)} />
  {/if}

  <section class="card">
    <h2 class="section-title">外观</h2>
    <p class="section-desc">选择浅色或深色主题。当前生效：{themeStore.resolved === 'dark' ? '深色' : '浅色'}。</p>
    <div class="theme-options">
      {#each themeOptions as option (option.value)}
        <button
          type="button"
          class="theme-option"
          class:active={themeStore.preference === option.value}
          aria-pressed={themeStore.preference === option.value}
          onclick={() => themeStore.setPreference(option.value)}
        >
          {option.label}
        </button>
      {/each}
    </div>
  </section>

  <section class="card">
    <h2 class="section-title">当前状态</h2>
    <dl class="status-list">
      <div class="status-row">
        <dt>保护模式</dt>
        <dd>{modeLabel}</dd>
      </div>
      <div class="status-row">
        <dt>访问状态</dt>
        <dd>{unlockLabel}</dd>
      </div>
      <div class="status-row">
        <dt>加密版本</dt>
        <dd>{vaultStore.status?.cryptoVersion ?? '—'}</dd>
      </div>
    </dl>
  </section>

  {#if vaultStore.status?.mode === 'none'}
    <section class="card">
      <h2 class="section-title">设置密码保护</h2>
      <p class="section-desc">启用后，所有记录将以加密方式存储，重启应用需输入密码解锁。</p>
      <form class="form" onsubmit={(e) => { e.preventDefault(); handleSetupPassword(); }}>
        <UiInput type="password" label="新密码" bind:value={setupPassword} />
        <UiInput type="password" label="确认密码" bind:value={setupConfirm} />
        <UiButton type="submit" disabled={actionLoading || !setupPassword || !setupConfirm}>
          {actionLoading ? '设置中…' : '启用密码保护'}
        </UiButton>
      </form>
    </section>
  {:else if vaultStore.status?.isUnlocked}
    <section class="card">
      <h2 class="section-title">安全操作</h2>
      <div class="actions">
        <UiButton variant="secondary" disabled={actionLoading} onclick={handleLock}> 锁定保险库 </UiButton>
      </div>
    </section>

    <section class="card">
      <h2 class="section-title">修改密码</h2>
      <p class="section-desc">验证当前密码后设置新密码。忘记密码将无法恢复数据访问。</p>
      <form class="form" onsubmit={(e) => { e.preventDefault(); handleChangePassword(); }}>
        <UiInput type="password" label="当前密码" bind:value={currentPassword} />
        <UiInput type="password" label="新密码" bind:value={newPassword} />
        <UiInput type="password" label="确认新密码" bind:value={newConfirm} />
        <UiButton type="submit" disabled={actionLoading || !currentPassword || !newPassword || !newConfirm}>
          {actionLoading ? '修改中…' : '修改密码'}
        </UiButton>
      </form>
    </section>
  {:else}
    <section class="card">
      <p class="section-desc">保险库已锁定，请在启动时输入密码解锁。</p>
    </section>
  {/if}
</div>

<style>
.section-title {
  margin: 0 0 var(--space-sm);
  font-size: var(--text-body);
  font-weight: 600;
}

.section-desc {
  margin: 0 0 var(--space-md);
  font-size: var(--text-caption);
  color: var(--color-text-muted);
}

.theme-options {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.theme-option {
  min-height: var(--control-height);
  padding: 8px 20px;
  font: inherit;
  font-size: var(--text-body);
  font-weight: 600;
  color: var(--color-text);
  cursor: pointer;
  background: var(--color-surface);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-sm);
}

.theme-option.active {
  color: var(--color-surface);
  background: var(--color-text);
  border-color: var(--color-text);
}

.status-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  margin: 0;
}

.status-row {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
  font-size: 0.9375rem;
}

.status-row dt {
  margin: 0;
  color: var(--color-text-muted);
}

.status-row dd {
  margin: 0;
  font-weight: 600;
}

.form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.actions :global(.ui-button) {
  width: 100%;
}

@media (min-width: 768px) {
  .theme-options {
    flex-direction: row;
    flex-wrap: wrap;
  }

  .theme-option {
    border-radius: 9999px;
  }

  .status-row {
    flex-direction: row;
    gap: var(--space-md);
    justify-content: space-between;
  }

  .actions {
    flex-direction: row;
    flex-wrap: wrap;
  }

  .actions :global(.ui-button) {
    width: auto;
  }
}
</style>
