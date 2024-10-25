<script>
	import { onMount } from 'svelte';
	import MainFrame from '../main-frame.svelte';
	import {
	getLaunchKeys,
		init,
		onOpenAddToBlacklistDialog,
		onOpenRemoveFromBlacklistDialog,
		onSetLaunchKey,
		onSetShowAppsAsGrid,
		onSetShowLaunchHint,
		state
	} from './search-results-vm';
	import ToggleSetting from '$lib/components/toggle-setting.svelte';
	import PlusIcon from '$lib/icons/plus.svg?component';
	import MinusIcon from '$lib/icons/minus.svg?component';
	import QuestionIcon from '$lib/icons/question.svg?component';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import SelectSetting from '$lib/components/select-setting.svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">

			<SelectSetting
				values={getLaunchKeys()}
				title="Launch Key"
				description="Shortcut key"
				selectedValue={uiState.settings.launch_key}
				on:selection={onSetLaunchKey}
			/>
			
			<ToggleSetting
				title="Show Shortcut Hint"
				description="When enabled, it shows the 'alt/ctrl + key' hint in the results."
				toggled={uiState.settings.show_launch_hint}
				on:toggle={onSetShowLaunchHint}
			/>

			<ToggleSetting
				title="Grid"
				description="Show apps as a grid"
				toggled={uiState.settings.show_apps_as_grid}
				on:toggle={onSetShowAppsAsGrid}
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
