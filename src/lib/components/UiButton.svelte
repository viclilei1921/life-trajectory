<script lang="ts">
import type { Snippet } from 'svelte';

type Variant = 'primary' | 'secondary' | 'ghost';

interface Props {
  type?: 'button' | 'submit';
  variant?: Variant;
  disabled?: boolean;
  onclick?: () => void;
  children: Snippet;
}

let { type = 'button', variant = 'primary', disabled = false, onclick, children }: Props = $props();
</script>

<button
  {type}
  class="ui-button"
  class:primary={variant === 'primary'}
  class:secondary={variant === 'secondary'}
  class:ghost={variant === 'ghost'}
  {disabled}
  {onclick}
>
  {@render children()}
</button>

<style>
.ui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: var(--control-height);
  padding: 12px 24px;
  font: inherit;
  font-size: 0.875rem;
  font-weight: 600;
  letter-spacing: 0.04em;
  cursor: pointer;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  transition:
    background 0.15s,
    border-color 0.15s,
    color 0.15s;
}

.primary {
  color: var(--color-on-primary);
  background: var(--color-primary);
  border-color: var(--color-primary);
}

.primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
  border-color: var(--color-primary-hover);
}

.primary:active:not(:disabled) {
  background: var(--color-primary-pressed);
  border-color: var(--color-primary-pressed);
}

.secondary {
  color: var(--color-primary);
  background: var(--color-surface);
  border-color: var(--color-primary);
}

.secondary:hover:not(:disabled) {
  background: var(--color-primary-soft);
}

.ghost {
  color: var(--color-text);
  background: var(--color-surface);
  border-color: var(--color-secondary);
}

.ghost:hover:not(:disabled) {
  background: var(--color-bg);
}

.ui-button:disabled {
  color: var(--color-on-primary);
  cursor: not-allowed;
  background: var(--color-disabled);
  border-color: var(--color-disabled);
}

.secondary:disabled,
.ghost:disabled {
  color: var(--color-text-muted);
  background: var(--color-surface);
  border-color: var(--color-disabled);
}
</style>
