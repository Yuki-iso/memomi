<script>
  let state = false;
  let progress = 30;
</script>
<svelte:head>
	<title>About</title>
	<meta name="description" content="About this app" />
</svelte:head>
<div class="flex justify-center w-screen h-full">
	<div class="flex flex-col justify-center w-1/2">
    <div class="h-6 rounded-xl border-gray-400 border-2 my-4">
      <div style="width: {progress}%" class="bg-green-500 h-full rounded-xl"></div>
    </div>
		<div class="bg-gray-100 text-center flex flex-col justify-center rounded-xl h-[600px]">
      {#if state}
			<p class="text-5xl font-bold p-5">〜か月</p>
			<p class="text-2xl p-5">2か 月[げつ] 前[まえ]に 日本[にほん]に 来[き]ました。</p>
      {:else}
      <p class="text-5xl font-bold p-5">test</p>
      <p class="text-2xl p-5">test</p>
      {/if}
		</div>
		<div class="flex gap-4 justify-center">
      {#if state}
			<button on:click={() => state = !state} class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100">Show</button>
      {:else}
      <button on:click={() => state = !state} class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-green-500">Easy</button>
      <button on:click={() => progress += 10} class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-yellow-500">Medium</button>
      <button on:click={() => state = !state} class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-red-500">Hard</button>
      {/if}
		</div>
	</div>
</div>
