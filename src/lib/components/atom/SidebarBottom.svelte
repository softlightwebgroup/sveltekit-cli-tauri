<div class='sidebar-bottom'>
	<slot></slot>
</div>

<style lang='scss'>
  .sidebar-bottom {
    width: 100%;
    height: var(--height-sidebar-bottom, 300px);
    background-color: var(--color-grey-2);
    border-top: 1px solid var(--color-grey-3);
    display: flex;
    flex-direction: column;
    margin-top: auto;

    &:empty {
      display: none;
    }
  }
</style>