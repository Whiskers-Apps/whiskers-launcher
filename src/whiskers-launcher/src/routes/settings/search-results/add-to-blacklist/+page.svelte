<script lang="ts">
	import PrimaryButton from '$lib/components/primary-button.svelte';
	import QuestionIcon from '$lib/icons/question.svg?component';
	import { onMount } from 'svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import DialogFrame from '../../../dialog-frame.svelte';
	import {
		getHighlightClass,
		init,
		onAddAppsToBlacklist,
		onAppClick as onToggleAppSelection,
		state
	} from './add-to-blacklist-dialog-vm';

	$: uiState = $state;

	onMount(async () => {
		init();
	});
</script>

<DialogFrame>
	{#if !uiState.loading}
		<div class="flex flex-col p-4 h-screen">
			<div class=" flex-grow overflow-auto grid grid-cols-3 gap-4 ">
				{#each uiState.whiteListedApps as app}
					<button
						class={`flex flex-col items-center p-2 gap-4 w-full hover-bg-tertiary rounded-2xl ${getHighlightClass(app.id)}`}
						on:click={() => onToggleAppSelection(app)}
					>
						{#if app.icon !== null}
							<img class=" h-14 w-14" src={convertFileSrc(app.icon)} alt="App Icon" />
						{:else}
							<QuestionIcon class=" h-6 w-6" />
						{/if}
						<p class=" flex-grow text-center">
							{app.title}
						</p>
					</button>
				{/each}
			</div>
			<div class=" flex justify-end mt-4">
				<PrimaryButton
					text="Add"
					disabled={uiState.selectedApps.length === 0}
					on:click={onAddAppsToBlacklist}
				/>
			</div>
		</div>
	{/if}
</DialogFrame>
