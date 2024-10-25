<script lang="ts">
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import { onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import {
		init,
		onClearWallpaper,
		onSelectWallpaper,
		onSetAccentBorder,
		onSetBorderRadius,
		onSetBorderWidth,
		onSetHideOnBlur,
		onSetShowPlaceholder,
		onSetShowSearchIcon,
		onSetShowSettingsIcon,
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
				title="Search Icon"
				description="Show the search icon."
				toggled={uiState.settings.show_search_icon}
				on:toggle={onSetShowSearchIcon}
			/>

			<ToggleSetting
				title="Settings Icon"
				description="Show the settings icon."
				toggled={uiState.settings.show_settings_icon}
				on:toggle={onSetShowSettingsIcon}
			/>

			<ToggleSetting
				title="Placeholder"
				description="Show the search placeholder."
				toggled={uiState.settings.show_placeholder}
				on:toggle={onSetShowPlaceholder}
			/>

			<ToggleSetting
				title="Hide On Blur"
				description="Close the launcher when clicking outside the search box. Only applies when wallpaper is set."
				toggled={uiState.settings.hide_on_blur}
				on:toggle={onSetHideOnBlur}
			/>

			<SliderSetting
				title={`Border Radius (${uiState.settings.border_radius})`}
				description="Search box corner radius."
				min={0}
				max={48}
				step={1}
				value={uiState.settings.border_radius}
				on:slide={onSetBorderRadius}
			/>

			<SliderSetting
				title={`Border Width (${uiState.settings.border_width})`}
				description="Search box border width."
				min={0}
				max={10}
				step={1}
				value={uiState.settings.border_width}
				on:slide={onSetBorderWidth}
			/>

			<ToggleSetting
				title="Accent Border"
				description="Use accent color for the search box border."
				toggled={uiState.settings.accent_border}
				on:toggle={onSetAccentBorder}
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
