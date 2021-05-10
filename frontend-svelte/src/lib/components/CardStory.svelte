<script lang="ts">
	import type { IStory } from "$lib/types";
	import { Contrast, PartKind, TagKind } from "$lib/types";

	import Link from "$lib/components/Link.svelte";
	import MediaObject from "$lib/components/MediaObject.svelte";

	import CardStoryTile from "$lib/components/CardStoryTile.svelte";
	import Tag from "$lib/components/Tag.svelte";

	export let story: IStory;
</script>

<div class="px-3 sm:px-6 lg:px-8 my-2">
	<MediaObject>
		<span slot="tile">
			<CardStoryTile state={story.state} rating={story.rating} warning={story.warnings.length != 0} />
		</span>
		<span slot="title">
			<p class="text-base">
				<Link href={"/story/" + story.id} contrast={Contrast.High}>{story.name}</Link>
				<span class="text-opacity-60 text-white">by</span>
				{#each story.authors as author (author.id)}
					<Link href={"/author/" + author.id} contrast={Contrast.High}>{author.name}</Link>
				{/each}
			</p>
		</span>
		<span slot="sub">
			<p class="text-sm">
				{#each story.origins as origin, i (origin.id)}
					<Link href={"/origin/" + origin.id} contrast={Contrast.Low}>{origin.text}</Link>{#if i != (story.origins.length - 1)}<span class="text-opacity-60 text-white">, </span>{/if}
				{/each}
			</p>
		</span>
		<span slot="meta">
			<p class="text-sm text-opacity-60 text-white">13 Jan 2021</p>
		</span>
	</MediaObject>
	<div class="pt-2 text-sm text-opacity-60 text-white">
		{#each story.summary as part (part.id)}
			{#if part.kind === PartKind.Paragraph}
				<p class="pb-2">{part.content}</p>
			{/if}
		{/each}
	</div>
	<div class="text-sm">
		<ul class="flex flex-wrap">
			{#each story.warnings as tag (tag.id)}
				<li><Tag id={tag.id} kind={TagKind.Warning}>{tag.text}</Tag></li>
			{/each}
			{#each story.pairings as tag (tag.id)}
				<li><Tag id={tag.id} kind={tag.minor ? TagKind.PairingMinor : TagKind.Pairing}>{tag.text}</Tag></li>
			{/each}
			{#each story.characters as tag (tag.id)}
				<li><Tag id={tag.id} kind={tag.minor ? TagKind.CharacterMinor : TagKind.Character}>{tag.text}</Tag></li>
			{/each}
			{#each story.tags as tag (tag.id)}
				<li><Tag id={tag.id} kind={TagKind.General}>{tag.text}</Tag></li>
			{/each}
		</ul>
	</div>
	<div class="text-sm text-opacity-60 text-white flex">
		<p class="flex-grow"></p>
		<p>english | 3.062 words | 1 chapter</p>
	</div>
</div>
