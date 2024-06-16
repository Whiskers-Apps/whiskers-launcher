<script lang="ts">
	import Input from '$lib/components/input.svelte';
	import PlusIcon from '$lib/icons/plus.svg?component';
	import SearchIcon from '$lib/icons/search.svg?component';
	import RoundCheckIcon from '$lib/icons/round-check.svg?component';
	import ThreeDotsIcon from '$lib/icons/three-dots.svg?component';
	import { getSettings, writeSettings, type Settings } from '$lib/settings/settings';
	import { listen } from '@tauri-apps/api/event';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { WebviewWindow } from '@tauri-apps/api/window';
	import SecondaryButton from '$lib/components/secondary-button.svelte';
	import { createEventDispatcher } from 'svelte';
	import { WindowSizes } from '../../../utils';

	// ================================
	// Props
	// ================================
	export let settings: Settings;
	let dispatch = createEventDispatcher();

	$: keyword = settings.search_keyword;
	$: searchEngines = settings.search_engines;
	$: default_engine = settings.default_search_engine;

	async function updateKeyword(event: CustomEvent<string>) {
		dispatch('updateSearchKeyword', event.detail.length === 0 ? 's' : event.detail);
	}

	document.addEventListener('click', () => {
		closeMenus();
	});

	async function addSearchEngine() {
		new WebviewWindow('add-search-engine', {
			url: 'dialogs/add-search-engine',
			title: 'Add Search Engine',
			resizable: false,
			width: WindowSizes.SearchEngine.width,
			height: WindowSizes.SearchEngine.height,
			center: true
		});

		const unlisten = await listen('refresh-search-engines', async (_) => {
			dispatch('refreshSearchEngines');
			unlisten();
		});
	}

	function openEngineMenu(id: number) {
		closeMenus();

		let buttonRect = document.getElementById(`menu-button-${id}`)!!.getBoundingClientRect();
		let menu = document.getElementById(`menu-${id}`)!!;
		menu.style.left = `${buttonRect.left - 100}px`;
		menu.style.top = `${buttonRect.top}px`;
		menu.classList.add('flex');
		menu.classList.remove('hidden');
	}

	function closeMenus() {
		searchEngines.forEach((engine) => {
			let element = document.getElementById(`menu-${engine.id}`);
			element?.classList.add('hidden');
			element?.classList.remove('flex');
		});
	}

	async function makeDefault(id: number) {
		default_engine = id;

		settings = await getSettings();
		settings.default_search_engine = id;
		writeSettings(settings);
	}

	async function editSearchEngine(id: number) {
		new WebviewWindow('edit-search-engine', {
			url: `dialogs/edit-search-engine?id=${id}`,
			title: 'Edit Search Engine',
			resizable: false,
			width: WindowSizes.SearchEngine.width,
			height: WindowSizes.SearchEngine.height,
			center: true
		});

		const unlisten = await listen('refresh-search-engines', async (_) => {
			dispatch('refreshSearchEngines');
			unlisten();
		});
	}

	async function deleteSearchEngine(id: number) {
		new WebviewWindow('delete-search-engine', {
			url: `dialogs/delete-search-engine/?id=${id}`,
			title: 'Delete Search Engine',
			resizable: false,
			width: WindowSizes.ConfirmDialog.width,
			height: WindowSizes.ConfirmDialog.height,
			center: true
		});

		const unlisten = await listen('refresh-search-engines', async (_) => {
			dispatch('refreshSearchEngines');
			unlisten();
		});
	}
</script>

<div class=" space-y-4">
	<div class="card">
		<p class=" text-xl font-medium">Search Keyword</p>
		<p class=" text-sub-text mb-2">The keyword used to search with the default engine</p>
		<Input value={keyword} placeholder="Type the default keyword" on:input={updateKeyword} />
	</div>

	<div class="card">
		<div class=" flex items-center gap-4">
			<p class=" text-xl font-medium flex-grow">Search Engines</p>
			<button class=" hover-bg-tertiary p-1 rounded-full" on:click={addSearchEngine}>
				<PlusIcon class="w-6 h-6 text-accent " />
			</button>
		</div>
		{#if searchEngines.length === 0}
			<p>No search engines. Click on the add button to add one.</p>
		{:else}
			{#each searchEngines as engine}
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
					{#if engine.id === default_engine}
						<RoundCheckIcon class=" ml-2 h-5 w-5 text-accent" />
					{/if}
					<div class=" flex-grow"></div>
					<div class=" bg-background p-1 pl-3 pr-3 rounded-md">{engine.keyword}</div>
					<button
						id={`menu-button-${engine.id}`}
						class=" p-2 hover-background rounded-full"
						on:click={(event) => {
							event.stopPropagation();
							openEngineMenu(engine.id);
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
							on:click={() => makeDefault(engine.id)}
						/>

						<SecondaryButton
							text="Edit"
							alignStart={true}
							on:click={() => editSearchEngine(engine.id)}
						/>

						<SecondaryButton
							text="Delete"
							alignStart={true}
							on:click={() => deleteSearchEngine(engine.id)}
						/>
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>
