<script>
	import SearchResult from "./lib/SearchResult.svelte";
	import Pages from "./Pages.svelte";
	import "ress";

	const limit = 100;
	const params = new URLSearchParams(window.location.search);
	let page = Number(params.get("page") ?? 1)

	let fetchResults = async () => {
		const offset = (page - 1) * limit;

		const resp = await fetch(import.meta.env.VITE_API_URI + `/api/search-results?offset=${offset}&limit=${limit}`);
		return await resp.json()
	};

	let results = fetchResults();

	const setPage = i => {
		page = i;
		const url = new URL(window.location);
		url.searchParams.set("page", i)
		history.pushState({}, "", url);
		results = fetchResults();
	};

</script>

<main>
{#await results then data}
	<Pages total={data.total} limit={limit} page={page} setPage={setPage} />
	{#each data.results as result}
		<SearchResult result={result} />
	{/each}
	<Pages total={data.total} limit={limit} page={page} setPage={setPage} />
{/await}
</main>
