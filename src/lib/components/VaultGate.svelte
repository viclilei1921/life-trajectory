<script lang="ts">
import ErrorBanner from '$lib/components/ErrorBanner.svelte';
import UiButton from '$lib/components/UiButton.svelte';
import UiInput from '$lib/components/UiInput.svelte';
import { vaultStore } from '$lib/stores/vault.svelte';

let password = $state('');
let submitting = $state(false);

async function handleUnlock() {
  if (!password.trim()) {
    return;
  }
  submitting = true;
  try {
    await vaultStore.unlock(password);
    password = '';
  } catch {
    // error shown via vaultStore.error
  } finally {
    submitting = false;
  }
}
</script>

<div class="vault-gate">
  <div class="card">
    <h1 class="title">解锁保险库</h1>
    <p class="desc">你的记录已加密保护，请输入密码以继续。</p>

    {#if vaultStore.error}
      <ErrorBanner message={vaultStore.error} ondismiss={() => (vaultStore.error = null)} />
    {/if}

    <form class="form" onsubmit={(e) => { e.preventDefault(); handleUnlock(); }}>
      <UiInput type="password" label="密码" placeholder="输入保险库密码" bind:value={password} />
      <UiButton type="submit" disabled={submitting || !password.trim()}>
        {submitting ? '解锁中…' : '解锁'}
      </UiButton>
    </form>
  </div>
</div>

<style>
.vault-gate {
  position: fixed;
  inset: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: calc(var(--space-lg) + var(--safe-top)) var(--page-gutter) calc(var(--space-lg) + var(--safe-bottom));
  background: var(--color-bg);
}

.card {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  width: 100%;
  max-width: 400px;
  padding: var(--space-xl) var(--space-lg);
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-soft);
}

.title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  line-height: 1.2;
}

.desc {
  margin: 0;
  font-size: 0.9375rem;
  color: var(--color-text-muted);
}

.form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

@media (min-width: 768px) {
  .card {
    padding: var(--space-xl);
  }
}

.form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}
</style>
