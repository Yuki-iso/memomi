<script>
</script>

<svelte:head>
	<title>Memomi</title>
	<meta name="Memomi" content="Memomi" />
</svelte:head>

<section class="flex p-10 gap-6 pt-20">
	<div class="w-2/3 flex flex-col">
		<p class="text-4xl pb-10">JLPT N5</p>
		<div class="flex gap-4">
			<a
				href="/vocab"
				class="bg-gray-200 flex flex-col justify-between w-1/2 h-72 p-5 rounded-xl hover:bg-gray-300 cursor-pointer"
			>
				<div>
					<p class="text-6xl font-bold">語彙</p>
					<p class="text-3xl font-bold">Vocabulary</p>
				</div>
				<div class="flex justify-end gap-2 text-xl">
					<p class="text-blue-500">10</p>
					<p class="text-red-500">22</p>
					<p class="text-green-500">100</p>
				</div>
			</a>
			<a
				href="/"
				class="flex flex-col justify-between bg-gray-200 w-1/2 h-72 p-5 rounded-xl hover:bg-gray-300 cursor-pointer"
			>
				<div>
					<p class="text-6xl font-bold">漢字</p>
					<p class="text-3xl font-bold">Kanji</p>
				</div>
				<div class="flex justify-end gap-2 text-xl">
					<p class="text-blue-500">10</p>
					<p class="text-red-500">22</p>
					<p class="text-green-500">100</p>
				</div>
			</a>
		</div>
		<a href="/" class="bg-gray-200 h-96 my-4 p-5 rounded-xl hover:bg-gray-300 cursor-pointer">
			<p class="text-5xl font-bold">文法</p>
			<p class="text-2xl font-bold">Grammar</p>
		</a>
	</div>

	<div class="w-1/3">
		<p class="text-4xl pb-10">Stats</p>
		<div class="bg-gray-200 h-[40rem] rounded-xl"></div>
	</div>
</section>
