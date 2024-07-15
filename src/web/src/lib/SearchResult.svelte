<script>
  export let result;

  const url = new URL(result.url);
  const serverUrl = `${import.meta.env.VITE_API_URI}/original/${result.url}`;
  const faviconUrl = `https://www.google.com/s2/favicons?domain=${url.hostname.split(".").slice(-2).join(".")}&sz=32`;
  const archiveUrl = "https://web.archive.org/web/*/" + result.url;
</script>

<section>
      <div class="result">
        <img class="favicon" alt="Favicon" src={faviconUrl} />
        <span class="mime">{result.mime}</span>
        <span class="timestamp">{new Date(result.crawledAt).toLocaleString()}</span>
        <div class="links">
          <a class="server" href={serverUrl} target="_blank">Server</a>
          <a class="external" href={result.url} target="_blank">External</a>
          <a class="archive" href={archiveUrl} target="_blank">Archive</a>
          <a class="default" href={result.url} target="_blank">
            {result.url}
            {#if result.mime.startsWith("image/")}
              <br /> 
              <img class="image" alt="Result" src={serverUrl} />
            {/if}
          </a>
        </div>
      </div>
</section>

<style>
  .image {
    max-width: 20%;
  }
</style>
