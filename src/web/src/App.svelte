<script>
	import SearchResult from "./lib/SearchResult.svelte";

	const fetchResults = async () => await (await fetch(import.meta.env.VITE_API_URI + "/api/search-results")).json();
</script>

<main>
{#await fetchResults() then searchResults}
	{#each searchResults as result}
		<SearchResult result={result} />
	{/each}
{/await}
</main>
