<script>
	let levels = [
		{
			name: 'N5',
			description: 'Basics'
		},
		{
			name: 'N4',
			description: 'Beginner'
		},
		{
			name: 'N3',
			description: 'Intermediate'
		},
		{
			name: 'N2',
			description: 'Advanced'
		},
		{
			name: 'N1',
			description: 'Expert'
		}
	];
</script>

<div class="flex justify-center items-center h-screen">
	<div class="flex flex-col w-1/2">
		<p class="text-center font-jomhuria text-6xl my-4">Select your JLPT level</p>
		<div class="flex flex-wrap justify-center">
			{#each levels as level}
				<div
					class="w-1/3 h-60 border-8 border-white bg-gray-100 rounded-3xl flex flex-col justify-center items-center hover:bg-gray-300 cursor-pointer"
				>
					<p class="font-jomhuria text-6xl">{level.name}</p>
					<p class="font-jomhuria text-5xl text-gray-400">{level.description}</p>
				</div>
			{/each}
		</div>
		<div class="flex justify-center my-4">
			<a
				href="/dashboard"
				class="p-2 w-1/4 text-center rounded-xl text-4xl font-jomhuria bg-gray-100">Next step</a
			>
		</div>
	</div>
</div>
