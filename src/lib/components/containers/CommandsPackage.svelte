<script lang='ts'>
	import type { ProjectModel } from '$lib/models/ProjectModel.svelte';
	import { invoke } from '@tauri-apps/api';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
	let { project } = $props<{ project: ProjectModel }>();
	let commands = $state([]);

	$effect(async () => {
		commands = await invoke('get_commands', {
			path: project.directory
		});
	});

	const onClickCommand = (command: string) => async () => {
		dispatch('startedCommand');
		let commandRs = await invoke('run_command', {
			path: project.directory,
			command
		});

		dispatch('finishedCommand');
	};
</script>

<h2>Commands</h2>
<ul class='commands-package'>
	{#each commands as command}
		<li on:click={onClickCommand(command)}>
			<i class='fa-solid fa-terminal'></i>
			<h3>{command}</h3>
		</li>
	{/each}
</ul>

<style lang='scss'>
  h2 {
    margin-top: 1rem;
  }

  .commands-package {
    display: flex;
    flex-direction: column;
    gap: .1rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
    list-style: none;

    li {
      display: flex;
      align-items: center;
      gap: .5rem;
      cursor: pointer;
    }
  }
</style>