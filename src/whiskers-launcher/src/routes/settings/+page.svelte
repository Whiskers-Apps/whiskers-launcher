<script lang="ts">
	import { getSettings, getThemeCss, type Settings } from '$lib/settings/settings';
	import { onMount } from 'svelte';
	import Navbar from './navbar.svelte';
	import GeneralTab from './tabs/general.svelte';
	import SearchBoxTab from './tabs/search-box.svelte';
	import SearchResultsTab from './tabs/search-results.svelte';
	import SearchEnginesTab from './tabs/search-engines.svelte';
	import ThemingTab from './tabs/theming.svelte';
	import ExtensionsTab from './tabs/extensions.svelte';
	import AboutTab from './tabs/about.svelte';

	// ===========================
	// UI
	// ===========================
	let settings: Settings | null = null;
	let themeClasses = '';

	let selectedTab = 0;

	// ===========================
	// UI Functions
	// ===========================
	onMount(async () => {
		settings = await getSettings();
		themeClasses = getThemeCss(settings);
	});

	// ===========================
	// UI Events
	// ===========================
	function selectTab(event: CustomEvent<number>) {
		selectedTab = event.detail;
	}
</script>

{#if settings !== null}
	{@html themeClasses}
	<div class=" bg-background h-screen overflow-auto text-text flex">
		<Navbar on:tabSelected={selectTab} />

		<div class="  p-4 max-w-[800px] flex-grow">
			{#if selectedTab === 0}
				<GeneralTab {settings} />
			{/if}
			{#if selectedTab === 1}
				<SearchBoxTab {settings} />
			{/if}
			{#if selectedTab === 2}
				<SearchResultsTab {settings} />
			{/if}
			{#if selectedTab === 3}
				<SearchEnginesTab {settings} />
			{/if}
			{#if selectedTab === 4}
				<ThemingTab {settings} />
			{/if}
			{#if selectedTab === 5}
				<ExtensionsTab {settings} />
			{/if}
			{#if selectedTab === 6}
				<AboutTab {settings} />
			{/if}
		</div>
	</div>
{/if}
