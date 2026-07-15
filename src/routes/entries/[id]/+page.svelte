<script lang="ts">
import { page } from '$app/stores';
import type { EntryResponse } from '$lib/bridge';
import { getEntry } from '$lib/bridge';
import EntryDetail from '$lib/components/EntryDetail.svelte';
import ErrorBanner from '$lib/components/ErrorBanner.svelte';
import { formatAppError } from '$lib/utils/errors';

let entry = $state<EntryResponse | null>(null);
let loading = $state(true);
let error = $state<string | null>(null);

$effect(() => {
  const id = $page.params.id;
  if (!id) {
    return;
  }

  loading = true;
  error = null;
  entry = null;

  getEntry(id)
    .then((result) => {
      entry = result;
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
  <title>{entry?.title ?? '记录详情'} · 生命轨迹</title>
</svelte:head>

<div class="page">
  <a href="/" class="back-link">← 返回记录</a>

  {#if loading}
    <p class="status">加载中…</p>
  {:else if error}
    <ErrorBanner message={error} />
  {:else if entry}
    <EntryDetail {entry} />
  {/if}
</div>

<style>
.back-link {
  display: inline-flex;
  align-items: center;
  min-height: var(--control-height);
  font-size: var(--text-body);
  font-weight: 600;
  color: var(--color-primary);
  text-decoration: none;
}

.back-link:hover {
  color: var(--color-primary-pressed);
  text-decoration: underline;
}

.status {
  margin: 0;
  color: var(--color-text-muted);
}
</style>
