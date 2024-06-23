<script lang="ts">
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import { getSettings, writeSettings, type Settings } from '$lib/settings/settings';
	import { createEventDispatcher } from 'svelte';


	// ==============================
	// Props
	// ==============================
	export let settings: Settings;
	let dispatch = createEventDispatcher();

	// ==============================
	// UI Elements
	// ==============================
	$: splitResults = settings.split_results;
	$: showSearchIcon = settings.show_search_icon;
	$: showSettingsIcon = settings.show_settings_icon;
	$: showPlaceholder = settings.show_placeholder;
	$: accentSearchBorder = settings.accent_search_border;
	$: hideOnBlur = settings.hide_on_blur;
	$: borderRadius = settings.border_radius;
	$: borderWidth = settings.border_width;

	// ==============================
	// Events
	// ==============================
	async function updateSplitResults(value: CustomEvent<boolean>) {
		dispatch("updateSplitResults", value.detail);
	}

	async function updateShowSearchIcon(value: CustomEvent<boolean>) {
		dispatch("updateShowSearchIcon", value.detail);
	}

	async function updateShowSettingsIcon(value: CustomEvent<boolean>) {
		dispatch("updateShowSettingsIcon", value.detail);
	}

	async function updateShowPlaceholder(value: CustomEvent<boolean>) {
		dispatch("updateShowPlaceholder", value.detail);
	}

	async function updateAccentSearchBorder(value: CustomEvent<boolean>) {
		dispatch("updateAccentSearchBorder", value.detail);
	}

	async function updateHideOnBlur(value: CustomEvent<boolean>) {
		dispatch("updateHideOnBlur", value.detail);
	}

	async function updateBorderRadius(value: CustomEvent<number>) {
		dispatch("updateBorderRadius", value.detail);
	}

	async function updateBorderWidth(value: CustomEvent<number>) {
		dispatch("updateBorderWidth", value.detail);
	}
</script>

<div class=" space-y-4">
	<ToggleSetting
		title="Split Results"
		description="When enabled, it splits the search bar and results."
		toggled={splitResults}
		on:toggle={updateSplitResults}
	/>

	<ToggleSetting
		title="Search Icon"
		description="When enabled, it shows the search icon."
		toggled={showSearchIcon}
		on:toggle={updateShowSearchIcon}
	/>

	<ToggleSetting
		title="Settings Icon"
		description="When enabled, it shows the settings icon."
		toggled={showSettingsIcon}
		on:toggle={updateShowSettingsIcon}
	/>

	<ToggleSetting
		title="Placeholder"
		description="When enabled, it shows the placeholder text."
		toggled={showPlaceholder}
		on:toggle={updateShowPlaceholder}
	/>

	<ToggleSetting
		title="Accent Border"
		description="When enabled, it uses the accent color as the border color."
		toggled={accentSearchBorder}
		on:toggle={updateAccentSearchBorder}
	/>

	<ToggleSetting
		title="Hide On Blur"
		description="When enabled, the search window closes when clicked outside the box."
		toggled={hideOnBlur}
		on:toggle={updateHideOnBlur}
	/>

	<SliderSetting
		title={`Border Radius (${borderRadius})`}
		description="Applies a border radius to the search box."
		min={0}
		max={48}
		step={1}
		value={borderRadius}
		on:slide={updateBorderRadius}
	/>

	<SliderSetting
		title={`Border Width (${borderWidth})`}
		description="Applies a border width to the search box."
		min={0}
		max={10}
		step={1}
		value={borderWidth}
		on:slide={updateBorderWidth}
	/>
</div>
