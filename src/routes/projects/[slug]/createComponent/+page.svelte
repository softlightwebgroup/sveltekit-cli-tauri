<script lang='ts'>
	import FormContainer from '$lib/components/atom/FormContainer.svelte';
	import FormColumn from '$lib/components/atom/FormColumn.svelte';
	import { Button } from '@softlightweb/svelte';
	import SpacingRows from '$lib/components/atom/SpacingRows.svelte';
	import FormRow from '$lib/components/atom/FormRow.svelte';
	import { invoke } from '@tauri-apps/api';
	import { page } from '$app/stores';
	import { projectsStore } from '$lib/stores';
	import type { ProjectModel } from '$lib/models/ProjectModel.svelte';

	let loading = $state<boolean>(false);

	let project: ProjectModel = $derived(projectsStore.value.find(i => i.id === $page.params.slug));

	const onSubmit = (e) => {
		const data = new FormData(e.target);
		e.preventDefault();
		loading = true;
		invoke('create_component', {
			directory: project.directory,
			component: data.get('name')
		});
		loading = false;
		e.target.reset();
	};
</script>

<FormContainer>
	<h1>Create new component</h1>
	<form action='' method='post' onsubmit={onSubmit}>
		<SpacingRows>
			<FormColumn>
				<input autofocus name='name' placeholder='Name' type='text' />
			</FormColumn>
			<FormRow>
				<Button {loading} type='submit'>
					{#if loading}
						<i class='fas fa-spinner fa-spin' />
					{:else}
						<i class='fas fa-plus' style='margin-right: 10px;' />
						Create new component
					{/if}
				</Button>
			</FormRow>
		</SpacingRows>
	</form>
</FormContainer>