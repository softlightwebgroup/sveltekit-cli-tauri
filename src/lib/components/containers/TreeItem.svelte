<script lang='ts'>
	import type { TDirectory } from '$lib/types/TDirectory';
	import { invoke } from '@tauri-apps/api';
	import Tree from '$lib/components/containers/Tree.svelte';

	let { file, path } = $props<{
		file: TDirectory;
		path: string
	}>();

	let directory = $state([]);

	async function onClick() {
		if (!file.is_dir) return;

		if (directory.length > 0) {
			directory = [];
			return;
		}

		directory = await invoke('get_files', {
			projectId: '',
			path: `${ path }/${ file.name }`
		});
	}
</script>

<div class='tree-group'>
	<div class='tree-item' on:click={onClick}>
		<i class="fa {file.is_dir ? 'fa-folder' : 'fa-file'}"></i>
		{file.name}
	</div>

	{#if file.is_dir}
		<Tree files={directory} path={`${ path }/${ file.name }`} />
	{/if}
</div>

<style lang='scss'>
  .tree-item {
    display: flex;
    align-items: center;
    gap: .2rem;
    padding: .2rem .5rem;
    border-radius: .4rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    background: var(--color-grey-2);
    transition: opacity .2s ease-in-out;
    user-select: auto;

    &:hover {
      opacity: .8;
      cursor: pointer;
    }
  }

  .tree-group {
    :global(>.tree) {
      padding-left: 1rem;
    }
  }
</style>