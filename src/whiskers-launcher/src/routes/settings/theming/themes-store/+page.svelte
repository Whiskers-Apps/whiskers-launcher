<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import Select from '$lib/components/select.svelte';
	import ChevronRightIcon from '$lib/icons/chevron-right.svg?component';
	import ChevronLeftIcon from '$lib/icons/chevron-left.svg?component';
	import { open } from '@tauri-apps/api/shell';
	import { onMount } from 'svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import DialogFrame from '../../../dialog-frame.svelte';
	import {
		applyTheme as onApplyTheme,
		getSelectValueId,
		getSelectValues,
		init,
		onSearchInput,
		onSelectAccent,
		state,
		goToPreviousPage as onGoToPreviousPage,
		goToNextPage as onGoToNextPage
	} from './themes-store-vm';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<DialogFrame>
	{#if !uiState.loading}
		<div class=" bg-background p-4 h-screen w-full text-text space-y-4 flex flex-col">
			{#if uiState.applyingTheme}
				<div class="flex items-center justify-center">
					<p>Applying theme. Do <b>NOT</b> close this window</p>
				</div>
			{:else}
				<Input value={uiState.searchText} placeholder="Search themes" on:input={onSearchInput} />

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
									{#if storeListing.variants !== null}
										<Select
											values={getSelectValues(storeListing.variants)}
											selectedValue={getSelectValueId(storeListing.id)}
											on:selection={(e) => onSelectAccent(storeListing.id, e.detail.id)}
										/>
									{/if}
									<div class="flex justify-end space-x-2">
										<SecondaryButton text="Source Code" on:click={() => open(storeListing.repo)} />
										<SecondaryButton
											text="Apply"
											disabled={uiState.applyingTheme}
											on:click={() => onApplyTheme(storeListing)}
										/>
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
	{:else}
		<div>Loading...</div>
	{/if}
</DialogFrame>
