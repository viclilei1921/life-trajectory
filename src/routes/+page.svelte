<script lang="ts">
import { goto } from '$app/navigation';
import type { EntrySummary } from '$lib/bridge';
import { listEntries } from '$lib/bridge';
import EntryListItem from '$lib/components/EntryListItem.svelte';
import ErrorBanner from '$lib/components/ErrorBanner.svelte';
import UiButton from '$lib/components/UiButton.svelte';
import { formatAppError } from '$lib/utils/errors';

let entries = $state<EntrySummary[]>([]);
let loading = $state(true);
let error = $state<string | null>(null);

$effect(() => {
  loading = true;
  error = null;

  listEntries()
    .then((result) => {
      entries = result;
    })
    .catch((e) => {
      error = formatAppError(e);
    })
    .finally(() => {
      loading = false;
    });
});
</script>

<svelte:head>
  <title>记录 · 生命轨迹</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1>记录</h1>
    <p class="subtitle">把零散时刻串成生命轨迹。</p>
  </header>

  {#if error}
    <ErrorBanner message={error} ondismiss={() => (error = null)} />
  {/if}

  {#if loading}
    <p class="status">加载中…</p>
  {:else if entries.length === 0}
    <section class="empty card">
      <h2 class="empty-title">还没有记录</h2>
      <p class="empty-desc">留下一个节点，比完美记录更重要。</p>
      <UiButton onclick={() => goto('/entries/new')}>写下第一条</UiButton>
    </section>
  {:else}
    <ul class="entry-list">
      {#each entries as entry (entry.id)}
        <li>
          <EntryListItem {entry} />
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
.status {
  margin: 0;
  color: var(--color-text-muted);
}

.empty {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  align-items: flex-start;
}

.empty-title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
}

.empty-desc {
  margin: 0;
  font-size: var(--text-caption);
  color: var(--color-text-muted);
}

.entry-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  padding: 0;
  margin: 0;
  list-style: none;
}
</style>
