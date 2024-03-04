<script>
	export let data;
	let cards = data.props;
	let show_card = false;
	let progress = 0;
	let id = 0;
	let max_progress = cards.length;

	$: new_cards = cards.filter((card) => card.status === 'New').length;
	$: failed_cards = cards.filter((card) => card.status === 'Failed').length;
	$: review_cards = cards.filter((card) => card.status === 'Review').length;
	let card = cards[id];

	function next_card(event) {
		switch (event.target.innerText) {
			case 'Easy':
				progress += 100 / max_progress;
				cards = cards.filter((_, index) => index !== id); 
				break;
			case 'Medium':
				progress += 100 / max_progress;
				cards = cards.filter((_, index) => index !== id); 
				break;
			case 'Hard':
				cards[id].status = 'Failed';

				break;
		}
		card = cards[id];
		show_card = false;
	}
</script>

<svelte:head>
	<title>About</title>
	<meta name="description" content="About this app" />
</svelte:head>

<div class="flex justify-center w-screen h-full pt-20">
	{#if card != null}
		<div class="flex flex-col justify-center w-1/2">
			<div class="h-6 rounded-xl border-gray-400 border-2 my-4">
				<div style="width: {progress}%" class="bg-green-500 h-full rounded-xl"></div>
			</div>
			<div class="bg-gray-100 text-center flex flex-col justify-center rounded-xl h-[600px]">
				{#if show_card}
					<p class="text-5xl font-bold p-5">{card.english}</p>
					<p class="text-2xl p-5">{card.en_sentence}</p>
				{:else}
					<p class="text-5xl font-bold p-5">{card.kanji}</p>
					<p class="text-2xl p-5">{card.jp_sentence}</p>
				{/if}
			</div>
			<div class="flex gap-4 justify-center">
				{#if !show_card}
					<button
						on:click={() => (show_card = !show_card)}
						class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100">Show</button
					>
				{:else}
					<button
						on:click={(event) => next_card(event)}
						class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-green-500"
						>Easy</button
					>
					<button
						on:click={(event) => next_card(event)}
						class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-yellow-500"
						>Medium</button
					>
					<button
						on:click={(event) => next_card(event)}
						class="p-5 w-1/4 my-4 rounded-xl text-xl font-bold bg-gray-100 text-red-500"
						>Hard</button
					>
				{/if}
			</div>
			{#key new_cards}
				<div class="flex justify-center text-center gap-4">
					<p class="text-blue-300">{new_cards}</p>
					<p class="text-red-300">{failed_cards}</p>
					<p class="text-green-300">{review_cards}</p>
				</div>
			{/key}
		</div>
	{:else}
		<p class="text-5xl font-bold p-5">No more cards</p>
	{/if}
</div>
