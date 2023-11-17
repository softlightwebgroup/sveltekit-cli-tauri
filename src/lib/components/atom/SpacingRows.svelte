<div class='spacing rows'>
	<slot></slot>
</div>

<style lang='scss'>
  .spacing {
    display: flex;
    flex-direction: column;
    gap: var(--spacing, 1rem);
  }
</style>