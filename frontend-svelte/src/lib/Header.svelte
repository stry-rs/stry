<script lang="ts">
	import { slide } from "svelte/transition";

	import NavbarEntry from "$lib/HeaderEntry.svelte";
	import NavbarUser from "$lib/HeaderUser.svelte";

	export let segment: string | undefined;

	const entries: { segment: string | undefined, url: string, name: string }[] = [
		{
			segment: "dashboard",
			url: "dashboard",
			name: "dashboard",
		},
		{
			segment: "team",
			url: "team",
			name: "team",
		},
		{
			segment: "projects",
			url: "projects",
			name: "projects",
		},
		{
			segment: "calender",
			url: "calender",
			name: "calendar",
		},
	];

	let open = false;
</script>

<nav>
	<div class="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8">
		<div class="relative flex items-center justify-between h-10">
			<div class="absolute inset-y-0 left-0 flex items-center md:hidden">
				<!-- Mobile menu button-->
				<button
					type="button"
					class="inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 transition-colors duration-75 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
					aria-controls="mobile-menu"
					aria-expanded="false"
					on:click={_ => open = !open}
				>
					<span class="sr-only">Open main menu</span>
					<svg
						class="{open ? "hidden" : "block"} h-6 w-6"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 6h16M4 12h16M4 18h16"
						/>
					</svg>
					<svg
						class="{open ? "block" : "hidden"} h-6 w-6"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						aria-hidden="true"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>
			<div class="flex-1 flex items-center justify-center md:items-stretch md:justify-start">
				<div class="flex-shrink-0 flex items-center">
					<a sveltekit:prefetch href="/" class="text-white hover:text-blue-400 transition-colors duration-75 font-bold tracking-widest my-2 rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">stry</a>
				</div>
				<div class="hidden md:block md:ml-3">
					<div class="flex">
						{#each entries as entry}
							<NavbarEntry selected={(entry.segment != undefined && entry.segment == segment)} url={entry.url} name={entry.name} mobile={false} />
						{/each}
					</div>
				</div>
			</div>
			<div class="flex-inital flex items-center justify-center md:items-stretch md:justify-start">
				<div class="hidden md:block md:ml-3">
					<input type="text" class=" text-white bg-gray-700 px-2 py-1 rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400" placeholder="search">
				</div>
			</div>
			<div class="absolute inset-y-0 right-0 flex items-center pr-2 md:static md:inset-auto md:ml-3 md:pr-0">
				<NavbarUser />
			</div>
		</div>
	</div>

	<!-- Mobile menu, show/hide based on menu state. -->
	{#if open}
		<div transition:slide="{{ duration: 150 }}">
			<div class="px-2 pt-2 pb-3 space-y-1">
				<div class="flex flex-col">
					{#each entries as entry}
						<NavbarEntry selected={(entry.segment != undefined && entry.segment == segment)} url={entry.url} name={entry.name} mobile={true} />
					{/each}

					<input type="text" class="text-white bg-gray-700 px-3 py-1 rounded w-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400" placeholder="search">
				</div>
			</div>
		</div>
	{/if}
</nav>
