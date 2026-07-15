<script lang="ts">
interface Props {
  id?: string;
  label?: string;
  placeholder?: string;
  rows?: number;
  value?: string;
  disabled?: boolean;
}

let { id, label, placeholder, rows = 6, value = $bindable(''), disabled = false }: Props = $props();

const fallbackId = `ui-textarea-${crypto.randomUUID()}`;
const textareaId = $derived(id ?? fallbackId);
</script>

<div class="ui-field">
  {#if label}
    <!-- biome-ignore lint/a11y/noLabelWithoutControl: label uses for/id to associate with textarea -->
    <label class="ui-label" for={textareaId}>{label}</label>
  {/if}
  <textarea id={textareaId} {rows} {placeholder} {disabled} bind:value class="ui-textarea"></textarea>
</div>

<style>
.ui-field {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.ui-label {
  font-size: 0.875rem;
  color: var(--color-text-muted);
}

.ui-textarea {
  box-sizing: border-box;
  width: 100%;
  padding: 12px 16px;
  font: inherit;
  font-size: 1rem;
  line-height: 1.4;
  color: var(--color-text);
  resize: vertical;
  background: var(--color-surface);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-sm);
}

.ui-textarea:focus {
  outline: none;
  border-color: var(--color-text);
}

.ui-textarea:disabled {
  color: var(--color-text-muted);
  cursor: not-allowed;
  background: var(--color-bg);
}
</style>
