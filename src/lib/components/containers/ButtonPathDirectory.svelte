<script lang='ts'>
	import Button from '$lib/components/atom/Button.svelte';
	import { dialog } from '@tauri-apps/api';

	let directoryPath = $state('');
	const onClick = async (e) => {
		e.preventDefault();
		const selected = await dialog.open({ directory: true });

		directoryPath = selected ?? '';
	};

</script>

<Button {onClick}>
	{#if directoryPath}
		{directoryPath}
	{:else}
		Open directory path
	{/if}
</Button>

<input name='directory' type='hidden' value={directoryPath}>