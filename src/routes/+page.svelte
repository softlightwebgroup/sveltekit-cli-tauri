<script>
	import ProjectsList from '$lib/components/containers/ProjectsList.svelte';
	import Title from '$lib/components/atom/Title.svelte';
	import SpacingRows from '$lib/components/atom/SpacingRows.svelte';
	import { appWindow } from '@tauri-apps/api/window';
	import { getContext } from 'svelte';
	import Container from '$lib/components/atom/Container.svelte';

	const data = getContext('Layout');

	appWindow.setTitle('SvelteKit: Projects');
	appWindow.unmaximize();

	data.aside = null;
	data.asideRight = null;
</script>

<Container>
	<SpacingRows>
		<Title>
			<h1>Welcome to SvelteKit CLI</h1>
			<p>Visit <a href='https://kit.svelte.dev'>kit.svelte.dev</a> to read the documentation</p>
		</Title>
		<ProjectsList />
	</SpacingRows>
</Container>