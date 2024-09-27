<script lang="ts">
	import SettingsIcon from '$lib/icons/settings.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import {
		displayedResults,
		getColorFilter,
		getIconPath,
		init,
		onBlur,
		onOpenSettings,
		onRunAction,
		onSearchInput,
		onSetSelectedIndex,
		resultOffset,
		results,
		searchText,
		selectedIndex,
		showConfirmationBox,
		state
	} from './search-vm';

	$: uiState = $state;
	$: resultsState = $results;
	$: displayedResultsState = $displayedResults;
	$: resultsOffsetState = $resultOffset;
	$: selectedIndexState = $selectedIndex;
	$: searchTextState = $searchText;
	$: showConfirmationBoxState = $showConfirmationBox;

	onMount(async () => {
		init();
	});
</script>

<div>
	{#if !uiState.loading}
		{@html uiState.css}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div
			class={`h-screen overflow-hidden flex justify-center text-text pt-16 wallpaper`}
			on:click={onBlur}
		>
			<div
				class={`search-box-width h-fit search-round overflow-hidden ${uiState.settings.split_results ? '' : 'search-border bg-background'}`}
			>
				<div
					class={`flex p-3 gap-2 search-round ${uiState.settings.split_results ? 'search-border bg-background' : ''}`}
				>
					{#if uiState.settings.show_search_icon}
						<SearchIcon class=" search-icon-size text-accent" />
					{/if}

					<!-- svelte-ignore a11y-autofocus -->
					<input
						class=" w-full outline-none flex-grow bg-transparent search-text"
						type="text"
						placeholder={uiState.settings.show_placeholder ? 'Search apps, extensions, web' : ''}
						value={searchTextState}
						on:input={onSearchInput}
						autofocus
					/>

					{#if uiState.settings.show_settings_icon}
						<button
							class=" hover-bg-tertiary rounded-full"
							on:click={(event) => {
								onOpenSettings(event);
							}}
						>
							<SettingsIcon class=" search-icon-size text-accent" />
						</button>
					{/if}
				</div>

				{#if uiState.settings.split_results}
					<div class="split-divider"></div>
				{/if}

				<div
					class={`overflow-hidden ${uiState.settings.split_results ? ` bg-background search-round  ${displayedResultsState.length > 0 ? 'search-border' : ''}` : ''}`}
				>
					{#each displayedResultsState as result, index}
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<!-- svelte-ignore a11y-mouse-events-have-key-events -->
						<button
							class={`flex w-full items-center overflow-hidden cursor-pointer ${index === selectedIndexState ? 'highlight-result' : ''}`}
							on:mouseover={() => onSetSelectedIndex(index)}
							on:focus={() => onSetSelectedIndex(index)}
							on:click={onRunAction}
						>
							{#if result.result_type === 'Text'}
								<div class="flex-grow flex gap-4 p-3 items-center">
									{#if getIconPath(result.text?.icon) !== null}
										<img
											class={`icon-size ${result.text?.tint}`}
											src={getIconPath(result.text?.icon)}
											alt=""
											style="filter: {getColorFilter(result.text?.tint ?? null)}"
										/>
									{/if}
									<div
										class={`flex-grow one-line text-start result-title ${selectedIndexState === index ? 'text-accent' : ' text-text'}`}
									>
										{result.text?.text}
									</div>
									{#if uiState.settings.show_alt_hint}
										<div
											class={`result-alt ${selectedIndexState === index ? 'text-accent' : ' text-sub-text'}`}
										>
											Alt + {index + 1}
										</div>
									{/if}
								</div>
							{/if}
							{#if result.result_type === 'TitleAndDescription'}
								<div class="flex-grow flex gap-4 p-3 items-center">
									{#if result.title_and_description?.icon !== null}
										<img
											class={`icon-size ${result.title_and_description?.tint}`}
											src={convertFileSrc(result.title_and_description?.icon ?? '')}
											alt=""
											style="filter: {getColorFilter(result.title_and_description?.tint ?? null)}"
										/>
									{/if}
									<div
										class={`flex-grow one-line text-start result-title flex flex-col ${selectedIndexState === index ? 'text-accent' : ' text-text'}`}
									>
										<p class="result-title one-line">{result.title_and_description?.title}</p>
										<p class="result-description one-line">
											{result.title_and_description?.description}
										</p>
									</div>
									{#if uiState.settings.show_alt_hint}
										<div
											class={`result-alt ${selectedIndexState === index ? 'text-accent' : ' text-sub-text'}`}
										>
											Alt + {index + 1}
										</div>
									{/if}
								</div>
							{/if}
							{#if result.result_type === 'Divider'}
								<div class="p-2 pt-4 pb-4 w-full">
									<div
										class={`result-divider rounded-full ${selectedIndexState === index ? 'bg-accent' : 'bg-tertiary'}`}
									></div>
								</div>
							{/if}
							{#if showConfirmationBoxState && selectedIndexState === index}
								<div
									class="flex bg-accent result-confirm rounded-l-md text-on-accent w-fit items-center p-2"
								>
									Confirm Action
								</div>
							{/if}
						</button>
					{/each}
				</div>
			</div>
		</div>
	{/if}
</div>
