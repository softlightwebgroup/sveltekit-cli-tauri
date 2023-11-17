<script lang='ts'>
	import type { TDirectory } from '$lib/types/TDirectory';
	import TreeItem from '$lib/components/containers/TreeItem.svelte';

	let { files, path } = $props<{
		files: TDirectory[],
		path: string
	}>();
</script>

<div class='tree'>
	{#each files as file}
		<TreeItem {file} {path}/>
	{/each}
</div>

<style lang='scss'>
  .tree {
    display: flex;
    flex-direction: column;
    gap: .1rem;
  }
</style>