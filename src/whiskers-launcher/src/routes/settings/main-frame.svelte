<script lang="ts">
	import { page } from '$app/stores';
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { goToRoute } from '$lib/utils/routing';
	import InfoIcon from '$lib/icons/info.svg?component';
	import HomeIcon from '$lib/icons/home.svg?component';
	import RoundIcon from '$lib/icons/round.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import SearchGlobeIcon from '$lib/icons/search-globe.svg?component';
	import PaletteIcon from '$lib/icons/palette.svg?component';
	import PuzzleIcon from '$lib/icons/puzzle.svg?component';
	import { createEventDispatcher, onMount } from 'svelte';
	import { init, state } from './main-frame-vm';

	let currentRoute = $page.route.id;
	$: uiState = $state;

	onMount(async () => {
		init();
	});

	function getHighlightClasses(route: string): string {
		if (`/settings${route}` === currentRoute) {
			return 'bg-tertiary';
		}

		return '';
	}
</script>

<div class="bg-black">
	{#if !uiState.loading}
		{@html uiState.css}
		<div class="h-screen w-full bg-secondary flex p-4 text-text">
			<div class=" items-start space-y-2 overflow-auto pl-2 pr-6 w-fit min-w-[240px]">
				<button
					class={`text-lg flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('')}`}
					on:click={() => {
						goToRoute('/settings');
					}}
				>
					<HomeIcon class=" h-5 w-5 mr-2" />
					<p>General</p>
				</button>
				<button
					class={`text-lg flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/search-box')}`}
					on:click={() => {
						goToRoute('/settings/search-box');
					}}
				>
					<RoundIcon class=" h-5 w-5 mr-2" />
					<p>Search Box</p>
				</button>

				<button
					class={`text-lg flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/search-results')}`}
					on:click={() => {
						goToRoute('/settings/search-results');
					}}
				>
					<SearchIcon class=" h-5 w-5 mr-2" />
					<p>Search Results</p>
				</button>

				<button
					class={`text-lg flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/search-engines')}`}
					on:click={() => {
						goToRoute('/settings/search-engines');
					}}
				>
					<SearchGlobeIcon class=" h-5 w-5 mr-2" />
					<p>Search Engines</p>
				</button>

				<button
					class={`text-lg  flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/theming')}`}
					on:click={() => {
						goToRoute('/settings/theming');
					}}
				>
					<PaletteIcon class=" h-5 w-5 mr-2" />
					<p>Theming</p>
				</button>

				<button
					class={`text-lg flex items-center rounded-lg hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/extensions')}`}
					on:click={() => {
						goToRoute('/settings/extensions');
					}}
				>
					<PuzzleIcon class=" h-5 w-5 mr-2" />
					<p>Extensions</p>
				</button>

				<button
					class={`text-lg rounded-lg flex items-center hover-bg-tertiary w-full text-start p-2 ${getHighlightClasses('/about')}`}
					on:click={() => {
						goToRoute('/settings/about');
					}}
				>
					<InfoIcon class=" h-5 w-5 mr-2" />
					<p>About</p>
				</button>
			</div>
			<div class="flex-grow bg-background rounded-xl border-tertiary p-8 overflow-auto">
				<slot></slot>
			</div>
		</div>
	{/if}
</div>
