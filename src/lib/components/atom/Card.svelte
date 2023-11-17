<script lang='ts'>
	import { page } from '$app/stores';

	let { href = undefined } = $props();
</script>

<svelte:element class='card' class:active={$page.url.pathname === href} {href} this={href ? 'a' : 'article'}>
	<slot></slot>
</svelte:element>

<style lang='scss'>
  .card {
    padding: 1rem;
    border-radius: 0.5rem;
    box-shadow: 0 0 0.5rem rgba(0, 0, 0, 0.2);
    background-color: var(--card-bg, var(--color-grey-2));
    transition: box-shadow 0.2s ease-in-out, opacity 0.2s ease-in-out;
    color: var(--card-color, var(--color-grey));

    &:hover, &.active {
      box-shadow: 0 0 1rem rgba(0, 0, 0, 0.2);
      cursor: pointer;
      opacity: .8;
			background-color: var(--card-bg-hover, var(--color-grey-3));
    }
  }
</style>