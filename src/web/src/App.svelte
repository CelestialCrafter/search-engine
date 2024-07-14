<script>
	import SearchResult from "./lib/SearchResult.svelte";
	import Pages from "./Pages.svelte";

	const params = new URLSearchParams(window.location.search);
	const page = Number(params.get("page") ?? 1);

	const limit = 100;
	const offset = (page - 1) * limit;

	const fetchResults = async () => await (await fetch(import.meta.env.VITE_API_URI + `/api/search-results?offset=${offset}&limit=${limit}`)).json();
</script>

<main>
{#await fetchResults() then data}
	<Pages total={data.total} limit={limit} page={page} />
	{#each data.results as result}
		<SearchResult result={result} />
	{/each}
	<Pages total={data.total} limit={limit} page={page} />
{/await}
</main>
