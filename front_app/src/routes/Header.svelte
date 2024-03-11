<script>
  import { page } from '$app/stores';  
</script>

<header class="w-full fixed flex justify-between p-4">
    <a href="/dashboard">
      <p class="font-jomhuria text-6xl">MEMOMI</p>
    </a>
    <a href="/">
    {#if $page.url.pathname === "/register"}
      <button class="p-3  rounded-xl text-xl font-bold bg-gray-100">Login</button>
    {:else if $page.url.pathname === "/login"}
      <button class="p-3  rounded-xl text-xl font-bold bg-gray-100">Register</button>
    {/if}
  </a>
</header>

