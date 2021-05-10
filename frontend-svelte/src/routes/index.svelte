<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import type { IStory } from "$lib/types";
	import { PartKind, Rating, State } from "$lib/types";

	import CardStory from '$lib/components/CardStory.svelte';

	let stories: IStory[] = [
		{
			id: "",

			name: "The Royal Night Sky",
			summary: [
				{
					id: "",
					kind: PartKind.Paragraph,
					content: "4 years without a job, and Hajime’s desperate. If being one of the thousand nobody workers at Jabberwock palace is what it takes to get by, then so be it. His first priority should probably be to lay low, and therefore he should stop cursing out the king, but when he's doing his job so terribly then what is Hajime supposed to do? Leave him be?",
				},
				{
					id: "",
					kind: PartKind.Paragraph,
					content: "Komaeda Nagito has been a ruler since the young age of 16 after an assassination attempt ended with both his parents dead and him the only survivor, and he can't remember anything rather then the pressure it takes to rule a kingdom and keep the peace. This loudmouth servant is a nice change from the normal. Shrouded in mystery and a past that he would obviously rather keep hidden, Nagito is determined to figure out Hinata Hajime, and whether he's good or bad luck.",
				},
				{
					id: "",
					kind: PartKind.Paragraph,
					content: "Junko’s just starting a war that's destroying the world, so you know, the normal.",
				},
			],

			rating: Rating.Teen,
			state: State.InProgress,

			authors: [
				{
					id: "",
					name: "Im_Miss_Understood",
				},
			],
			commissioners: [],
			dedicatees: [],

			origins: [
				{
					id: "",
					text: "Dangan Ronpa - All Media Types",
				},
				{
					id: "",
					text: "Dangan Ronpa - All Media Types",
				},
			],
			warnings: [
				{
					id: "/",
					text: "Creator Chose Not To Use Archive Warnings",
				}
			],
			pairings: [
				{
					id: "/",
					text: "Hinata Hajime/Komaeda Nagito",
					minor: false,
				},
				{
					id: "/",
					text: "Hinata Hajime/Komaeda Nagito",
					minor: true,
				},
			],
			characters: [
				{
					id: "/",
					text: "Hinata Hajime",
					minor: false,
				},
				{
					id: "/",
					text: "Komaeda Nagito",
					minor: false,
				},
				{
					id: "/",
					text: "Kamukura Izuru",
					minor: false,
				},
				{
					id: "/",
					text: "Naegi Makoto",
					minor: false,
				},
				{
					id: "/",
					text: "Enoshima Junko",
					minor: false,
				},
				{
					id: "/",
					text: "Nanami Chiaki",
					minor: false,
				},
				{
					id: "/",
					text: "Class 77 (Dangan Ronpa)",
					minor: false,
				},
				{
					id: "/",
					text: "Naegi Komaru",
					minor: true,
				}
			],
			tags: [
				{
					id: "/",
					text: "Alternate Universe - Royalty",
				}
			],
		}
	];
</script>

<svelte:head>
	<title>home | story</title>
</svelte:head>

{#each stories as story (story.id)}
	<CardStory {story} />

	<div class="hidden sm:block sm:px-6 lg:px-8 text-sm" aria-hidden="true">
		<div class="border-t border-gray-700"></div>
	</div>
{/each}
