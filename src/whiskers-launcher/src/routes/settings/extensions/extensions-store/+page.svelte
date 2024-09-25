<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import ChevronRightIcon from '$lib/icons/chevron-right.svg?component';
	import ChevronLeftIcon from '$lib/icons/chevron-left.svg?component';
	import DownloadIcon from '$lib/icons/download.svg?component';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import DialogFrame from '../../../dialog-frame.svelte';
	import {
		init,
		onGoToNextPage,
		onGoToPreviousPage,
		onInstallExtension,
		onSearchInput,
		state
	} from './extensions-store-dialog-vm';

	$: uiState = $state;

	onMount(async () => {
		init();
	});
</script>

<DialogFrame>
	{#if !uiState.loading}
		<div class=" bg-background p-4 h-screen w-full text-text space-y-4 flex flex-col">
			{#if uiState.installingExtension}
				<div class="flex flex-col items-center w-full justify-center h-screen">
					<DownloadIcon class="h-14 mb-8" />
					<p>Installing extension. Do <b>NOT</b> close this window.</p>
				</div>
			{:else}
				<Input
					value={uiState.searchText}
					placeholder="Search extensions"
					on:input={onSearchInput}
				/>

				<div class=" flex-grow overflow-auto space-y-2">
					{#each uiState.displayedStore as storeListing}
						<div class="p-4 border border-tertiary bg-secondary rounded-lg space-y-2 w-full">
							<div class="flex justify-center bg-background rounded-lg">
								<img
									class="h-52 object-contain"
									src={storeListing.preview}
									alt={`Extension, ${storeListing.preview}`}
								/>
							</div>
							<div class=" text-xl font-medium text-start">{storeListing.name}</div>
							<div class="flex items-start">
								<div class="text-start flex flex-col gap-2 flex-grow">
									<div class="flex-grow">
										{storeListing.description}
									</div>
									<div class="flex justify-end space-x-2">
										<SecondaryButton text="Source Code" on:click={() => open(storeListing.repo)} />
										{#if !uiState.installedExtensions.includes(storeListing.id)}
											<SecondaryButton
												text="Install"
												disabled={uiState.installingExtension}
												on:click={() => onInstallExtension(storeListing.repo)}
											/>
										{/if}
									</div>
								</div>
							</div>
						</div>
					{/each}
				</div>

				<div class="flex justify-end space-x-4 items-center">
					<button
						class={`p-2  rounded-md ${uiState.page == 0 ? 'bg-tertiary text-sub-text' : 'bg-accent text-on-accent'}`}
						disabled={uiState.page === 0}
						on:click={onGoToPreviousPage}
					>
						<ChevronLeftIcon class={` h-6 w-6 ${uiState.page === 0}`} />
					</button>
					<div class="p-2 pr-4 pl-4 bg-tertiary rounded-md">{uiState.page + 1}</div>
					<button
						class={`p-2  rounded-md ${(uiState.page + 1) * 12 + 1 <= uiState.filteredStore.length ? 'bg-accent text-on-accent' : 'bg-tertiary text-sub-text'}`}
						disabled={(uiState.page + 1) * 12 > uiState.filteredStore.length}
						on:click={onGoToNextPage}
					>
						<ChevronRightIcon class=" h-6 w-6" />
					</button>
				</div>
			{/if}
		</div>
	{/if}
</DialogFrame>
