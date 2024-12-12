<script lang="ts">
	import { onMount } from 'svelte';
	import {
		colorFilter,
		getColorFilter,
		getIconPath,
		getMainClasses,
		init,
		onOpenSettings,
		onSearchInput,
		onSelectResult,
		state
	} from './search-vm';
	import SearchIcon from '$lib/icons/search.svg?component';
	import SettingsIcon from '$lib/icons/settings.svg?component';
	import CatIcon from '$lib/icons/cat.svg?component';

	$: uiState = $state;
	$: accentColorFilter = $colorFilter;

	onMount(() => {
		init();
	});
</script>

<div>
	{#if !uiState.loading}
		{@html uiState.css}
		<div class={getMainClasses()}>
			<div
				class="bg-background p-3 w-full h-screen text-text flex flex-col max-w-[900px] max-h-[800px] round search-border"
			>
				<div class="p-4 bg-secondary w-full rounded-full flex items-center">
					{#if uiState.settings.show_search_icon}
						<SearchIcon class="w-5 h-5 mr-4" />
					{/if}
					<!-- svelte-ignore a11y-autofocus -->
					<input
						class=" flex-grow bg-secondary outline-none"
						placeholder={uiState.settings.show_placeholder ? 'Search apps, web, extensions' : ''}
						value={uiState.searchText}
						on:input={(e) => onSearchInput(e)}
						autofocus
					/>
					{#if uiState.settings.show_settings_icon}
						<button on:click={() => onOpenSettings()}>
							<SettingsIcon class="w-5 h-5" />
						</button>
					{/if}
				</div>
				<div class="mt-4 flex-grow flex flex-col">
					{#if uiState.results.length === 0}
						<div class="flex flex-col flex-grow justify-center items-center">
							<CatIcon class=" h-20 w-20" />
							<p>No Results</p>
						</div>
					{:else if uiState.resultsType == 'List'}
						{#each uiState.results as result, resultIndex}
							<button
								class={`flex items-center w-full h-[60px] p-2 pl-4 pr-4 hover-bg-tertiary rounded-full ${resultIndex === uiState.selectedIndex ? 'bg-tertiary' : ''}`}
								on:click={() => onSelectResult(resultIndex)}
							>
								{#if result.icon !== null}
									<img
										src={getIconPath(result.icon)}
										alt="Result Icon"
										class={`h-9 w-9 mr-4`}
										style="filter: {result.icon_tint === 'accent'
											? accentColorFilter
											: getColorFilter(result.icon_tint)}"
									/>
								{/if}
								<div class="flex-grow one-line justify-start">
									<p
										class={`one-line text-start ${resultIndex === uiState.selectedIndex ? 'text-accent' : ''}`}
									>
										{result.title}
									</p>
									{#if result.description !== null}
										<p class="text-sm text-start one-line">{result.description}</p>
									{/if}
								</div>
								{#if uiState.settings.show_launch_hint}
									<p class="text-accent">{uiState.settings.launch_key} + {resultIndex + 1}</p>
								{/if}
								{#if uiState.askConfirmation && uiState.selectedIndex === resultIndex}
									<div
										class="bg-accent text-on-accent font-semibold rounded-full h-full pl-2 pr-2 ml-4 flex items-center"
									>
										<p>Confirm</p>
									</div>
								{/if}
							</button>
						{/each}
					{:else}
						<div class="grid grid-cols-4 w-full">
							{#each uiState.results as result, resultIndex}
								<button
									class={`flex flex-col p-4 items-center justify-center one-line h-[160px] hover-bg-tertiary rounded-2xl overflow-hidden ${resultIndex === uiState.selectedIndex ? 'bg-tertiary' : ''}`}
								>
									{#if uiState.askConfirmation && uiState.selectedIndex === resultIndex}
										<div
											class="bg-accent text-on-accent font-semibold h-full w-full pl-2 pr-2 flex items-center justify-center rounded-full"
										>
											<p>Confirm</p>
										</div>
									{:else}
										{#if result.icon !== null}
											<img
												src={getIconPath(result.icon)}
												alt="Result Icon"
												class={`h-14 w-14`}
												style="filter: {getColorFilter(result.icon_tint)}"
											/>
										{/if}
										<p
											class={`one-line w-full mt-4 ${resultIndex === uiState.selectedIndex ? 'text-accent' : ''}`}
										>
											{result.title}
										</p>
									{/if}
								</button>
							{/each}
						</div>
					{/if}
				</div>
				{#if uiState.results.length > 0}
					<div class="flex justify-center">Total Results {uiState.resultsCount}</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
