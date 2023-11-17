<script lang='ts'>
	import Link from '$lib/components/molecules/Link.svelte';

	let { project } = $props<{ project: ProjectModel }>();
</script>

<article class='project-item'>
	<Link href='/projects/{project.id}'>
		<h2>{project.name}</h2>
		<p>{project.directory}</p>
	</Link>
</article>

<style lang='scss'>
  .project-item {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    background-color: var(--color-grey-2);
    border-radius: 10px;
    padding: .5rem;
    box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.2) inset, 0 0 5px 0 rgba(0, 0, 0, 0.2);
    transition: box-shadow 0.2s ease-in-out;

    :global(a) {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }

    &:hover {
      box-shadow: 0 0 5px 0 rgba(0, 0, 0, 0.4) inset, 0 0 5px 0 rgba(0, 0, 0, 0.4);
      cursor: pointer;
    }
  }
</style>