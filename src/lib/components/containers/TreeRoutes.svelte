<script lang='ts'>
	import { invoke } from '@tauri-apps/api';

	let { path } = $props<{ path: string }>();

	let routes = $state([]);

	invoke('get_routes', { path }).then(rs => {
		console.log({ rs });
		routes = rs;
	});
</script>

<div class='tree-routes'>
	{#each routes as route}
		<div class='route'>
			{route}
		</div>
	{/each}
</div>

<style lang='scss'>

</style>