<script lang="ts">
import type { EntryResponse } from '$lib/bridge';
import { formatHappenedAt } from '$lib/utils/date';

interface Props {
  entry: EntryResponse;
}

let { entry }: Props = $props();
</script>

<article class="entry-detail">
  {#if entry.title}
    <h1 class="title">{entry.title}</h1>
  {:else}
    <h1 class="title muted">无标题</h1>
  {/if}

  <time class="time" datetime={String(entry.happenedAt)}>
    {formatHappenedAt(entry.happenedAt)}
  </time>

  {#if entry.isEncrypted}
    <span class="badge">已加密存储</span>
  {/if}

  <div class="content">{entry.content}</div>
</article>

<style>
.entry-detail {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.title {
  margin: 0;
  font-size: var(--text-title);
  font-weight: 600;
  line-height: 1.2;
  overflow-wrap: break-word;
}

.title.muted {
  color: var(--color-text-muted);
}

.time {
  font-size: var(--text-caption);
  color: var(--color-text-muted);
}

.badge {
  display: inline-block;
  width: fit-content;
  padding: 6px 12px;
  font-size: 0.875rem;
  color: var(--color-on-primary);
  background: var(--color-secondary);
  border-radius: var(--radius-lg);
}

.content {
  min-width: 0;
  padding: var(--space-lg);
  line-height: 1.7;
  overflow-wrap: break-word;
  white-space: pre-wrap;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-soft);
}
</style>
