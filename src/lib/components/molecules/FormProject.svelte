<script lang='ts'>
	import FormColumn from '$lib/components/atom/FormColumn.svelte';
	import Button from '$lib/components/atom/Button.svelte';
	import ButtonPathDirectory from '$lib/components/containers/ButtonPathDirectory.svelte';
	import { invoke } from '@tauri-apps/api';
	import Loading from '$lib/components/molecules/Loading.svelte';
	import { projectsStore } from '$lib/stores';
	import { ProjectModel } from '$lib/models/ProjectModel.svelte';
	import type { EProject } from '$lib/types/Project';

	let loading = $state(false);
	const onSubmit = async (e) => {
		const data = new FormData(e.target);
		const name = data.get('name');
		const directory = data.get('directory');
		loading = true;

		const id = crypto.randomUUID();
		const rq: EProject = { id, name, directory };
		await invoke('save_project', rq);

		projectsStore.push(new ProjectModel(rq));
		loading = false;
	};
</script>

<form class='form-project' method='post' on:submit|preventDefault={onSubmit}>
	<FormColumn>
		Project name
		<input autofocus name='name' placeholder='Project Name' required type='text' />
	</FormColumn>
	<FormColumn>
		Project directory
		<ButtonPathDirectory />
	</FormColumn>
	{#if loading}
		<Loading />
	{:else}
		<Button>Create project</Button>
	{/if}
</form>

<style lang='scss'>
  .form-project {
    --color-bg: var(--color-grey-2);
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
    margin-top: 1rem;
  }
</style>