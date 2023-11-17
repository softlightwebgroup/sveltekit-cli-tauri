<script lang='ts'>
	import FormColumn from '$lib/components/atom/FormColumn.svelte';
	import FormContainer from '$lib/components/atom/FormContainer.svelte';
	import FormRow from '$lib/components/atom/FormRow.svelte';
	import SpacingRows from '$lib/components/atom/SpacingRows.svelte';
	import Button from '$lib/components/atom/Button.svelte';
	import { invoke } from '@tauri-apps/api';
	import { projectsStore } from '$lib/stores/index.js';
	import { page } from '$app/stores';
	import type { ProjectModel } from '$lib/models/ProjectModel.svelte.js';
	import Loading from '$lib/components/molecules/Loading.svelte';

	let pages = [
		{
			value: '+page.svelte',
			name: '+page.svelte'
		},
		{
			value: '+page.server.ts',
			name: '+page.server.ts'
		},
		{
			value: '+layout.ts',
			name: '+layout.ts'
		},
		{
			value: '+layout.server.ts',
			name: '+layout.server.ts'
		},
		{
			value: '+layout.svelte',
			name: '+layout.svelte'
		}
	];

	let loading = $state(false);
	let checked = [pages[0].value, pages[1].value];

	let project: ProjectModel = $derived(projectsStore.value.find(i => i.id === $page.params.slug));

	async function onSubmit(e) {
		const data = new FormData(e.target);
		loading = true;
		const routes = await invoke('create_route', {
			directory: project.directory,
			route: data.get('path'),
			pages: checked
		});
		console.log({ routes });
		loading = false;
	}
</script>

<FormContainer>
	<h1>Create new route</h1>

	<form method='post' on:submit|preventDefault={onSubmit}>
		<SpacingRows>
			<FormColumn>
				Path
				<input autofocus name='path' placeholder='Name' required type='text' />
			</FormColumn>
			{#each pages as page}
				<FormRow>
					<input bind:group={checked} type='checkbox' value={page.value} />
					<span>{page.name}</span>
				</FormRow>
			{/each}

			<FormRow>
				{#if loading}
					<Loading />
				{:else}
					<Button --color-bg='var(--color-primary)'>
						<i class='fas fa-plus'></i>
						Create route
					</Button>
				{/if}
			</FormRow>
		</SpacingRows>
	</form>
</FormContainer>