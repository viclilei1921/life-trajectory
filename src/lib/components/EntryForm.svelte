<script lang="ts">
import { createEntry } from '$lib/bridge';
import UiButton from '$lib/components/UiButton.svelte';
import UiInput from '$lib/components/UiInput.svelte';
import UiTextarea from '$lib/components/UiTextarea.svelte';
import { fromDatetimeLocal, nowUnix, toDatetimeLocal } from '$lib/utils/date';
import { formatAppError } from '$lib/utils/errors';

interface Props {
  onsuccess?: (id: string) => void;
}

let { onsuccess }: Props = $props();

let title = $state('');
let content = $state('');
let happenedAtLocal = $state(toDatetimeLocal(nowUnix()));
let submitting = $state(false);
let error = $state<string | null>(null);

async function handleSubmit() {
  if (!content.trim()) {
    error = '请填写记录内容';
    return;
  }

  submitting = true;
  error = null;
  try {
    const entry = await createEntry({
      title: title.trim() || undefined,
      content: content.trim(),
      happenedAt: fromDatetimeLocal(happenedAtLocal)
    });
    onsuccess?.(entry.id);
  } catch (e) {
    error = formatAppError(e);
  } finally {
    submitting = false;
  }
}
</script>

<form class="entry-form" onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
  <UiInput label="标题（可选）" placeholder="给这条记录起个名字" bind:value={title} />
  <UiTextarea label="内容" placeholder="写下此刻的想法、见闻或感受…" rows={8} bind:value={content} />
  <UiInput type="datetime-local" label="发生时间" bind:value={happenedAtLocal} />

  {#if error}
    <p class="error" role="alert">{error}</p>
  {/if}

  <UiButton type="submit" disabled={submitting || !content.trim()}>
    {submitting ? '保存中…' : '保存记录'}
  </UiButton>
</form>

<style>
.entry-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.error {
  margin: 0;
  font-size: 0.875rem;
  color: var(--color-error-text);
}
</style>
