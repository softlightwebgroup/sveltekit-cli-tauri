<script lang='ts'>
	import Title from '$lib/components/atom/Title.svelte';
	import type { ProjectModel } from '$lib/models/ProjectModel.svelte';
	import Loading from '$lib/components/molecules/Loading.svelte';
	import { fnCapitalizeFirstLetter } from '@softlightweb/svelte';
	import Card from '$lib/components/atom/Card.svelte';
	import SpacingRows from '$lib/components/atom/SpacingRows.svelte';
	import CommandsPackage from '$lib/components/containers/CommandsPackage.svelte';

	let { project } = $props<{ project: ProjectModel }>();

	const commands = [
		{ title: 'Create route', command: 'createRoute' },
		{ title: 'Create component', command: 'createComponent' },
		{ title: 'Create store', command: 'createStore' },
		{ title: 'Create service', command: 'createService' },
		{ title: 'Create action', command: 'createAction' },
		{ title: 'Create model', command: 'createModel' },
		{ title: 'Create translation', command: 'createTranslation' },
	];
</script>

<div class='sidebar-project-detail'>
	{#if project}
		<Title row onClick={() => history.back()}>
			<i class='fa-solid fa-chevron-left'></i>
			<h2>{fnCapitalizeFirstLetter(project.name)}</h2>
		</Title>
		<SpacingRows --spacing='.5rem'>
			{#each commands as command}
				<Card href='/projects/{project.id}/{command.command}'>
					<h3>{command.title}</h3>
				</Card>
			{/each}
		</SpacingRows>
		<CommandsPackage {project} on:finishedCommand />
	{:else}
		<Title>
			<Loading />
			<h2>Loading...</h2>
		</Title>
	{/if}
</div>

<style lang='scss'>
  .sidebar-project-detail {
    :global(.title) {
      margin-bottom: 1rem;
    }
  }
</style>