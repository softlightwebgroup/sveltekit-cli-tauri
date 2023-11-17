<script lang="ts">
	import { projectsStore } from '$lib/stores/index.js';
	import ProjectItem from '$lib/components/molecules/ProjectItem.svelte';
	import NoProjectsFound from '$lib/components/molecules/NoProjectsFound.svelte';
</script>

{#each projectsStore.value as project}
	<ProjectItem {project} />
{:else}
	<NoProjectsFound />
{/each}