<script lang="ts">
	import Header from '$lib/Header.svelte';

	import '../app.css';
</script>

<main class="container mx-auto my-2 max-w-screen-xl">
	<Header segment=undefined/>

	<slot />
</main>
