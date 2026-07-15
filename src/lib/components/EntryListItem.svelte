<script lang="ts">
import type { EntrySummary } from '$lib/bridge';
import { formatHappenedAt } from '$lib/utils/date';

interface Props {
  entry: EntrySummary;
}

let { entry }: Props = $props();

const displayTitle = $derived(entry.title?.trim() || '无标题');
</script>

<a class="entry-item" href={`/entries/${entry.id}`}>
  <div class="meta">
    <time class="time" datetime={String(entry.happenedAt)}>
      {formatHappenedAt(entry.happenedAt)}
    </time>
    {#if entry.isEncrypted}
      <span class="badge">加密</span>
    {/if}
  </div>
  <h2 class="title">{displayTitle}</h2>
  {#if entry.excerpt}
    <p class="excerpt">{entry.excerpt}</p>
  {/if}
</a>

<style>
.entry-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  padding: var(--space-lg);
  color: inherit;
  text-decoration: none;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-soft);
  transition: border-color 0.15s ease;
}

.entry-item:hover {
  border-color: var(--color-border-strong);
}

.entry-item:focus-visible {
  outline: 2px solid var(--color-primary);
  outline-offset: 2px;
}

.meta {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
  align-items: center;
}

.time {
  font-size: var(--text-caption);
  color: var(--color-text-muted);
}

.badge {
  padding: 2px 8px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--color-on-primary);
  background: var(--color-secondary);
  border-radius: var(--radius-lg);
}

.title {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  line-height: 1.3;
  overflow-wrap: break-word;
}

.excerpt {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  margin: 0;
  overflow: hidden;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  font-size: var(--text-caption);
  line-height: 1.5;
  color: var(--color-text-muted);
  overflow-wrap: break-word;
}
</style>
