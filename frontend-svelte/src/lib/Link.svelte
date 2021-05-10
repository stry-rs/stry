<script lang="ts">
	import { Contrast } from "../types";

	export let href: string;
	export let contrast: Contrast = Contrast.High;
</script>

<a
	{href}
	class="
		text-white
		hover:text-blue-400 transition-colors duration-75
		rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400
	"
	class:text-opacity-90={ contrast === Contrast.High }
	class:text-opacity-60={ contrast === Contrast.Low }
	class:text-opacity-40={ contrast === Contrast.Disabled }
>
	<slot></slot>
</a>
