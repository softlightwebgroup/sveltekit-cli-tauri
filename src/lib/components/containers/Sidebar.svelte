<script lang='ts'>
	import Sidebar from '$lib/components/molecules/Sidebar.svelte';
	import Projects from '$lib/components/containers/Projects.svelte';
	import Button from '$lib/components/atom/Button.svelte';
	import Title from '$lib/components/atom/Title.svelte';

	let { aside = null } = $props();
</script>

{#snippet content()}
<Title class='sidebar__title' row>
	<h2>List of projects</h2>
	<Button href='/projects/new'>New Project</Button>
</Title>
<div class='projects'>
	<Projects />
</div>
{/snippet}

<Sidebar>
	{#if aside}
		{@render aside()}
	{:else}
		{@render content()}
	{/if}
</Sidebar>

<style lang='scss'>
  .projects {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  :global(.sidebar__title) {
    --color-bg: red;
    margin-bottom: 2rem;
  }
</style>