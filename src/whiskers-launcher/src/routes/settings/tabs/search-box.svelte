<script lang="ts">
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import FolderIcon from '$lib/icons/folder.svg?component';
	import TrashIcon from '$lib/icons/trash.svg?component';
	import { getSettings, writeSettings, type Settings } from '$lib/settings/settings';
	import { open } from '@tauri-apps/api/dialog';
	import { pictureDir } from '@tauri-apps/api/path';
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
	$: wallpaper = settings.wallpaper;

	// ==============================
	// Events
	// ==============================
	async function updateSplitResults(value: CustomEvent<boolean>) {
		dispatch('updateSplitResults', value.detail);
	}

	async function updateShowSearchIcon(value: CustomEvent<boolean>) {
		dispatch('updateShowSearchIcon', value.detail);
	}

	async function updateShowSettingsIcon(value: CustomEvent<boolean>) {
		dispatch('updateShowSettingsIcon', value.detail);
	}

	async function updateShowPlaceholder(value: CustomEvent<boolean>) {
		dispatch('updateShowPlaceholder', value.detail);
	}

	async function updateAccentSearchBorder(value: CustomEvent<boolean>) {
		dispatch('updateAccentSearchBorder', value.detail);
	}

	async function updateHideOnBlur(value: CustomEvent<boolean>) {
		dispatch('updateHideOnBlur', value.detail);
	}

	async function updateBorderRadius(value: CustomEvent<number>) {
		dispatch('updateBorderRadius', value.detail);
	}

	async function updateBorderWidth(value: CustomEvent<number>) {
		dispatch('updateBorderWidth', value.detail);
	}

	async function selectWallpaper(){
		const path = await open({
			defaultPath: await pictureDir(),
			filters:[
				{
					name: "Supported Types",
					extensions: ["jpg", "jpeg", "png", "gif", "webp"]
				}
			]
		});

		if(path){
            dispatch('updateWallpaper', path);
        }
	}

	async function clearWallpaper(){
		dispatch('updateWallpaper', null)
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

	<div class=" card space-y-2">
		<div>
			<p class=" text-xl font-medium">Wallpaper</p>
			<p class=" text-sub-text">Select the wallpaper. Webp images are prefered.</p>
		</div>
		<div class="flex items-center">
			<button class="flex items-center pt-2 w-full" on:click={selectWallpaper}>
				<FolderIcon class=" h-8 w-8" />
				{#if wallpaper === null}
					<div class="ml-2">
						<p>Select a image to use in the background.</p>
					</div>
				{:else}
					<div class="ml-2">
						<p>{wallpaper}</p>
					</div>
				{/if}
			</button>
			{#if wallpaper !== null}
				<button class="p-2 hover-bg-danger rounded-full" on:click={clearWallpaper}>
					<TrashIcon class="h-6 w-6"/>
				</button>
			{/if}
		</div>
	</div>
</div>
