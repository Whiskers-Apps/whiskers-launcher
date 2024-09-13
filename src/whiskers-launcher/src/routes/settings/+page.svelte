<script lang="ts">
	import {
		getSettings,
		getThemeCss,
		writeSettings,
		type Theme,
		type Settings,
		type ExtensionSetting
	} from '$lib/settings/settings';
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
	let css = '';
	let tabContent: HTMLDivElement;

	let selectedTab = 0;

	// ===========================
	// UI Functions
	// ===========================
	onMount(async () => {
		settings = await getSettings();
		css = getThemeCss(settings);
	});

	// ===========================
	// UI Events
	// ===========================
	function selectTab(event: CustomEvent<number>) {
		selectedTab = event.detail;
		tabContent.scrollTop = 0;
	}

	// ==========================
	// Settings Events
	// ==========================
	async function updateFirstKey(event: CustomEvent<string>) {
		settings!!.first_key = event.detail;
		writeSettings(settings!!);
	}

	async function updateSecondKey(event: CustomEvent<string | null>) {
		settings!!.second_key = event.detail;
		writeSettings(settings!!);
	}

	async function updateThirdKey(event: CustomEvent<string>) {
		settings!!.third_key = event.detail;
		writeSettings(settings!!);
	}

	async function updateScaling(event: CustomEvent<number>) {
		settings!!.scaling = event.detail;
		writeSettings(settings!!);
	}

	async function updateAutoStart(event: CustomEvent<boolean>) {
		settings!!.auto_start = event.detail;
		writeSettings(settings!!);
	}

	async function updateShowRecentApps(event: CustomEvent<boolean>) {
		settings!!.show_recent_apps = event.detail;
		writeSettings(settings!!);
	}

	async function updateSplitResults(event: CustomEvent<boolean>) {
		settings!!.split_results = event.detail;
		writeSettings(settings!!);
	}

	async function updateShowSearchIcon(event: CustomEvent<boolean>) {
		settings!!.show_search_icon = event.detail;
		writeSettings(settings!!);
	}

	async function updateShowSettingsIcon(event: CustomEvent<boolean>) {
		settings!!.show_settings_icon = event.detail;
		writeSettings(settings!!);
	}

	async function updateShowPlaceholder(event: CustomEvent<boolean>) {
		settings!!.show_placeholder = event.detail;
		writeSettings(settings!!);
	}

	async function updateAccentSearchBorder(event: CustomEvent<boolean>) {
		settings!!.accent_search_border = event.detail;
		writeSettings(settings!!);
	}

	async function updateHideOnBlur(event: CustomEvent<boolean>) {
		settings!!.hide_on_blur = event.detail;
		writeSettings(settings!!);
	}

	async function updateBorderRadius(event: CustomEvent<number>) {
		settings!!.border_radius = event.detail;
		writeSettings(settings!!);
	}

	async function updateBorderWidth(event: CustomEvent<number>) {
		settings!!.border_width = event.detail;
		writeSettings(settings!!);
	}

	async function updateWallpaper(event: CustomEvent<string | null>){
		settings!!.wallpaper = event.detail;
		writeSettings(settings!!);
	}

	async function updateHighlightBackground(event: CustomEvent<boolean>) {
		settings!!.highlight_selected_background = event.detail;
		writeSettings(settings!!);
	}

	async function updateShowAltHint(event: CustomEvent<boolean>) {
		settings!!.show_alt_hint = event.detail;
		writeSettings(settings!!);
	}

	async function updateResultsCount(event: CustomEvent<number>) {
		settings!!.results_count = event.detail;
		writeSettings(settings!!);
	}

	async function refreshBlacklist() {
		settings!!.blacklist = (await getSettings()).blacklist;
	}

	async function updateSearchKeyword(event: CustomEvent<string>) {
		settings!!.search_keyword = event.detail;
		writeSettings(settings!!);
	}

	async function refreshSearchEngines() {
		settings!!.search_engines = (await getSettings()).search_engines;
	}

	async function updateTheme(event: CustomEvent<Theme>) {
		settings!!.theme = event.detail;
		css = getThemeCss(settings!!);
		writeSettings(settings!!);
	}

	async function refreshExtensions() {
		settings!!.extensions = (await getSettings()).extensions;
	}

	async function updateExtensionsSettings(event: CustomEvent<ExtensionSetting[]>) {
		settings!!.extensions = event.detail;
		writeSettings(settings!!);
	}
</script>

{#if settings !== null}
	{@html css}
	<div class=" bg-background min-h-screen h-screen text-text flex">
		<Navbar on:tabSelected={selectTab} />

		<div bind:this={tabContent} class="  p-4 flex-grow flex justify-center overflow-auto">
			<div class=" w-full max-w-[800px]">
				{#if selectedTab === 0}
					<GeneralTab
						{settings}
						on:updateFirstKey={updateFirstKey}
						on:updateSecondKey={updateSecondKey}
						on:updateThirdKey={updateThirdKey}
						on:updateScaling={updateScaling}
						on:updateAutoStart={updateAutoStart}
						on:updateShowRecentApps={updateShowRecentApps}
					/>
				{/if}
				{#if selectedTab === 1}
					<SearchBoxTab
						{settings}
						on:updateSplitResults={updateSplitResults}
						on:updateShowSearchIcon={updateShowSearchIcon}
						on:updateShowSettingsIcon={updateShowSettingsIcon}
						on:updateShowPlaceholder={updateShowPlaceholder}
						on:updateAccentSearchBorder={updateAccentSearchBorder}
						on:updateHideOnBlur={updateHideOnBlur}
						on:updateBorderRadius={updateBorderRadius}
						on:updateBorderWidth={updateBorderWidth}
						on:updateWallpaper={updateWallpaper}
					/>
				{/if}
				{#if selectedTab === 2}
					<SearchResultsTab
						{settings}
						on:updateHighlightBackground={updateHighlightBackground}
						on:updateShowAltHint={updateShowAltHint}
						on:updateResultsCount={updateResultsCount}
						on:refreshBlacklist={refreshBlacklist}
					/>
				{/if}
				{#if selectedTab === 3}
					<SearchEnginesTab
						{settings}
						on:updateSearchKeyword={updateSearchKeyword}
						on:refreshSearchEngines={refreshSearchEngines}
					/>
				{/if}
				{#if selectedTab === 4}
					<ThemingTab {settings} on:updateTheme={updateTheme} />
				{/if}
				{#if selectedTab === 5}
					<ExtensionsTab
						{settings}
						on:refresh-extensions={refreshExtensions}
						on:updateExtensionsSettings={updateExtensionsSettings}
					/>
				{/if}
				{#if selectedTab === 6}
					<AboutTab />
				{/if}
			</div>
		</div>
	</div>
{/if}
