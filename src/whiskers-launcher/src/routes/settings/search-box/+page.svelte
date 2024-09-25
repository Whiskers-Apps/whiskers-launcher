<script lang="ts">
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import { onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import {
		init,
		onClearWallpaper,
		onSelectWallpaper,
		onSetAccentSearchBorder,
		onSetBorderRadius,
		onSetBorderWidth,
		onSetHideOnBlur,
		onSetShowPlaceholder,
		onSetShowSearchIcon,
		onSetShowSettingsIcon,
		onSetSplitResults,
		state
	} from './search-box-vm';
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import FolderIcon from '$lib/icons/folder.svg?component';
	import TrashIcon from '$lib/icons/trash.svg?component';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">
			<ToggleSetting
				title="Split Results"
				description="When enabled, it splits the search bar and results."
				toggled={uiState.settings.split_results}
				on:toggle={onSetSplitResults}
			/>
			<ToggleSetting
				title="Search Icon"
				description="When enabled, it shows the search icon."
				toggled={uiState.settings.show_search_icon}
				on:toggle={onSetShowSearchIcon}
			/>

			<ToggleSetting
				title="Settings Icon"
				description="When enabled, it shows the settings icon."
				toggled={uiState.settings.show_settings_icon}
				on:toggle={onSetShowSettingsIcon}
			/>

			<ToggleSetting
				title="Placeholder"
				description="When enabled, it shows the placeholder text."
				toggled={uiState.settings.show_placeholder}
				on:toggle={onSetShowPlaceholder}
			/>

			<ToggleSetting
				title="Accent Border"
				description="When enabled, it uses the accent color as the border color."
				toggled={uiState.settings.accent_search_border}
				on:toggle={onSetAccentSearchBorder}
			/>

			<ToggleSetting
				title="Hide On Blur"
				description="When enabled, the search window closes when clicked outside the box."
				toggled={uiState.settings.hide_on_blur}
				on:toggle={onSetHideOnBlur}
			/>

			<SliderSetting
				title={`Border Radius (${uiState.settings.border_radius})`}
				description="Applies a border radius to the search box."
				min={0}
				max={48}
				step={1}
				value={uiState.settings.border_radius}
				on:slide={onSetBorderRadius}
			/>

			<SliderSetting
				title={`Border Width (${uiState.settings.border_width})`}
				description="Applies a border width to the search box."
				min={0}
				max={10}
				step={1}
				value={uiState.settings.border_width}
				on:slide={onSetBorderWidth}
			/>
			<div class="space-y-2">
				<div>
					<p class=" text-xl font-medium">Wallpaper</p>
					<p class=" text-sub-text">Select the wallpaper. Webp images are prefered.</p>
				</div>
				<div class="flex items-center card">
					<button class="flex items-center w-full" on:click={onSelectWallpaper}>
						<FolderIcon class=" h-8 w-8" />
						{#if uiState.settings.wallpaper === null}
							<div class="ml-2">
								<p>Select a image to use in the background.</p>
							</div>
						{:else}
							<div class="ml-2">
								<p>{uiState.settings.wallpaper}</p>
							</div>
						{/if}
					</button>
					{#if uiState.settings.wallpaper !== null}
						<button
							class=" p-2 hover-bg-tertiary text-danger rounded-full hover-text-on-danger hover-bg-danger"
							on:click={onClearWallpaper}
						>
							<TrashIcon class=" " width="24" height="24" />
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</MainFrame>
