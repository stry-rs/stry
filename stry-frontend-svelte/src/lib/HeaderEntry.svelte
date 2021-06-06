<script lang="ts">
	export let selected: boolean = false;
	export let mobile: boolean = false;
	export let url: string;
	export let name: string;
</script>

<a
	sveltekit:prefetch
	href={url}
	class="
		{mobile ? "block" : ""}
		text-opacity-90
		transition-colors duration-75
		mx-3 my-2
		rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400
		{selected ? "text-blue-400" : "text-white hover:text-blue-400"}
		text-base font-medium
	"
>{name}</a>
