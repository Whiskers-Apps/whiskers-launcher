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
	import MainFrame from './main-frame.svelte';
	import {
		state,
		init,
		onSetScaling,
		onSetShowRecentApps,
		onSetAutoStart,

		onSetFirstKey,

		onSetSecondKey,

		onSetThirdKey



	} from './general-settings-page-vm';
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import Warning from '$lib/icons/warning.svg?component';
	import Select from '$lib/components/select.svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});

	let settings: Settings | null = null;
	let css = '';

	onMount(async () => {
		settings = await getSettings();
	});

	// ==========================
	// Settings Events
	// ==========================

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

	async function updateWallpaper(event: CustomEvent<string | null>) {
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

<MainFrame>
	{#if !uiState.loading}
		<div class=" space-y-8">
			<div>
				{#if !uiState.isWayland}
					<p class=" text-xl font-medium">Shortcut Keys</p>
					<p class=" text-sub-text">The shortcut to spawn the search window.</p>
					<div class=" space-y-2">
						<div>
							<p class="">First Key</p>
							<Select
								values={uiState.firstKeyOptions}
								selectedValue={uiState.settings.first_key}
								on:selection={onSetFirstKey}
							/>
						</div>
						<div>
							<p class="">Second Key</p>
							<Select
								values={uiState.secondKeyOptions}
								selectedValue={uiState.settings.second_key ?? '-'}
								on:selection={onSetSecondKey}
							/>
						</div>
						<div>
							<p class="">Third Key</p>
							<Select
								values={uiState.thirdKeyOptions}
								selectedValue={uiState.settings.third_key}
								on:selection={onSetThirdKey}
							/>
						</div>
					</div>
					{#if uiState.showShortcutWarnings}
						<div class=" flex items-center text-warning">
							<Warning class="w-6 h-6 mr-2" />
							<p>
								The launcher needs a restart to apply this setting. Please restart using the system
								tray app
							</p>
						</div>
					{/if}
				{:else}
					<div class=" flex items-center text-warning">
						<Warning class="w-6 h-6 mr-2" />
						<p>
							Wayland detected. Please add the shortcut to run "whiskers-launcher" manually from
							your DE/WM settings.
						</p>
					</div>
				{/if}
			</div>
			<SliderSetting
				title={`Scaling (${uiState.settings.scaling.toFixed(2)})`}
				description="This scaling is applied in the search window"
				min={0.5}
				max={2}
				step={0.1}
				value={uiState.settings.scaling}
				on:slide={onSetScaling}
			/>

			{#if uiState.os !== 'windows'}
				<ToggleSetting
					title="Recent Apps"
					description="When enabled, it shows the most recent opened apps when opening the launcher"
					toggled={uiState.settings.show_recent_apps}
					on:toggle={onSetShowRecentApps}
				/>
			{/if}

			<ToggleSetting
				title="Auto Start"
				description="When enabled, it auto starts the app at login"
				toggled={uiState.settings.auto_start}
				on:toggle={onSetAutoStart}
			/>
		</div>
	{/if}
</MainFrame>
