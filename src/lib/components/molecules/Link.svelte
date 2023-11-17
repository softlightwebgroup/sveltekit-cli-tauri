<script lang='ts'>
	let { href, button, ...rest } = $props<{ href: string }>();
</script>

<a {...rest} class='link' class:button {href}>
	<slot></slot>
</a>

<style lang='scss'>
  .link {
    color: inherit;
    text-decoration: none;
    cursor: pointer;

    &.button {
      border: 1px solid var(--color-grey-3);
      padding: 0.5rem 1rem;
      border-radius: 0.25rem;
      transition: all 0.2s ease-in-out;
      width: 100%;
			display: flex;
			justify-content: center;
			align-items: center;
      background: var(--color-grey-2);

			&:hover {
				background: var(--color-grey-3);
			}
    }
  }
</style>