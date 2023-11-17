<script lang='ts'>
	import { fnClassnames } from '@softlightweb/svelte';

	let { class: classname = '', row = false, ...rest } = $props();
</script>

<div {...rest} class={fnClassnames("title", classname)} class:hover={rest.onClick} class:row>
	<slot></slot>
</div>

<style lang='scss'>
  .title {
    display: flex;
    flex-direction: column;
    gap: .5rem;
    flex: 1;

    &.hover {
      cursor: pointer;
    }

    &.row {
      align-items: center;
      flex-direction: row;
    }
  }
</style>