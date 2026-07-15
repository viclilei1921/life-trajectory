<script lang="ts">
import { onMount } from 'svelte';
import AppShell from '$lib/components/AppShell.svelte';
import VaultGate from '$lib/components/VaultGate.svelte';
import { themeStore } from '$lib/stores/theme.svelte';
import { vaultStore } from '$lib/stores/vault.svelte';
import './layout.css';

let { children } = $props();

onMount(() => {
  themeStore.init();
  vaultStore.refresh();
});
</script>

{#if vaultStore.loading && !vaultStore.status}
  <div class="loading-screen">
    <p>加载中…</p>
  </div>
{:else if vaultStore.needsUnlock}
  <VaultGate />
{:else}
  <AppShell> {@render children()} </AppShell>
{/if}

<style>
.loading-screen {
  display: flex;
  align-items: center;
  justify-content: center;
  height: var(--app-height);
  color: var(--color-text-muted);
}
</style>
