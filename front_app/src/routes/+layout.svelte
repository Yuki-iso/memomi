<script>
	import Footer from './Footer.svelte';
import Header from './Header.svelte';
	import './styles.css';
</script>

<div class="app">
	<Header />
	<main>
		<slot />
	</main>
  <Footer />
</div>
