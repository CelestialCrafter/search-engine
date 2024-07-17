<script>
	import SearchResult from "./lib/SearchResult.svelte";
	import Pages from "./Pages.svelte";
	import "ress";

	const params = new URLSearchParams(window.location.search);

	let fetchResults = async (query, page, algorithm) => {
		if (query == "") return;
		const resp = await fetch(import.meta.env.VITE_API_URI + `/api/search-results?page=${page}&query=${query}&algorithm=${algorithm}`);

		return await resp.json()
	};

	$: page = 1;
	$: query = "";
	$: algorithm = "fuzzy";

	$: results = fetchResults(query, page, algorithm);
</script>

<main>
	<input type="text" placeholder="Search..." bind:value={query}  />
	<select bind:value={algorithm}>
		<option value="fuzzy">Fuzzy</option>
		<option value="bm25">BM25</option>
	</select>

	<br /><br />
	{#await results then data}
		<Pages total={data.total} bind:page={page} />
		{#each data.results as result}
			<SearchResult result={result} />
		{/each}
		<Pages total={data.total} bind:page={page} />
	{/await}
</main>
