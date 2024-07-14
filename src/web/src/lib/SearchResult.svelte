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
          <a class="server" href={serverUrl}>Server</a>
          <a class="external" href={result.url}>External</a>
          <a class="archive" href={archiveUrl}>Archive</a>
          <a class="default" href={result.url}>
            {#if result.mime.startsWith("image/")}
              <br /> 
              <img class="image" alt="Result" src={serverUrl} />
            {:else}
              {result.url}
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
