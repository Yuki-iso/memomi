<script>
  import { enhance } from '$app/forms';

  export let form;
</script>

<svelte:head>
  <title>Register Memomi</title>
</svelte:head>

<div class="flex justify-center items-center h-screen">
	<form method="POST" use:enhance class="flex flex-col w-1/4 text-2xl gap-5">
		<p class="text-center font-jomhuria text-6xl my-4">Create your account</p>
		<input
			type="text"
			placeholder="Username"
      name="username"
			class="pl-8 bg-gray-100 text-gray-500 p-2 rounded-xl"
		/>
		<input type="email" name="email" placeholder="Email" class="pl-8 bg-gray-100 text-gray-500 p-2 rounded-xl" />
		<input
			type="password"
      name="password"
			placeholder="Password"
			class="bg-gray-100 text-gray-500 p-2 rounded-xl pl-8"
		/>
		<input
			type="password"
			placeholder="Confirm password"
			class="bg-gray-100 text-gray-500 p-2 rounded-xl pl-8"
		/>
		<button type="submit" class="p-2 my-4 rounded-xl text-xl text-white font-bold bg-gray-500"
			>Create account</button
		>
		<hr class="bg-black h-[2px]" />
		<button type="submit" class="p-2 my-4 rounded-xl text-xl text-white font-bold bg-gray-500"
			>Google</button
		>
	</form>
</div>
