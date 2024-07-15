<script>
	import SearchResult from "./lib/SearchResult.svelte";
	import Pages from "./Pages.svelte";
	import "ress";

	const params = new URLSearchParams(window.location.search);

	let fetchResults = async (query, page, algorithm) => {
		if (query == "") return;
		const resp = await fetch(import.meta.env.VITE_API_URI + `/api/search-results?page=${page}&query=${query}&algorithm=${algorithm}`);

		const url = new URL(window.location);

		url.searchParams.set("query", query);
		url.searchParams.set("page", page)

		history.replaceState({query}, "", url)

		return await resp.json()
	};

	let page = Number(params.get("page") ?? 1);
	let query = params.get("query") ?? "";
	let algorithm = params.get("algorithm") ?? "fuzzy";

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
