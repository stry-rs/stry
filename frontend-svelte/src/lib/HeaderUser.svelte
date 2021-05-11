<script lang="ts">
	import { fade } from "svelte/transition";

	import clickOutside from "../utils/clickout";

	let open = false;
</script>

<div class="ml-3 relative" use:clickOutside={() => { if (open) { open = false; } }}>
	<div>
		<button
			type="button"
			class="bg-gray-800 flex text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400"
			id="user-menu"
			aria-expanded="false"
			aria-haspopup="true"
			on:click={_ => open = !open}
		>
			<span class="sr-only text-white">Open user menu</span>
			<img
				class="h-8 w-8 rounded-full"
				src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
				alt=""
			/>
		</button>
	</div>

	{#if open}
		<div
			class="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-gray-800 ring-1 ring-black ring-opacity-5 focus:outline-none"
			role="menu"
			aria-orientation="vertical"
			aria-labelledby="user-menu"
			transition:fade="{{ duration: 75 }}"
		>
			<a
				href="/"
				class="block px-4 py-2 text-sm text-gray-100 hover:bg-gray-700"
				role="menuitem"
			>Your Profile</a>
			<a
				href="/"
				class="block px-4 py-2 text-sm text-gray-100 hover:bg-gray-700"
				role="menuitem"
			>Settings</a>
			<a
				href="/"
				class="block px-4 py-2 text-sm text-gray-100 hover:bg-gray-700"
				role="menuitem"
			>Sign out</a>
		</div>
	{/if}
</div>
