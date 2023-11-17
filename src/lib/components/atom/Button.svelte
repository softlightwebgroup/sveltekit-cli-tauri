<script lang='ts'>
	let { href = undefined, ...rest } = $props();
</script>

<svelte:element {...rest} class='button' {href} this={href ? 'a' : 'button'}>
	<slot></slot>
</svelte:element>

<style lang='scss'>
  .button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 1rem;
    font-size: 1rem;
    font-weight: 500;
    color: var(--color-text);
    background-color: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 0.25rem;
    cursor: pointer;
    transition: opacity 0.2s ease-in-out;

    &:hover {
      opacity: 0.8;
    }
  }
</style>