<script>
	import SearchResult from "./lib/SearchResult.svelte";
	import Pages from "./Pages.svelte";
	import protobuf from "protobufjs";
	import "ress";

	const params = new URLSearchParams(window.location.search);

	let fetchResults = async (query, page, algorithm) => {
		const resp = await fetch(import.meta.env.VITE_API_URI + `/api/search-results?page=${page}&query=${query}&algorithm=${algorithm}`);

		const data = await resp.json();
		const proto = await (await fetch('/public/crawled.proto')).text();
		const { root } = protobuf.parse(proto);
		const Document = root.lookupType("crawler.Document");
		
		data.results = data.results.map(pb => {
			const buf = new Uint8Array(pb.length)
			console.log(protobuf.util.utf8.write(pb, buf, 0))
			console.log(pb)
			const message = Document.decode(pb)
			return Document.toObject(message);
		});

		return data
	};

	let page = 1;
	let query = "";
	let algorithm = "fuzzy";
	let results = (async () => ({
		total: 0,
		results: [],
	}))();

	$: if (query != "") (results = fetchResults(page, query, algorithm));
	$: console.log(results)
</script>

<main>
	<input type="text" placeholder="Search..." bind:value={query}  />
	<select bind:value={algorithm}>
		<option value="fuzzy">Fuzzy</option>
		<option value="bm25">BM25</option>
	</select>

	<br /><br />
	{#await results then data}
		<!-- <Pages total={data.total} bind:page={page} /> -->
		{#each data.results as result}
			<SearchResult result={result} />
		{/each}
		<!-- <Pages total={data.total} bind:page={page} /> -->
	{:catch error}
		<span>{error}</span>
	{/await}
</main>
