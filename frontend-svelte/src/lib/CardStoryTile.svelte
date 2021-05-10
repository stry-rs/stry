<script lang="ts">
	import { Rating, State } from "../types";

	export let rating: Rating;
	export let warning: boolean;
	export let state: State;
</script>

<a href="/" class="w-9 h-9 mt-1.5 mr-3 flex flex-col rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400">
	<div class="flex">
		<div
			class="block w-4 h-4 mr-1 rounded"
			class:bg-red-400={rating === Rating.Explicit}
			class:bg-yellow-400={rating === Rating.Mature}
			class:bg-blue-400={rating === Rating.Teen}
			class:bg-gray-100={rating === Rating.General}
		>
			<span class="sr-only">Rating</span>
		</div>
		<div
			class="block w-4 h-4 rounded"
			class:bg-gray-100={warning === false}
			class:bg-red-400={warning === true}
		>
			<span class="sr-only">Warning</span>
		</div>
	</div>
	<div class="flex justify-center">
		<div
			class="block w-4 h-4 mt-1 rounded"
			class:bg-green-400={state === State.Completed}
			class:bg-blue-400={state === State.InProgress}
			class:bg-yellow-400={state === State.Hiatus}
			class:bg-red-400={state === State.Abandoned}
		>
			<span class="sr-only">State</span>
		</div>
	</div>
</a>
