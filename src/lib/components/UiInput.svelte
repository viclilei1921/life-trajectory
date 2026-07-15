<script lang="ts">
interface Props {
  id?: string;
  type?: string;
  label?: string;
  placeholder?: string;
  value?: string;
  disabled?: boolean;
  oninput?: (value: string) => void;
}

let { id, type = 'text', label, placeholder, value = $bindable(''), disabled = false, oninput }: Props = $props();

const fallbackId = `ui-input-${crypto.randomUUID()}`;
const inputId = $derived(id ?? fallbackId);

function handleInput(e: Event) {
  const target = e.currentTarget as HTMLInputElement;
  value = target.value;
  oninput?.(target.value);
}
</script>

<div class="ui-field">
  {#if label}
    <!-- biome-ignore lint/a11y/noLabelWithoutControl: label uses for/id to associate with input -->
    <label class="ui-label" for={inputId}>{label}</label>
  {/if}
  <input id={inputId} {type} {placeholder} {disabled} {value} class="ui-input" oninput={handleInput}>
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

.ui-input {
  box-sizing: border-box;
  width: 100%;
  height: var(--control-height);
  padding: 12px 16px;
  font: inherit;
  font-size: 1rem;
  color: var(--color-text);
  background: var(--color-surface);
  border: 1px solid var(--color-border-strong);
  border-radius: var(--radius-sm);
}

.ui-input:focus {
  outline: none;
  border-color: var(--color-text);
}

.ui-input:disabled {
  color: var(--color-text-muted);
  cursor: not-allowed;
  background: var(--color-bg);
}
</style>
