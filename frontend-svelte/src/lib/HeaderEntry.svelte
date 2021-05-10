<script lang="ts">
	export let selected: boolean = false;
	export let mobile: boolean = false;
	export let url: string;
	export let name: string;
</script>

{#if mobile}
	{#if selected}
		<a sapper:prefetch href={url} class="text-opacity-90 text-blue-400 block mx-3 my-2 text-base font-medium  rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">{name}</a>
	{:else}
		<a sapper:prefetch href={url} class="text-opacity-90 text-white hover:text-blue-400 transition-colors duration-75 block mx-3 my-2 text-base font-medium  rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">{name}</a>
	{/if}
{:else}
	{#if selected}
		<a sapper:prefetch href={url} class="text-opacity-90 text-blue-400 mx-3 my-2 text-base font-medium  rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">{name}</a>
	{:else}
		<a sapper:prefetch href={url} class="text-opacity-90 text-white hover:text-blue-400 transition-colors duration-75 mx-3 my-2 text-base font-medium  rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">{name}</a>
	{/if}
{/if}
