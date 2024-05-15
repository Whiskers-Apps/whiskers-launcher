<script lang="ts">
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import { writeSettings, type Settings } from '$lib/settings/settings';


	// ==============================
	// Props
	// ==============================
	export let settings: Settings;

	// ==============================
	// UI Elements
	// ==============================
	let splitResults = settings.split_results;
	let showSearchIcon = settings.show_search_icon;
	let showSettingsIcon = settings.show_settings_icon;
	let showPlaceholder = settings.show_placeholder;
	let accentSearchBorder = settings.accent_search_border;
	let hideOnBlur = settings.hide_on_blur;
	let borderRadius = settings.border_radius;
	let borderWidth = settings.border_width;

	// ==============================
	// Events
	// ==============================
	function updateSplitResults(value: CustomEvent<boolean>) {
		splitResults = value.detail;
		let newSettings = settings;
		newSettings.split_results = splitResults;
		writeSettings(newSettings);
	}

	function updateShowSearchIcon(value: CustomEvent<boolean>) {
		showSearchIcon = value.detail;
		let newSettings = settings;
		newSettings.show_search_icon = showSearchIcon;
		writeSettings(newSettings);
	}

	function updateShowSettingsIcon(value: CustomEvent<boolean>) {
		showSettingsIcon = value.detail;
		let newSettings = settings;
		newSettings.show_settings_icon = showSettingsIcon;
		writeSettings(newSettings);
	}

	function updateShowPlaceholder(value: CustomEvent<boolean>) {
		showPlaceholder = value.detail;
		let newSettings = settings;
		newSettings.show_placeholder = showPlaceholder;
		writeSettings(newSettings);
	}

	function updateAccentSearchBorder(value: CustomEvent<boolean>) {
		accentSearchBorder = value.detail;
		let newSettings = settings;
		newSettings.accent_search_border = accentSearchBorder;
		writeSettings(newSettings);
	}

	function updateHideOnBlur(value: CustomEvent<boolean>) {
		hideOnBlur = value.detail;
		let newSettings = settings;
		newSettings.hide_on_blur = hideOnBlur;
		writeSettings(newSettings);
	}

	function updateBorderRadius(value: CustomEvent<number>) {
		borderRadius = value.detail;
		let newSettings = settings;
		newSettings.border_radius = borderRadius;
		writeSettings(newSettings);
	}

	function updateBorderWidth(value: CustomEvent<number>) {
		borderWidth = value.detail;
		let newSettings = settings;
		newSettings.border_width = borderWidth;
		writeSettings(newSettings);
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
		title={`Border Radius ${borderRadius}`}
		description="Applies a border radius to the search box."
		min={0}
		max={48}
		step={1}
		value={borderRadius}
		on:slide={updateBorderRadius}
	/>

	<SliderSetting
		title={`Border Width ${borderWidth}`}
		description="Applies a border width to the search box."
		min={0}
		max={10}
		step={1}
		value={borderWidth}
		on:slide={updateBorderWidth}
	/>
</div>
