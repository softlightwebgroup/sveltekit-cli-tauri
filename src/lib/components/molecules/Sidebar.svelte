<aside class='sidebar'>
	<slot></slot>
</aside>

<style lang='scss'>
  .sidebar {
    width: 300px;
    height: 100%;
    overflow: hidden;
    overflow-y: auto;
    border-right: 1px solid var(--color-grey-2);
    background-color: var(--color-grey-1);
    padding: 1rem;
  }
</style>