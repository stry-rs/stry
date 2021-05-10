<script lang="ts">
	import { TagKind, TagKindUtil } from "../types";

	export let id: string;
	export let kind: TagKind;

	let url: string = TagKindUtil.asPathPrefix(kind) + id;
</script>

<a
	href={url}

	class="
		inline-block
		text-sm
		mr-1.5 mb-1.5 px-2 py-0.5
		transition-colors duration-75
		rounded focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-900 focus:ring-blue-400
	"


	class:bg-red-400={kind === TagKind.Warning}
	class:hover:bg-red-500={kind === TagKind.Warning}

	class:bg-yellow-400={kind === TagKind.Pairing}
	class:hover:bg-yellow-500={kind === TagKind.Pairing}

	class:bg-blue-400={kind === TagKind.Character}
	class:hover:bg-blue-500={kind === TagKind.Character}

	class:bg-gray-400={kind === TagKind.General}
	class:hover:bg-gray-500={kind === TagKind.General}


	class:tag-minor={TagKindUtil.isMinor(kind)}

	class:gradient-stripes-yellow-400-500={kind === TagKind.PairingMinor}
	class:hover:gradient-stripes-yellow-500-600={kind === TagKind.PairingMinor}

	class:gradient-stripes-blue-400-500={kind === TagKind.CharacterMinor}
	class:hover:gradient-stripes-blue-500-600={kind === TagKind.CharacterMinor}
>
	<slot></slot>
</a>
