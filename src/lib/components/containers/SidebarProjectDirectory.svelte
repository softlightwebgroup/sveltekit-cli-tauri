<script lang='ts'>
	import type { ProjectModel } from '$lib/models/ProjectModel.svelte';
	import { invoke } from '@tauri-apps/api';
	import Tree from '$lib/components/containers/Tree.svelte';
	import { useSelector } from '$lib/hooks/selector.svelte';
	import TreeRoutes from '$lib/components/containers/TreeRoutes.svelte';

	let { project } = $props<{ project: ProjectModel }>();

	let directory = $state([]);
	const tabs = ['Directory', 'Routes'];
	const selector = useSelector(tabs[0]);

	$effect(async () => {
		if (!project) return;
		console.log('GET FILES', project.id, project.directory);
		let files = await invoke('get_files', { projectId: project.id, path: project.directory });
		directory = files;
	});
</script>

{#if project}
	<div class='sidebar-project-directory'>
		<div class='tabs'>
			{#each tabs as tab}
				<div class:active={selector.isEqual(tab)} class='tab' on:click={() => selector.set(tab)}>
					{tab}
				</div>
			{/each}
		</div>
		{#if selector.isEqual('Directory')}
			<Tree files={directory} path={project.directory} />
		{:else if selector.isEqual('Routes')}
			<TreeRoutes path='{project.directory}' />
		{/if}
	</div>
{/if}

<style lang='scss'>
  .tabs {
    display: flex;
    align-items: center;
    gap: .5rem;
    margin-bottom: 1rem;
  }

  .tab {
    padding: .5rem;
    cursor: pointer;
    background: var(--color-grey-2);
    border-radius: .5rem;
    transition: background-color .3s ease-in-out;

    &:hover,
    &.active {
      background-color: var(--color-grey-3);
    }
  }
</style>