<script lang='ts'>
	import '$lib/scss/app.scss';
	import { invoke } from '@tauri-apps/api';
	import { onMount, setContext } from 'svelte';
	import { projectsStore } from '$lib/stores';
	import Sidebar from '$lib/components/containers/Sidebar.svelte';
	import { ProjectModel } from '$lib/models/ProjectModel.svelte';

	let aside = $state(null);
	let asideRight = $state(null);

	setContext('Layout', {
		set aside(value) {
			aside = value;
		},
		set asideRight(value) {
			asideRight = value;
		}
	});

	onMount(async () => {
		const projects = await invoke('get_projects');
		projectsStore.value = projects.map((project) => new ProjectModel(project));
	});
</script>

<section class='layout'>
	<Sidebar {aside} />
	<main>
		<slot></slot>
	</main>
	{#if asideRight}
		<Sidebar aside={asideRight} />
	{/if}
</section>

<style lang='scss'>
  .layout {
    display: grid;
    grid-template-columns: 300px 1fr auto;
    height: 100vh;
    overflow: hidden;

    @media (width <= 600px) {
      grid-template-columns: 0 1fr auto;
    }
  }
</style>