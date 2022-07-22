<script context="module" lang="ts">
	// export const prerender = true;

	export async function load({ params, fetch, session, stuff }) {
		const res = await fetch("/api/v1/stories");

		// TODO: check response status

		const content = res.ok && await res.json();

		return {};
	}
</script>

<script lang="ts">
	import type { IStory } from "../utils/types";
	import { PartKind, Rating, State } from "../utils/types";

	import CardStory from '$lib/CardStory.svelte';

	let stories: IStory[] = [
		{
			id: "7Xo1WwlLKt2yHVrzvHQRh",

			name: "The Royal Night Sky",
			summary: [
				{
					id: "Gk3EHSuIx7BRkm6aUO1VW",
					kind: PartKind.Paragraph,
					content: "4 years without a job, and Hajime’s desperate. If being one of the thousand nobody workers at Jabberwock palace is what it takes to get by, then so be it. His first priority should probably be to lay low, and therefore he should stop cursing out the king, but when he's doing his job so terribly then what is Hajime supposed to do? Leave him be?",
				},
				{
					id: "h-cJaVwHWX9Ky0clGZsVi",
					kind: PartKind.Paragraph,
					content: "Komaeda Nagito has been a ruler since the young age of 16 after an assassination attempt ended with both his parents dead and him the only survivor, and he can't remember anything rather then the pressure it takes to rule a kingdom and keep the peace. This loudmouth servant is a nice change from the normal. Shrouded in mystery and a past that he would obviously rather keep hidden, Nagito is determined to figure out Hinata Hajime, and whether he's good or bad luck.",
				},
				{
					id: "wFu_Lh6ioguc2FkzHk2JH",
					kind: PartKind.Paragraph,
					content: "Junko’s just starting a war that's destroying the world, so you know, the normal.",
				},
			],

			rating: Rating.Teen,
			state: State.InProgress,

			authors: [
				{
					id: "B0kIpGiXKliuYVld9CpVb",
					name: "Im_Miss_Understood",
				},
			],
			commissioners: [],
			dedicatees: [],

			origins: [
				{
					id: "X_mLpi2_682Y7sAmkEtxR",
					text: "Dangan Ronpa - All Media Types",
				},
				{
					id: "q04-jayoO2Iyov9x0EX2H",
					text: "Dangan Ronpa - All Media Types",
				},
			],
			warnings: [
				{
					id: "3FRWNIgbozvmwW-mVOslF",
					text: "Creator Chose Not To Use Archive Warnings",
				}
			],
			pairings: [
				{
					id: "fOI5Au3veY6n9tLaxJWTv",
					text: "Hinata Hajime/Komaeda Nagito",
					minor: false,
				},
				{
					id: "WXqjzCAAT-TnfJkSPlDp6",
					text: "Hinata Hajime/Komaeda Nagito",
					minor: true,
				},
			],
			characters: [
				{
					id: "xAqSuVkcqEWqHst1ZG9pA",
					text: "Hinata Hajime",
					minor: false,
				},
				{
					id: "AclZkyGg9bQHXFOKjCREs",
					text: "Komaeda Nagito",
					minor: false,
				},
				{
					id: "VFEreptVD6JC4_AKT1P-x",
					text: "Kamukura Izuru",
					minor: false,
				},
				{
					id: "As1gXSYeNxvOYCPY0-QuQ",
					text: "Naegi Makoto",
					minor: false,
				},
				{
					id: "rGlXkxXUuCStZ-_BPyqTO",
					text: "Enoshima Junko",
					minor: false,
				},
				{
					id: "Uf8rWpDlY6Y0wiz9B5S1c",
					text: "Nanami Chiaki",
					minor: false,
				},
				{
					id: "ZWnGeMZEilcTw2P4vX_Cn",
					text: "Class 77 (Dangan Ronpa)",
					minor: false,
				},
				{
					id: "KBwnCCrAk5K7n944Ir7jU",
					text: "Naegi Komaru",
					minor: true,
				}
			],
			tags: [
				{
					id: "us2kgW0Lztq3Qc28e1eYK",
					text: "Alternate Universe - Royalty",
				}
			],
		}
	];
</script>

<svelte:head>
	<title>home | stry</title>
</svelte:head>

{#each stories as story (story.id)}
	<CardStory {story} />

	<div class="hidden sm:block sm:px-6 lg:px-8 text-sm" aria-hidden="true">
		<div class="border-t border-gray-700"></div>
	</div>
{/each}
