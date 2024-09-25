<script>
	import Input from '$lib/components/input.svelte';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import MainFrame from '../main-frame.svelte';
	import {
		init,
		onAddSearchEngine,
		onDeleteSearchEngine,
		onEditSearchEngine,
		onOpenSearchEngineMenu,
		onSetDefaultSearchEngine,
		onSetSearchKeyword,
		state
	} from './search-engines-vm';
	import PlusIcon from '$lib/icons/plus.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import RoundCheckIcon from '$lib/icons/round-check.svg?component';
	import ThreeDotsIcon from '$lib/icons/three-dots.svg?component';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	$: uiState = $state;

	onMount(() => {
		init();
	});
</script>

<MainFrame>
	{#if !uiState.loading}
		<div class="space-y-8">
			<div>
				<p class=" text-xl font-medium">Search Keyword</p>
				<p class=" text-sub-text mb-2">The keyword used to search with the default engine</p>
				<Input
					value={uiState.settings.search_keyword}
					placeholder="Type the default keyword"
					on:input={onSetSearchKeyword}
				/>
			</div>

			<div class="">
				<div class=" flex items-center gap-4">
					<p class=" text-xl font-medium flex-grow">Search Engines</p>
					<button class=" hover-bg-tertiary p-1 rounded-full" on:click={onAddSearchEngine}>
						<PlusIcon class="w-6 h-6 text-accent " />
					</button>
				</div>
				{#if uiState.settings.search_engines.length === 0}
					<p>No search engines. Click on the add button to add one.</p>
				{:else}
					{#each uiState.settings.search_engines as engine}
						<div class=" flex gap-2 pt-2 pl-2 pb-2 items-center">
							{#if engine.icon_path !== null}
								<img
									class={` h-6 w-6 ${engine.tint_icon ? 'accent-filter' : ''}`}
									src={convertFileSrc(engine.icon_path)}
									alt="Search Engine Icon"
								/>
							{:else}
								<SearchIcon class="h-6 w-6 text-accent" />
							{/if}
							{engine.name}
							{#if engine.id === uiState.settings.default_search_engine}
								<RoundCheckIcon class=" ml-2 h-5 w-5 text-accent" />
							{/if}
							<div class=" flex-grow"></div>
							<div class=" bg-background p-1 pl-3 pr-3 rounded-md">{engine.keyword}</div>
							<button
								id={`menu-button-${engine.id}`}
								class=" p-2 hover-background rounded-full"
								on:click={(event) => {
									event.stopPropagation();
									onOpenSearchEngineMenu(engine.id);
								}}
							>
								<ThreeDotsIcon class=" h-5 w-5 text-accent" />
							</button>
							<div
								id={`menu-${engine.id}`}
								class=" bg-secondary border-tertiary hidden flex-col absolute z-20 rounded-md"
							>
								<SecondaryButton
									text="Make Default"
									alignStart={true}
									on:click={() => onSetDefaultSearchEngine(engine.id)}
								/>

								<SecondaryButton
									text="Edit"
									alignStart={true}
									on:click={() => onEditSearchEngine(engine.id)}
								/>

								<SecondaryButton
									text="Delete"
									alignStart={true}
									on:click={() => onDeleteSearchEngine(engine.id)}
								/>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</MainFrame>
