<div class='form-container'>
	<slot />
</div>

<style lang='scss'>
  .form-container {
    max-width: 400px;
    margin: auto;
    display: flex;
    flex-direction: column;
    gap: .5rem;
    background: var(--color-bg, var(--color-grey-1));
    padding: 1rem;
    box-shadow: 0 0 1rem rgba(0, 0, 0, .1);
    border-radius: .5rem;
  }
</style>