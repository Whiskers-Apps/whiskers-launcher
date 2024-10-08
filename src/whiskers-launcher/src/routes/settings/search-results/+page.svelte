<script>
	import { onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import {
		init,
		onOpenAddToBlacklistDialog,
		onOpenRemoveFromBlacklistDialog,
		onSetHighlightSelectedBackground,
		onSetResultsCount,
		onSetShowAltHint,
		state
	} from './search-results-vm';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import SliderSetting from '$lib/components/slider-setting.svelte';
	import PlusIcon from '$lib/icons/plus.svg?component';
	import MinusIcon from '$lib/icons/minus.svg?component';
	import QuestionIcon from '$lib/icons/question.svg?component';
	import { convertFileSrc } from '@tauri-apps/api/tauri';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">
			<ToggleSetting
				title="Highlight Background"
				description="When enabled, it changes the background color of the selected result."
				toggled={uiState.settings.highlight_selected_background}
				on:toggle={onSetHighlightSelectedBackground}
			/>

			<ToggleSetting
				title="Show Alt Hint"
				description="When enabled, it shows the 'alt + key' hint in the results."
				toggled={uiState.settings.show_alt_hint}
				on:toggle={onSetShowAltHint}
			/>

			<SliderSetting
				title={`Results Count (${uiState.settings.results_count})`}
				description="The amount of results to show."
				min={2}
				max={9}
				step={1}
				value={uiState.settings.results_count}
				on:slide={onSetResultsCount}
			/>

			<div class="card">
				<div class=" flex items-center gap-4">
					<p class=" text-xl font-medium flex-grow">Blacklist</p>
					<button class=" hover-bg-tertiary p-1 rounded-full" on:click={onOpenAddToBlacklistDialog}>
						<PlusIcon class="w-6 h-6 text-accent " />
					</button>
				</div>
				{#if uiState.blacklist_apps.length === 0}
					<p class=" text-sub-text">
						Blacklist is empty. Click on the add button to add apps to it.
					</p>
				{/if}
				{#each uiState.blacklist_apps as app}
					<div class=" flex gap-4 pt-2 pl-2 pb-2 items-center">
						{#if app.icon !== null}
							<img class=" h-6 w-6" src={convertFileSrc(app.icon)} alt="App Icon" />
						{:else}
							<QuestionIcon class=" h-6 w-6" />
						{/if}
						<p class=" flex-grow text-start">
							{app.title}
						</p>
						<button
							class=" hover-bg-tertiary p-1 rounded-full"
							on:click={() => onOpenRemoveFromBlacklistDialog(app.id)}
						>
							<MinusIcon class="w-6 h-6 text-accent " />
						</button>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</MainFrame>
