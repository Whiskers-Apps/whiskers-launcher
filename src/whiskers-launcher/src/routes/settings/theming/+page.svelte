<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import {
		init,
		onExportTheme,
		onImportTheme,
		onOpenStore,
		onSetAccentColor,
		onSetBackgroundColor,
		onSetDangerColor,
		onSetOnAccentColor,
		onSetOnDangerColor,
		onSetSecondaryColor,
		onSetSubTextColor,
		onSetTertiaryColor,
		onSetTextColor,
		onSetWarningColor,
		state
	} from './theming-vm';
	import SecondaryButton from '$lib/components/secondary-button.svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">
			<div class=" flex">
				<SecondaryButton text="Themes Store" on:click={onOpenStore} />
				<SecondaryButton text="Import" on:click={onImportTheme} />
				<SecondaryButton text="Export" on:click={onExportTheme} />
			</div>
			<div class=" card space-y-2">
				<div>
					<p class=" text-xl font-medium">Background Colors</p>
					<p class=" text-sub-text">
						The most used colors. They are mostly the background of the app, ui elements and border
						of elements.
					</p>
				</div>
				<div class=" flex gap-4">
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-background rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.background}
							on:input={onSetBackgroundColor}
						/>
						<p class=" flex-grow text-center">Background</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-secondary rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.secondary}
							on:input={onSetSecondaryColor}
						/>
						<p class=" flex-grow text-center">Secondary</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-tertiary rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.tertiary}
							on:input={onSetTertiaryColor}
						/>
						<p class=" flex-grow text-center">Tertiary</p>
					</div>
				</div>
			</div>
			<div class=" card space-y-2">
				<div>
					<p class=" text-xl font-medium">Accent Colors</p>
					<p class=" text-sub-text">These colors are mostly used in buttons;</p>
				</div>
				<div class=" flex gap-4">
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-accent rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.accent}
							on:input={onSetAccentColor}
						/>
						<p class=" flex-grow text-center">Accent</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-warning rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.warning}
							on:input={onSetWarningColor}
						/>
						<p class=" flex-grow text-center">Warning</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-danger rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.danger}
							on:input={onSetDangerColor}
						/>
						<p class=" flex-grow text-center">Danger</p>
					</div>
				</div>
				<div class=" flex gap-4">
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-on-accent rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.on_accent}
							on:input={onSetOnAccentColor}
						/>
						<p class=" flex-grow text-center">On Accent</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-on-danger rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.on_danger}
							on:input={onSetOnDangerColor}
						/>
						<p class=" flex-grow text-center">On Danger</p>
					</div>
				</div>
			</div>
			<div class=" card space-y-2">
				<div>
					<p class=" text-xl font-medium">Text Colors</p>
					<p class=" text-sub-text">The text colors.</p>
				</div>
				<div class=" flex gap-4">
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] class bg-text rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.text}
							on:input={onSetTextColor}
						/>
						<p class=" flex-grow text-center">Text</p>
					</div>
					<div class=" flex flex-col flex-grow">
						<input
							class=" h-[100px] bg-sub-text class rounded-lg hover:brightness-90 border-tertiary cursor-pointer w-full"
							type="color"
							value={uiState.settings.theme.sub_text}
							on:input={onSetSubTextColor}
						/>
						<p class=" flex-grow text-center">Sub Text</p>
					</div>
				</div>
			</div>
		</div>
	{/if}
</MainFrame>

<style scoped>
	input[type='color']::-webkit-color-swatch {
		border: none;
	}
</style>
