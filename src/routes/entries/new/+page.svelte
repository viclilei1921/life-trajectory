<script lang="ts">
import { goto } from '$app/navigation';
import EntryForm from '$lib/components/EntryForm.svelte';

function handleCreateSuccess(id: string) {
  goto(`/entries/${id}`);
}
</script>

<svelte:head>
  <title>写下 · 生命轨迹</title>
</svelte:head>

<div class="page">
  <header class="page-header">
    <h1>写下</h1>
    <p class="subtitle">留下一个节点，比完美记录更重要。</p>
  </header>

  <section class="card">
    <EntryForm onsuccess={handleCreateSuccess} />
  </section>
</div>
