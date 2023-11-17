<script lang='ts'>
	import { appWindow } from '@tauri-apps/api/window';
	import { getContext, onMount, setContext, tick } from 'svelte';
	import { page } from '$app/stores';
	import SidebarProjectDetail from '$lib/components/containers/SidebarProjectDetail.svelte';
	import { projectsStore } from '$lib/stores';
	import SidebarProjectDirectory from '$lib/components/containers/SidebarProjectDirectory.svelte';
	import Content from '$lib/components/atom/Content.svelte';
	import SidebarBottom from '$lib/components/atom/SidebarBottom.svelte';
	import { listen } from '@tauri-apps/api/event';
	import AnsiToHtml from 'ansi-to-html';

	const data = getContext('Layout');
	const convert = new AnsiToHtml();
	let refTerminal = $state(null);
	let terminal = $state([]);
	appWindow.maximize();
	appWindow.setTitle('Detail project');

	data.aside = aside;
	data.asideRight = asideRight;

	let project = $derived(projectsStore.value.find(i => i.id === $page.params.slug));

	onMount(() => {
		listen('command:output', (event) => {
			const lineHtml = convert.toHtml(event.payload);
			terminal = terminal.concat(lineHtml);
		});
	});

	$effect(() => {
		if (!refTerminal) return;

		terminal;

		tick().then(() => {
			refTerminal.scrollTop = refTerminal.scrollHeight;
		});
	});

	setContext('LayoutProject', {
		clearTerminal: () => {
			terminal = [];
		},
		addTerminalLine: (line) => {
			terminal = terminal.concat(line);
		}
	});
</script>

{#snippet aside()}
<SidebarProjectDetail
	on:finishedCommand={() => {
		terminal = terminal.concat('- Command finished -');
	}}
	{project}
/>
{/snippet}

{#snippet asideRight()}
<SidebarProjectDirectory
	{project}
/>
{/snippet}

<div class='project-detail'>
	<Content>
		<slot></slot>
	</Content>
	<SidebarBottom>
		{#if Boolean(terminal.length)}
			<div bind:this={refTerminal} class='terminal'>
				{#each terminal as line}
					<div class='terminal-line'>
						{line}
					</div>
				{/each}
			</div>
		{/if}
	</SidebarBottom>
</div>

<style lang='scss'>
  .project-detail {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .terminal {
    padding: 2rem;
    scroll-behavior: smooth;
    overflow: auto;
  }

  .terminal-line {
    margin-bottom: .5rem;
  }
</style>