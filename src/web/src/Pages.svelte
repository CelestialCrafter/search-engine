<script>
	export let total;
	export let page;
</script>

<section>
	<ul class="pages">
	{#each Array(total) as _, i}
		{(i = i + 1) && ""}
		<button on:click={() => (page = i)} class={page == i ? "selected" : ""}>{i}</button>
	{/each}
	</ul>
</section>

<style>
	.pages {
		display: flex;
		gap: 0.4rem;
	}
	.selected {
		background-color: plum;
	}
</style>
