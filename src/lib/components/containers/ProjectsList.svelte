<script lang='ts'>
	import Projects from '$lib/components/containers/Projects.svelte';
</script>

<section class='projects-list'>
	<Projects />
</section>

<style lang='scss'>
  .projects-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;

    @media (width < 600px) {
      grid-template-columns: 1fr;
    }
  }
</style>