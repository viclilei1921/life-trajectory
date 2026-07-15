<script lang="ts">
interface Props {
  message: string;
  ondismiss?: () => void;
}

let { message, ondismiss }: Props = $props();
</script>

<div class="error-banner" role="alert">
  <span>{message}</span>
  {#if ondismiss}
    <button type="button" class="dismiss" onclick={ondismiss} aria-label="关闭">×</button>
  {/if}
</div>

<style>
.error-banner {
  display: flex;
  gap: var(--space-sm);
  align-items: flex-start;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  font-size: 0.875rem;
  color: var(--color-error-text);
  background: var(--color-error-bg);
  border: 1px solid var(--color-error-text);
  border-radius: var(--radius-lg);
}

.dismiss {
  padding: 0;
  font-size: 1.25rem;
  line-height: 1;
  color: inherit;
  cursor: pointer;
  background: none;
  border: none;
}
</style>
